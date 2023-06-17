mod models;
mod proto;
mod schema;
mod users_service;

use crate::proto::users_service_server::UsersServiceServer;
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

    // Run migrations - diesel_async have an open PR to support this
    let mut conn = establish_connection_sync(&database_url)?;
    conn.run_pending_migrations(MIGRATIONS)
        .map_err(|e| anyhow::anyhow!("Error running migrations: {:?}", e.to_string()))?;
    println!("Migrations run successfully");

    // Create a connection pool without tls
    let pool = establish_connection_tls(&database_url)?;

    let addr = ("[::]:".to_owned() + &port).parse()?;
    println!("Server started on port: {}", port);

    let cert = tokio::fs::read("/app/src/tls/server.pem").await?;
    let key = tokio::fs::read("/app/src/tls/server.key").await?;
    let server_identity = tonic::transport::Identity::from_pem(cert, key);

    // let client_ca_cert = tokio::fs::read("/app/src/tls/ca.pem").await?;
    // let client_ca_cert = tonic::transport::Certificate::from_pem(client_ca_cert);
    let tls = tonic::transport::ServerTlsConfig::new()
        .identity(server_identity);
        // .client_ca_root(client_ca_cert);

    let server = MyService { pool };
    let svc = UsersServiceServer::new(server);
    Server::builder()
        .tls_config(tls)?
        .add_service(svc)
        .serve(addr)
        .await?;

    Ok(())
}
