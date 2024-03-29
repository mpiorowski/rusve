mod proto;
mod migrations;
mod grpc;
mod profile_service;
mod profile_validation;
mod profile_db;
mod stripe_service;
mod stripe_db;
mod token_db;
mod user_service;
mod user_db;

use crate::proto::users_service_server::UsersServiceServer;
use anyhow::{Context, Result};

pub struct MyService {
    env: service_users::Env,
    pool: deadpool_postgres::Pool,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initalize environment variables
    let env: service_users::Env = service_users::init_envs()?;

    // Initialize tracing
    let filter = &env.rust_log;
    tracing_subscriber::fmt().with_env_filter(filter).init();

    // Connect to database
    let pool = service_users::connect_to_db(&env).context("Failed to connect to database")?;
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
    let svc = UsersServiceServer::new(server);
    tonic::transport::Server::builder()
        .add_service(svc)
        .serve(addr)
        .await
        .context("Failed to run gRPC server")?;
    Ok(())
}
