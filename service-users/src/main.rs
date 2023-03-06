mod proto;
mod users_service;
mod utils;

use crate::proto::users_service_server::UsersServiceServer;
use anyhow::{Context, Result};
use sqlx::{postgres::PgPoolOptions, PgPool};
use tonic::{transport::Server, Status};
use utils::check_env;

trait IntoStatus {
    fn into_status(self) -> Status;
}

impl IntoStatus for sqlx::Error {
    fn into_status(self: sqlx::Error) -> Status {
        Status::internal(self.to_string())
    }
}

#[derive(Debug)]
pub struct MyService {
    pool: PgPool,
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("Starting server...");

    let database_url = check_env("DATABASE_URL")?;
    let pool = PgPoolOptions::new()
        .max_connections(20)
        .connect(&database_url)
        .await
        .with_context(|| format!("Failed to connect to database: {}", database_url))?;

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .context("Failed to run migrations")?;

    println!("Migrations ran successfully");

    let port = check_env("PORT")?;
    let addr = ("[::]:".to_owned() + &port)
        .parse()
        .context("Failed to parse address")?;

    let service = MyService { pool };
    println!("Server started on port: {}", port);
    Server::builder()
        .add_service(UsersServiceServer::new(service))
        .serve(addr)
        .await
        .context("Failed to start server")?;

    Ok(())
}
