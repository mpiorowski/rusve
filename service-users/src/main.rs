mod proto;
mod users_service;

use crate::proto::users_service_server::UsersServiceServer;
use anyhow::{Context, Result};
use tonic::transport::Server;

pub struct MyService {
    pool: deadpool_diesel::postgres::Pool,
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("Starting server...");

    let database_url = std::env::var("DATABASE_URL").context("DATABASE_URL not set")?;
    let port = std::env::var("PORT").context("PORT not set")?;

    let manager = deadpool_diesel::postgres::Manager::new(
        database_url,
        deadpool_diesel::postgres::Runtime::Tokio1,
    );
    let pool = deadpool_diesel::postgres::Pool::builder(manager)
        .max_size(10)
        .build()
        .context("Failed to create database pool")?;

    // let connection = &mut establish_connection();

    // Database connection pool
    // let pg_config = tokio_postgres::Config::from_str(&database_url)?;
    // let manager = deadpool_postgres::Manager::from_config(
    //     pg_config,
    //     postgres_native_tls::MakeTlsConnector::new(native_tls::TlsConnector::new()?),
    //     deadpool_postgres::ManagerConfig {
    //         recycling_method: deadpool_postgres::RecyclingMethod::Fast,
    //     },
    // );
    // let pool = deadpool_postgres::Pool::builder(manager)
    //     .max_size(20)
    //     .build()
    //     .context("Failed to create database pool")?;
    //
    println!("Connected to database");

    let addr = ("[::]:".to_owned() + &port).parse()?;
    println!("Server started on port: {}", port);

    let server = MyService { pool };
    let svc = UsersServiceServer::new(server);
    Server::builder().add_service(svc).serve(addr).await?;

    Ok(())
}
