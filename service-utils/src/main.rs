mod email_service;
mod files_db;
mod files_service;
mod files_utils;
mod migrations;
mod proto;

use crate::proto::utils_service_server::UtilsServiceServer;
use anyhow::{Context, Result};

pub struct MyService {
    pool: deadpool_postgres::Pool,
}

#[tokio::main]
async fn main() -> Result<()> {
    email_service::subscribe_to_emails()
        .await
        .context("Failed to subscribe to emails")?;

    // Initialize tracing
    let filter = std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_owned());
    tracing_subscriber::fmt().with_env_filter(filter).init();

    // Connect to database
    let pool = rusve_users::connect_to_db().context("Failed to connect to database")?;
    tracing::info!("Connected to database");

    // Run migrations
    migrations::run_migrations(&pool)
        .await
        .context("Failed to run migrations")?;
    tracing::info!("Migrations complete");

    // Run gRPC server
    let port = std::env::var("PORT").context("PORT not set")?;
    let addr = format!("[::]:{}", port).parse()?;
    tracing::info!("gRPC server started on port: {:?}", port);
    let server = MyService { pool };
    let svc = UtilsServiceServer::new(server);
    tonic::transport::Server::builder()
        .add_service(svc)
        .serve(addr)
        .await?;

    Ok(())
}
