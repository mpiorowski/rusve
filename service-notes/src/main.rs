mod notes_service;
mod proto;

use anyhow::{Context, Result};
use proto::notes_service_server::NotesServiceServer;
use tonic::transport::Server;

mod embedded {
    use refinery::embed_migrations;
    embed_migrations!("./migrations");
}

pub struct MyService {
    pool: mysql_async::Pool,
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("Starting server...");

    let port = std::env::var("PORT").context("PORT not set")?;
    let database_url = std::env::var("DATABASE_URL").context("DATABASE_URL not set")?;

    let opts = mysql_async::Opts::from_url(&database_url)?;
    let mut pool = mysql_async::Pool::new(opts);

    embedded::migrations::runner().run_async(&mut pool).await?;
    println!("Migrations run successfully");

    let addr = ("[::]:".to_owned() + &port).parse()?;
    println!("Server started on port: {}", port);

    let server = MyService { pool };
    let svc = NotesServiceServer::new(server);
    Server::builder().add_service(svc).serve(addr).await?;

    Ok(())
}
