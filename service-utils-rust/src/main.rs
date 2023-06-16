mod email_service;
mod files_db;
mod files_service;
mod models;
mod proto;
mod schema;

use crate::proto::utils_service_server::UtilsServiceServer;
use anyhow::{Context, Result};
use diesel_async::{pooled_connection::deadpool::Pool, AsyncPgConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use rusve_users::{establish_connection_sync, establish_connection_tls};
use tonic::transport::Server;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub struct MyService {
    pool: Pool<AsyncPgConnection>,
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("Starting server...");

    let port = std::env::var("PORT").context("PORT not set")?;
    let database_url = std::env::var("DATABASE_URL").context("DATABASE_URL not set")?;

    // Subscribe to emails
    email_service::subscribe_to_emails().await?;
    println!("Subscribed to emails");

    // Run migrations - diesel_async have an open PR to support this
    let mut conn = establish_connection_sync(&database_url)?;
    conn.run_pending_migrations(MIGRATIONS)
        .map_err(|e| anyhow::anyhow!("Error running migrations: {:?}", e.to_string()))?;
    println!("Migrations run successfully");
    drop(conn);

    // Create a connection pool without tls
    let pool = establish_connection_tls(&database_url)?;

    let addr = ("[::]:".to_owned() + &port).parse()?;
    println!("Server started on port: {}", port);

    let server = MyService { pool };
    let svc = UtilsServiceServer::new(server);
    Server::builder().add_service(svc).serve(addr).await?;

    Ok(())
}
