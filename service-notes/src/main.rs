mod models;
mod notes_service;
mod proto;
mod schema;

use anyhow::{Context, Result};
use diesel_async::{pooled_connection::AsyncDieselConnectionManager, AsyncPgConnection};
use proto::notes_service_server::NotesServiceServer;
use rusve_notes::establish_connection;
use tonic::transport::Server;

type DPool = deadpool::managed::Pool<AsyncDieselConnectionManager<AsyncPgConnection>>;
pub struct MyService {
    pool: DPool,
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
    let pool = diesel_async::pooled_connection::deadpool::Pool::builder(mgr).build()?;

    let addr = ("[::]:".to_owned() + &port).parse()?;
    println!("Server started on port: {}", port);

    let server = MyService { pool };
    let svc = NotesServiceServer::new(server);
    Server::builder().add_service(svc).serve(addr).await?;

    Ok(())
}
