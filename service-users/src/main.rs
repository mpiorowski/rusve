mod proto;
mod users_service;

use rusve_users::establish_connection;

use crate::proto::users_service_server::UsersServiceServer;
use anyhow::{Context, Result};
use std::str::FromStr;
use tonic::transport::Server;

#[derive(Debug)]
pub struct MyService {
    pool: deadpool_postgres::Pool,
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("Starting server...");

    let connection = &mut establish_connection();

    let database_url = std::env::var("DATABASE_URL").context("DATABASE_URL not set")?;
    let port = std::env::var("PORT").context("PORT not set")?;

    // Database connection pool
    let pg_config = tokio_postgres::Config::from_str(&database_url)?;
    let manager = deadpool_postgres::Manager::from_config(
        pg_config,
        postgres_native_tls::MakeTlsConnector::new(native_tls::TlsConnector::new()?),
        deadpool_postgres::ManagerConfig {
            recycling_method: deadpool_postgres::RecyclingMethod::Fast,
        },
    );
    let pool = deadpool_postgres::Pool::builder(manager)
        .max_size(20)
        .build()
        .context("Failed to create database pool")?;
    println!("Connected to database");

    let addr = ("[::]:".to_owned() + &port).parse()?;
    println!("Server started on port: {}", port);

    let server = MyService { pool };
    let svc = UsersServiceServer::new(server);
    Server::builder().add_service(svc).serve(addr).await?;

    Ok(())
}
