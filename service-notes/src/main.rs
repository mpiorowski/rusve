mod models;
mod notes_db;
mod notes_service;
mod proto;
mod schema;

use anyhow::{Context, Result};
use diesel_async::{pooled_connection::deadpool::Pool, AsyncMysqlConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations};
use proto::notes_service_server::NotesServiceServer;
use rusve_notes::{establish_connection, establish_connection_sync, establish_connection_tls};
use tonic::transport::Server;

use diesel_migrations::MigrationHarness;

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

    let pool = establish_connection(&database_url)?;
    let conn = pool
        .get()
        .await
        .context("Error getting connection from pool")?;
    drop(conn);

    let addr = ("[::]:".to_owned() + &port).parse()?;
    println!("Server started on port: {}", port);

    let server = MyService { pool, mysql_pool };
    let svc = NotesServiceServer::new(server);
    Server::builder().add_service(svc).serve(addr).await?;

    Ok(())
}
