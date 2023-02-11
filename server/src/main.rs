mod notes_service;
mod proto;
mod utils;

use anyhow::Result;
use proto::notes_service_server::NotesServiceServer;
use sqlx::{postgres::PgPoolOptions, PgPool};
use tonic::{transport::Server, Status};
use utils::check_env;

#[derive(Debug)]
pub struct MyNotes {
    pool: PgPool,
}

trait IntoStatus {
    fn into_status(self) -> Status;
}

impl IntoStatus for sqlx::Error {
    fn into_status(self: sqlx::Error) -> Status {
        Status::internal(self.to_string())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("Starting server...");

    let database_url = check_env("DATABASE_URL")?;
    let pool = PgPoolOptions::new()
        .max_connections(20)
        .connect(&database_url)
        .await
        .unwrap();

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");
    println!("Migrations ran successfully");

    let port = check_env("PORT")?;
    let addr = ("0.0.0.0:".to_owned() + &port).parse()?;
    let notes = MyNotes { pool };

    println!("Server started on port: {}", port);
    Server::builder()
        .add_service(NotesServiceServer::new(notes))
        .serve(addr)
        .await?;

    Ok(())
}
