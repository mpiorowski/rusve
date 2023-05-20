mod proto;
mod users_service;

use std::str::FromStr;

use crate::proto::users_service_server::UsersServiceServer;
use anyhow::{Context, Result};
use tonic::transport::Server;

#[derive(Debug)]
pub struct MyService {
    pool: deadpool_postgres::Pool,
}

mod embedded {
    use refinery::embed_migrations;
    embed_migrations!("./migrations");
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("Starting server...");

    let database_url = std::env::var("DATABASE_URL").context("DATABASE_URL not set")?;
    let port = std::env::var("PORT").context("PORT not set")?;

    // Leaving sqlx pools for future reference, when the sqlx performance will be fixed
    // Sqlx database
    // Migrations
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(1)
        .connect(&database_url)
        .await
        .with_context(|| format!("Failed to connect to database: {}", database_url))?;
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .context("Failed to run migrations")?;
    println!("Migrations ran successfully");
    pool.close().await;

    // Database connection pool
    let pg_config = tokio_postgres::Config::from_str(&database_url)?;
    let manager = deadpool_postgres::Manager::from_config(
        pg_config,
        tokio_postgres::NoTls,
        deadpool_postgres::ManagerConfig {
            recycling_method: deadpool_postgres::RecyclingMethod::Fast,
        },
    );
    let pool = deadpool_postgres::Pool::builder(manager)
        .max_size(20)
        .build()
        .context("Failed to create database pool")?;
    println!("Connected to database");

    let addr = ("[::]:".to_owned() + &port).parse()?;
    println!("Server started on port: {}", port);

    let server = MyService { pool };
    let svc = UsersServiceServer::new(server);
    Server::builder().add_service(svc).serve(addr).await?;

    Ok(())
}
