mod models;
mod proto;
mod schema;
mod users_service;

use crate::proto::users_service_server::UsersServiceServer;
use anyhow::{Context, Result};
use diesel_async::{pooled_connection::deadpool::Pool, AsyncPgConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use rusve_users::{establish_connection_sync, establish_connection_tls};
use serde::{Deserialize, Serialize};
use tonic::{transport::Server, Request, Status};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub struct MyService {
    pool: Pool<AsyncPgConnection>,
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("Starting server...");

    let port = std::env::var("PORT").context("PORT not set")?;
    let database_url = std::env::var("DATABASE_URL").context("DATABASE_URL not set")?;

    // Run migrations - diesel_async have an open PR to support this
    let mut conn = establish_connection_sync(&database_url)?;
    conn.run_pending_migrations(MIGRATIONS)
        .map_err(|e| anyhow::anyhow!("Error running migrations: {:?}", e.to_string()))?;
    println!("Migrations run successfully");

    // Create a connection pool without tls
    let pool = establish_connection_tls(&database_url)?;

    let addr = ("[::]:".to_owned() + &port).parse()?;
    println!("Server started on port: {}", port);

    let server = MyService { pool };
    let svc = UsersServiceServer::with_interceptor(server, check_auth);
    Server::builder().add_service(svc).serve(addr).await?;

    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
}
fn check_auth(req: Request<()>) -> Result<Request<()>, Status> {
    let token = match req.metadata().get("x-authorization") {
        Some(token) => token,
        None => return Err(Status::unauthenticated("Missing authorization token")),
    };
    let token = token
        .to_str()
        .map_err(|_| Status::unauthenticated("Invalid authorization token"))?
        .strip_prefix("bearer ")
        .ok_or_else(|| Status::unauthenticated("Invalid authorization token"))?;

    let token_message = jsonwebtoken::decode::<Claims>(
        token,
        &jsonwebtoken::DecodingKey::from_rsa_pem(include_bytes!("../public.key"))
            .map_err(|_| Status::unauthenticated("Invalid authorization token"))?,
        &jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::RS256),
    );

    match token_message {
        Ok(_) => Ok(req),
        Err(_) => Err(Status::unauthenticated("Invalid authorization token")),
    }
}
