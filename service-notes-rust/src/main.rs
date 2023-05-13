mod notes_service;
mod proto;
mod utils;

use std::str::FromStr;

use anyhow::{Context, Result};
use deadpool_postgres::{Manager, ManagerConfig, Pool};
use proto::notes_service_server::NotesServiceServer;
use sqlx::postgres::PgPoolOptions;
use tonic::transport::Server;

#[derive(Debug)]
pub struct MyService {
    pool: Pool,
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("Starting server...");

    let database_url = std::env::var("DATABASE_URL").context("DATABASE_URL not set")?;
    let port = std::env::var("PORT").context("PORT not set")?;

    // Database
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

    // Database connection pool
    let pg_config = tokio_postgres::Config::from_str(&database_url)?;
    let mgr = Manager::from_config(pg_config, tokio_postgres::NoTls, ManagerConfig::default());
    let pool = Pool::builder(mgr)
        .max_size(20)
        .build()
        .context("Failed to create database pool")?;
    println!("Connected to database");

    let addr = ("[::]:".to_owned() + &port).parse()?;
    println!("Server started on port: {}", port);

    let server = MyService { pool };
    let svc = NotesServiceServer::new(server);
    Server::builder().add_service(svc).serve(addr).await?;

    Ok(())
}
