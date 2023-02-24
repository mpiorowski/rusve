mod notes_service;
mod proto;
mod utils;

use anyhow::{Context, Result};
use proto::notes_service_server::NotesServiceServer;
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::sync::Arc;
use time::OffsetDateTime;
use tokio::sync::Mutex;
use tonic::transport::Server;
use utils::check_env;

use crate::proto::users_service_client::UsersServiceClient;

#[derive(Debug)]
pub struct CachedToken {
    token: String,
    expires: OffsetDateTime,
}

#[derive(Debug)]
pub struct MyService {
    pool: PgPool,
    users_conn: UsersServiceClient<tonic::transport::Channel>,
    cached_token: Arc<Mutex<CachedToken>>,
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

    // Users service
    let uri_users = check_env("URI_USERS")?;
    let users_conn = UsersServiceClient::connect(uri_users)
        .await
        .context("Failed to connect to users service")?;

    let service = MyService {
        pool,
        users_conn,
        cached_token: Arc::new(Mutex::new(CachedToken {
            token: "".to_string(),
            expires: OffsetDateTime::now_utc(),
        })),
    };

    println!("Server started on port: {}", port);
    Server::builder()
        .add_service(NotesServiceServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}
