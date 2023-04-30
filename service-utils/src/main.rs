mod proto;
mod files_service;
mod emails_service;

use crate::{proto::utils_service_server::UtilsServiceServer, emails_service::subscribe_to_email};
use anyhow::{Context, Result};
use sqlx::{postgres::PgPoolOptions, PgPool};
use tonic::transport::Server;

#[derive(Debug)]
pub struct MyService {
    pool: PgPool,
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("Starting server...");

    // check all env at start
    let envs = vec!["ENV", "PORT", "DATABASE_URL", "BUCKET"];
    for env in envs {
        if std::env::var(env).is_err() || std::env::var(env).unwrap().is_empty() {
            println!("Missing env: {}", env);
            std::process::exit(1);
        }
    }

    // Database
    let database_url = std::env::var("DATABASE_URL")?;
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

    // Subscribe to email
    subscribe_to_email().await?;

    let port = std::env::var("PORT")?;
    let addr = ("[::]:".to_owned() + &port).parse()?;
    println!("Server started on port: {}", port);

    let server = MyService { pool };
    let svc = UtilsServiceServer::new(server);
    Server::builder().add_service(svc).serve(addr).await?;

    Ok(())
}
