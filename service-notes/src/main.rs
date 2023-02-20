mod notes_service;
mod proto;
mod utils;

use anyhow::{Context, Result};
use proto::notes_service_server::NotesServiceServer;
use sqlx::{postgres::PgPoolOptions, PgPool};
use tonic::{
    transport::{Channel, Server},
    Request, Status,
};
use utils::check_env;

use crate::{proto::users_service_client::UsersServiceClient, utils::fetch_auth_token};

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

    // Database
    let database_url = check_env("DATABASE_URL")?;
    let pool = PgPoolOptions::new()
        .max_connections(20)
        .connect(&database_url)
        .await
        .with_context(|| format!("Failed to connect to database: {}", database_url))?;

    // Migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .context("Failed to run migrations")?;
    println!("Migrations ran successfully");

    let port = check_env("PORT")?;
    let addr = ("0.0.0.0:".to_owned() + &port).parse()?;

    // Users service
    let uri_users = check_env("URI_USERS")?;
    let token = fetch_auth_token(&uri_users).await.unwrap();
    let channel = Channel::from_shared(uri_users);
    if let Err(e) = channel {
        panic!("Failed to connect to users service: {}", e);
    }
    let channel = channel.unwrap().connect().await.unwrap();

    let mut users_conn =
        UsersServiceClient::with_interceptor(channel, move |mut req: Request<()>| {
            req.metadata_mut()
                .insert("authorization", token.parse().unwrap());
            Ok(req)
        });

    let service = MyService { pool };

    println!("Server started on port: {}", port);
    Server::builder()
        .add_service(NotesServiceServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}
