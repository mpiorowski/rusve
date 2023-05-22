mod notes_service;
mod proto;

use anyhow::{Context, Result};
use proto::notes_service_server::NotesServiceServer;
use tonic::transport::Server;

#[derive(Debug)]
pub struct MyService {}

#[tokio::main]
async fn main() -> Result<()> {
    println!("Starting server...");

    let port = std::env::var("PORT").context("PORT not set")?;

    let addr = ("[::]:".to_owned() + &port).parse()?;
    println!("Server started on port: {}", port);

    let server = MyService {};
    let svc = NotesServiceServer::new(server);
    Server::builder().add_service(svc).serve(addr).await?;

    Ok(())
}
