mod proto;
mod utils;
mod files_service;

use anyhow::{Context, Result};
use sqlx::{postgres::PgPoolOptions, PgPool};
use tonic::transport::Server;
use utils::check_env;

use crate::proto::utils_service_server::UtilsServiceServer;


#[derive(Debug)]
pub struct MyService {
    pool: PgPool,
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("Starting server...");

    // Database
    let database_url = check_env("DATABASE_URL")?;
    let pool = PgPoolOptions::new()
        .max_connections(20)
        .connect(&database_url)
        .await
        .with_context(|| format!("Failed to connect to database: {}", database_url))?;

    // Migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .context("Failed to run migrations")?;
    println!("Migrations ran successfully");

    let port = check_env("PORT")?;
    let addr = ("0.0.0.0:".to_owned() + &port).parse()?;

    let service = MyService {
        pool,
    };

    println!("Server started on port: {}", port);
    Server::builder()
        .add_service(UtilsServiceServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}
