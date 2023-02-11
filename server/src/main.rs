mod notes_service;
mod proto;
mod utils;

use async_once::AsyncOnce;
use lazy_static::lazy_static;

use anyhow::Result;
use proto::notes_service_server::NotesServiceServer;
use sqlx::{postgres::PgPoolOptions, PgPool};
use tonic::{transport::Server, Status};
use utils::check_env;

#[derive(Debug, Default)]
pub struct MyNotes {}

trait IntoStatus {
    fn into_status(self) -> Status;
}

impl IntoStatus for sqlx::Error {
    fn into_status(self: sqlx::Error) -> Status {
        Status::internal(self.to_string())
    }
}

lazy_static! {
    static ref POOL: AsyncOnce<PgPool> = AsyncOnce::new(async {
        let database_url = "postgres://postgres:12345@db-notes/notes";
        PgPoolOptions::new()
            .max_connections(20)
            .connect(database_url)
            .await
            .unwrap()
    });
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("Starting server...");

    sqlx::migrate!("./migrations")
        .run(POOL.get().await)
        .await
        .expect("Failed to run migrations");
    println!("Migrations ran successfully");

    let port = check_env("PORT")?;
    let addr = ("0.0.0.0:".to_owned() + &port).parse()?;
    let notes = MyNotes::default();

    println!("Server started on port: {}", port);
    Server::builder()
        .add_service(NotesServiceServer::new(notes))
        .serve(addr)
        .await?;

    Ok(())
}
