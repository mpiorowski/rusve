mod proto;
mod utils;
mod utils_service;

use crate::proto::utils_service_server::UtilsServiceServer;
use anyhow::{Context, Result};
use sqlx::{postgres::PgPoolOptions, PgPool};
use tonic::transport::Server;
use utils::check_env;

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
    println!("Connected to database");

    // Migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .context("Failed to run migrations")?;
    println!("Migrations ran successfully");

    let port = check_env("PORT")?;
    let addr = ("[::]:".to_owned() + &port).parse()?;
    println!("Server started on port: {}", port);

    let server = MyService { pool };
    let svc = UtilsServiceServer::new(server);
    Server::builder().add_service(svc).serve(addr).await?;

    Ok(())
}
