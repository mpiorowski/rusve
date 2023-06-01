mod emails;
mod files;
mod proto;
mod models;
mod schema;
mod db;

use anyhow::Context;
use anyhow::Result;
use diesel_async::{pooled_connection::deadpool::Pool, AsyncPgConnection};
use diesel_migrations::embed_migrations;
use diesel_migrations::EmbeddedMigrations;
use diesel_migrations::MigrationHarness;
use rusve_utils::establish_connection;
use rusve_utils::establish_connection_sync;
use tonic::transport::Server;

use crate::emails::subscribe_to_email;
use crate::proto::utils_service_server::UtilsServiceServer;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub struct MyService {
    pool: Pool<AsyncPgConnection>,
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("Starting server...");

    // check all env at start
    let envs = vec!["ENV", "PORT", "DATABASE_URL", "BUCKET", "SENDGRID_API_KEY"];
    for env in envs {
        if std::env::var(env).is_err() || std::env::var(env).unwrap().is_empty() {
            println!("Missing env: {}", env);
            std::process::exit(1);
        }
    }

    println!("Starting server...");

    subscribe_to_email().await?;

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
    let svc = UtilsServiceServer::new(server);
    Server::builder().add_service(svc).serve(addr).await?;

    Ok(())
}
