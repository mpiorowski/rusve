mod models;
mod proto;
mod schema;
mod users_service;

use crate::proto::users_service_server::UsersServiceServer;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use anyhow::{Context, Result};
use deadpool::managed::Pool;
use diesel_async::{pooled_connection::AsyncDieselConnectionManager, AsyncPgConnection};
use rusve_users::{establish_connection, establish_connection_sync};
use tonic::transport::Server;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub struct MyService {
    pool: Pool<AsyncDieselConnectionManager<AsyncPgConnection>>,
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("Starting server...");

    let port = std::env::var("PORT").context("PORT not set")?;
    let database_url = std::env::var("DATABASE_URL").context("DATABASE_URL not set")?;

    // Run migrations - diesel_async have an open PR to support this
    let mut conn = establish_connection_sync(&database_url)?;
    conn.run_pending_migrations(MIGRATIONS)
        .map_err(|e| anyhow::anyhow!("Error running migrations: {:?}", e.to_string()))?;
    println!("Migrations run successfully");

    // Create a connection pool without tls
    let pool = establish_connection(&database_url)?;

    let addr = ("[::]:".to_owned() + &port).parse()?;
    println!("Server started on port: {}", port);

    let server = MyService { pool };
    let svc = UsersServiceServer::new(server);
    Server::builder().add_service(svc).serve(addr).await?;

    Ok(())
}
