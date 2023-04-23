mod notes_service;
mod proto;
mod utils;

use crate::proto::users_service_client::UsersServiceClient;
use anyhow::{Context, Result};
use proto::notes_service_server::NotesServiceServer;
use sqlx::{postgres::PgPoolOptions, PgPool};
use tonic::{transport::Server, Request, Status};
use utils::{check_env, decode_token};

#[derive(Debug)]
pub struct MyService {
    pool: PgPool,
    users_conn: UsersServiceClient<tonic::transport::Channel>,
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
    println!("Connecting to users service at: {}", uri_users);
    let users_conn = UsersServiceClient::connect(uri_users)
        .await
        .context("Failed to connect to users service")?;

    println!("Server started on port: {}", port);

    let server = MyService { pool, users_conn };
    let svc = NotesServiceServer::with_interceptor(server, check_auth);
    Server::builder().add_service(svc).serve(addr).await?;

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
