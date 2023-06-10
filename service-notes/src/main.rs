mod models;
mod proto;
mod schema;
mod notes_service;
mod notes_db;

use anyhow::{Context, Result};
use diesel_async::{pooled_connection::deadpool::Pool, AsyncMysqlConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use proto::notes_service_server::NotesServiceServer;
use rusve_notes::{establish_connection, establish_connection_sync};
use tonic::transport::Server;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub struct MyService {
    pool: Pool<AsyncMysqlConnection>,
    mysql_pool: mysql_async::Pool,
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("Starting server...");

    let port = std::env::var("PORT").context("PORT not set")?;
    let database_url = std::env::var("DATABASE_URL").context("DATABASE_URL not set")?;

    let opts = mysql_async::Opts::from_url(&database_url)?;
    let mysql_pool = mysql_async::Pool::new(opts);

    // Run migrations - diesel_async have an open PR to support this
    // let mut conn = establish_connection_sync(&database_url)?;
    // conn.run_pending_migrations(MIGRATIONS)
    //     .map_err(|e| anyhow::anyhow!("Error running migrations: {:?}", e.to_string()))?;
    // println!("Migrations run successfully");

    // // Create a connection pool without tls
    // let pool = establish_connection(&database_url)?;

    let addr = ("[::]:".to_owned() + &port).parse()?;
    println!("Server started on port: {}", port);

    let server = MyService { pool, mysql_pool };
    let svc = NotesServiceServer::new(server);
    Server::builder().add_service(svc).serve(addr).await?;

    Ok(())
}
