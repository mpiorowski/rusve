mod proto;
mod users_service;
mod utils;

use crate::proto::users_service_server::UsersServiceServer;
use anyhow::{Context, Result};
use jsonwebtoken::{DecodingKey, Validation};
use serde::Deserialize;
use sqlx::{postgres::PgPoolOptions, PgPool};
use tonic::{transport::Server, Request, Status};
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
    println!("Connected to database");

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

fn check_auth(mut req: Request<()>) -> Result<Request<()>, Status> {
    match req.metadata().get("authorization") {
        Some(t) => {
            let token = t
                .to_str()
                .map_err(|_| Status::unauthenticated("Invalid auth token"))?;
            let secret =
                check_env("SECRET").map_err(|_| Status::unauthenticated("Missing auth secret"))?;
            let token = token.trim_start_matches("Bearer ");
            let user_id = decode_token(token, secret.as_ref())
                .map_err(|_| Status::unauthenticated("Invalid auth token"))?;
            req.metadata_mut().insert(
                "user_id",
                user_id
                    .parse()
                    .map_err(|_| Status::unauthenticated("Invalid user id"))?,
            );
            Ok(req)
        }
        _ => Err(Status::unauthenticated("No valid auth token")),
    }
}

#[derive(Debug, Deserialize)]
struct Claims {
    user_id: String,
}

fn decode_token(token: &str, secret: &[u8]) -> Result<String, jsonwebtoken::errors::Error> {
    let validation = Validation::new(jsonwebtoken::Algorithm::HS256);
    let decoding_key = DecodingKey::from_secret(secret);
    let token_data = jsonwebtoken::decode::<Claims>(token, &decoding_key, &validation)?;
    Ok(token_data.claims.user_id)
}
