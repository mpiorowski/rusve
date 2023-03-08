mod proto;
mod users_service;
mod utils;

use crate::proto::users_service_server::UsersServiceServer;
use anyhow::{Context, Result};
use hmac::Hmac;
use sha2::Sha256;
use sqlx::{postgres::PgPoolOptions, PgPool};
use tonic::{metadata::MetadataValue, transport::Server, Request, Status};
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

    let database_url = check_env("DATABASE_URL")?;
    let pool = PgPoolOptions::new()
        .max_connections(20)
        .connect(&database_url)
        .await
        .with_context(|| format!("Failed to connect to database: {}", database_url))?;

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .context("Failed to run migrations")?;

    println!("Migrations ran successfully");

    let port = check_env("PORT")?;
    let addr = ("0.0.0.0:".to_owned() + &port)
        .parse()
        .context("Failed to parse address")?;

    println!("Server started on port: {}", port);

    let server = MyService { pool };
    let svc = UsersServiceServer::with_interceptor(server, check_auth);
    Server::builder()
        .add_service(svc)
        .serve(addr)
        .await
        .context("Failed to start server")?;

    Ok(())
}

fn check_auth(req: Request<()>) -> Result<Request<()>, Status> {
    let token: MetadataValue<_> = "Bearer some-secret-token".parse().unwrap();

    let key: Hmac<Sha256> = Hmac::new_from_slice(b"my-secret-key")?;

    match req.metadata().get("authorization") {
        Some(t) => {
            validate_token(t.to_str()?, key.as_ref())?;
            Ok(req)
        }
        _ => Err(Status::unauthenticated("No valid auth token")),
    }
}

fn validate_token(token: &str, secret: &[u8]) -> Result<(), jsonwebtoken::errors::Error> {
    let validation = Validation::default();
    let decoding_key = DecodingKey::from_secret(secret);

    decode::<serde_json::Value>(token, &decoding_key, &validation)?;
    Ok(())
}
