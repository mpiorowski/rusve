mod notes_service;
mod proto;
mod utils;

use anyhow::Result;
use proto::notes_service_server::NotesServiceServer;
use sqlx::{postgres::PgPoolOptions, PgPool};
use tonic::{transport::Server, Status};
use utils::check_env;

trait IntoStatus {
    fn into_status(self) -> Status;
}

impl IntoStatus for sqlx::Error {
    fn into_status(self: sqlx::Error) -> Status {
        Status::internal(self.to_string())
    }
}

#[derive(Debug)]
pub struct MyService {
    pool: PgPool,
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("Starting server...");

    let database_url = check_env("DATABASE_URL")? + "/notes";
    let pool = PgPoolOptions::new()
        .max_connections(20)
        .connect(&database_url)
        .await
        .expect("Failed to connect to database");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");
    println!("Migrations ran successfully");

    let port = check_env("PORT")?;
    let addr = ("0.0.0.0:".to_owned() + &port).parse()?;
    let service = MyService { pool };

    println!("Server started on port: {}", port);
    Server::builder()
        .add_service(NotesServiceServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}
