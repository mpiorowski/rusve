mod proto;
mod auth_db;
mod auth_oauth;
mod auth_service;
mod migrations;

use anyhow::Context;
use anyhow::Result;
use axum::http::StatusCode;
use axum::Json;
use axum::{routing::get, Router};
use http::header::{AUTHORIZATION, CONTENT_TYPE};
use http::Method;
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};

struct AppState {
    env: rusve_auth::Env,
    pool: deadpool_postgres::Pool,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initalize environment variables
    let env: rusve_auth::Env = rusve_auth::init_envs()?;

    // Initialize tracing
    let filter = &env.rust_log;
    tracing_subscriber::fmt().with_env_filter(filter).init();

    // Connect to database
    let pool = rusve_auth::connect_to_db(&env).context("Failed to connect to database")?;
    tracing::info!("Connected to database");

    // Run migrations
    migrations::run_migrations(&pool)
        .await
        .context("Failed to run migrations")?;
    tracing::info!("Migrations complete");

    // Create shared state
    let shared_state = Arc::new(AppState {
        pool,
        env: env.clone(),
    });

    // TODO - Add origin for production
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::DELETE])
        .allow_headers([AUTHORIZATION, CONTENT_TYPE])
        .allow_origin(Any);

    let app = Router::new()
        .route("/", get(root))
        .route("/oauth-login/:provider", get(auth_service::oauth_login))
        .route(
            "/oauth-callback/:provider",
            get(auth_service::oauth_callback),
        )
        .with_state(shared_state.clone())
        .layer(ServiceBuilder::new().layer(cors));

    // Run HTTP server
    let addr = format!("[::]:{}", env.port);
    tracing::info!("HTTP server started on port: {:?}", env.port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app)
        .await
        .context("Failed to run HTTP server")?;
    Ok(())
}

/**
 * Ping the database to check if it's up
 */
async fn root() -> Result<(StatusCode, Json<String>), StatusCode> {
    tracing::info!("Ping");
    Ok((StatusCode::OK, Json("Hello, World!".to_string())))
}
