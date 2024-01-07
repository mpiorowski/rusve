mod migrations;
mod note_db;
mod note_service;
mod note_validation;
mod proto;

use crate::proto::notes_service_server::NotesServiceServer;
use anyhow::{Context, Result};

pub struct MyService {
    pool: deadpool_postgres::Pool,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initalize environment variables
    let env: rusve_notes::Env = rusve_notes::init_envs()?;

    // Initialize tracing
    let filter = &env.rust_log;
    tracing_subscriber::fmt().with_env_filter(filter).init();

    // Connect to database
    let pool = rusve_notes::connect_to_db(&env).context("Failed to connect to database")?;
    tracing::info!("Connected to database");

    // Run migrations
    migrations::run_migrations(&pool)
        .await
        .context("Failed to run migrations")?;
    tracing::info!("Migrations complete");

    // Run gRPC server
    let addr = format!("[::]:{}", env.port).parse()?;
    tracing::info!("gRPC server started on port: {:?}", env.port);
    let server = MyService { pool };
    let svc = NotesServiceServer::new(server);
    tonic::transport::Server::builder()
        .add_service(svc)
        .serve(addr)
        .await
        .context("Failed to start gRPC server")?;
    Ok(())
}
