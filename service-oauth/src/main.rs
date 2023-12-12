mod oauth_service;

use anyhow::Context;
use anyhow::Result;
use axum::Extension;
use axum::http::StatusCode;
use axum::Json;
use axum::{routing::get, Router};
use http::header::{AUTHORIZATION, CONTENT_TYPE};
use http::Method;
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};

use crate::oauth_service::build_oauth_client;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt().init();

    let port = std::env::var("PORT").context("PORT not set")?;

    // TODO - Add origin for production
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::DELETE])
        .allow_headers([AUTHORIZATION, CONTENT_TYPE])
        .allow_origin(Any);

    let app = Router::new()
        .route("/", get(root))
        .route("/api/auth/google", get(oauth_service::google_auth))
        .layer(ServiceBuilder::new().layer(cors))
        .layer(Extension(build_oauth_client(
            std::env::var("GOOGLE_CLIENT_ID").context("GOOGLE_CLIENT_ID not set")?,
            std::env::var("GOOGLE_CLIENT_SECRET").context("GOOGLE_CLIENT_SECRET not set")?,
        )));

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
