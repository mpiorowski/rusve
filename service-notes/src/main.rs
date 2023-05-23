mod models;
mod notes_service;
mod proto;
mod schema;

use anyhow::{Context, Result};
use diesel_async::{
    pooled_connection::{bb8::Pool, AsyncDieselConnectionManager},
    AsyncPgConnection,
};
use proto::notes_service_server::NotesServiceServer;
use rusve_notes::establish_connection;
use std::time::Duration;
use tonic::transport::Server;

#[derive(Debug)]
pub struct MyService {
    pool: Pool<AsyncDieselConnectionManager<AsyncPgConnection>>,
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("Starting server...");

    let port = std::env::var("PORT").context("PORT not set")?;
    let database_url = std::env::var("DATABASE_URL").context("DATABASE_URL not set")?;

    // create a new connection pool with the default config
    let mgr = AsyncDieselConnectionManager::<AsyncPgConnection>::new_with_setup(
        database_url,
        establish_connection,
    );
    let pool = Pool::builder()
        .max_size(10)
        .min_idle(Some(5))
        .max_lifetime(Some(Duration::from_secs(60 * 60 * 24)))
        .idle_timeout(Some(Duration::from_secs(60 * 2)))
        .build(mgr)
        .await?;

    let addr = ("[::]:".to_owned() + &port).parse()?;
    println!("Server started on port: {}", port);

    let server = MyService { pool };
    let svc = NotesServiceServer::new(server);
    Server::builder().add_service(svc).serve(addr).await?;

    Ok(())
}
