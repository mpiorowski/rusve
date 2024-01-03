mod util_service;
mod file_utils;
mod file_db;
mod email_utils;
mod email_db;
mod migrations;
mod proto;

use crate::proto::utils_service_server::UtilsServiceServer;
use anyhow::{Context, Result};

pub struct MyService {
    env: rusve_users::Env,
    pool: deadpool_postgres::Pool,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initalize environment variables
    let env: rusve_users::Env = rusve_users::init_envs()?;

    // Initialize tracing
    let filter = &env.rust_log;
    tracing_subscriber::fmt().with_env_filter(filter).init();

    // Connect to database
    let pool = rusve_users::connect_to_db(&env).context("Failed to connect to database")?;
    tracing::info!("Connected to database");

    // Run migrations
    migrations::run_migrations(&pool)
        .await
        .context("Failed to run migrations")?;
    tracing::info!("Migrations complete");

    // Run gRPC server
    let addr = format!("[::]:{}", env.port).parse()?;
    tracing::info!("gRPC server started on port: {:?}", env.port);
    let server = MyService { env, pool };
    let svc = UtilsServiceServer::new(server);
    tonic::transport::Server::builder()
        .add_service(svc)
        .serve(addr)
        .await
        .context("Failed to run gRPC server")?;

    Ok(())
}
