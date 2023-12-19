mod migrations;
mod oauth_db;
mod oauth_service;

use anyhow::Context;
use anyhow::Result;
use axum::http::StatusCode;
use axum::Extension;
use axum::Json;
use axum::{routing::get, Router};
use http::header::{AUTHORIZATION, CONTENT_TYPE};
use http::Method;
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};

struct AppState {
    db_pool: deadpool_postgres::Pool,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    let filter = std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_owned());
    tracing_subscriber::fmt().with_env_filter(filter).init();

    // Connect to database
    let db_pool = rusve_oauth::connect_to_db().context("Failed to connect to database")?;
    let shared_state = Arc::new(AppState { db_pool });
    tracing::info!("Connected to database");

    // Run migrations
    migrations::run_migrations(&shared_state.db_pool)
        .await
        .context("Failed to run migrations")?;
    tracing::info!("Migrations complete");

    // TODO - Add origin for production
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::DELETE])
        .allow_headers([AUTHORIZATION, CONTENT_TYPE])
        .allow_origin(Any);

    let app = Router::new()
        .route("/", get(root))
        .route("/oauth-login/google", get(oauth_service::oauth_login))
        .route("/oauth-callback/google", get(oauth_service::oauth_callback))
        .with_state(shared_state)
        .layer(ServiceBuilder::new().layer(cors))
        .layer(Extension(oauth_service::build_oauth_client(
            std::env::var("GOOGLE_CLIENT_ID").context("GOOGLE_CLIENT_ID not set")?,
            std::env::var("GOOGLE_CLIENT_SECRET").context("GOOGLE_CLIENT_SECRET not set")?,
        )));

    let port = std::env::var("PORT").context("PORT not set")?;
    let addr = format!("0.0.0.0:{}", port);
    tracing::info!("Api server started on {}", addr);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app)
        .await
        .context("Failed to start server")?;
    Ok(())
}

/**
 * Ping the database to check if it's up
 */
async fn root() -> Result<(StatusCode, Json<String>), StatusCode> {
    tracing::info!("Ping");
    Ok((StatusCode::OK, Json("Hello, World!".to_string())))
}