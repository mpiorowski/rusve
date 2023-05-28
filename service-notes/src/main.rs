mod models;
mod notes_service;
mod proto;
mod schema;

use anyhow::{Context, Result};
use proto::notes_service_server::NotesServiceServer;
use rusve_notes::establish_connection;
use tonic::transport::Server;

pub struct MyService {
    // pool: diesel_async::pooled_connection::deadpool::Pool<
    //     diesel_async::pooled_connection::AsyncDieselConnectionManager<
    //         diesel_async::AsyncPgConnection,
    //     >,
    // >,
    pool: deadpool::managed::Pool<
        diesel_async::pooled_connection::AsyncDieselConnectionManager<
            diesel_async::AsyncPgConnection,
        >,
    >,
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("Starting server...");

    let port = std::env::var("PORT").context("PORT not set")?;
    let database_url = std::env::var("DATABASE_URL").context("DATABASE_URL not set")?;

    // Create a connection pool without tls
    let pool = establish_connection(&database_url)?;

    // Create a connection pool with tls
    // let mgr = AsyncDieselConnectionManager::<AsyncPgConnection>::new_with_setup(
    //     database_url,
    //     establish_connection_tls,
    // );
    // let pool = diesel_async::pooled_connection::deadpool::Pool::builder(mgr).build()?;

    let addr = ("[::]:".to_owned() + &port).parse()?;
    println!("Server started on port: {}", port);

    let server = MyService { pool };
    let svc = NotesServiceServer::new(server);
    Server::builder().add_service(svc).serve(addr).await?;

    Ok(())
}
