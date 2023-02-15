mod notes_service;
mod proto;
mod utils;

use anyhow::{Context, Result};
use proto::notes_service_server::NotesServiceServer;
use sqlx::{postgres::PgPoolOptions, PgPool};
use tonic::{transport::Server, Status};
use utils::check_env;

use crate::proto::users_service_client::UsersServiceClient;

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
    users_conn: UsersServiceClient<tonic::transport::Channel>,
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
    let addr = ("0.0.0.0:".to_owned() + &port).parse()?;

    let uri_users = check_env("URI_USERS")?;
    let users_conn = UsersServiceClient::connect(uri_users)
        .await
        .context("Failed to connect to users service")?;

    let service = MyService { pool, users_conn };

    println!("Server started on port: {}", port);
    Server::builder()
        .add_service(NotesServiceServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}
