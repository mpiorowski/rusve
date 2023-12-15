mod migrations;
mod proto;
mod users_db;
mod users_service;

use crate::proto::users_service_server::UsersServiceServer;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use tonic::{Request, Status};

pub struct MyService {
    pool: deadpool_postgres::Pool,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    let filter = std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_owned());
    tracing_subscriber::fmt().with_env_filter(filter).init();

    // Connect to database
    let pool = rusve_users::connect_to_db().context("Failed to connect to database")?;
    tracing::info!("Connected to database");

    // Run migrations
    migrations::run_migrations(&pool)
        .await
        .context("Failed to run migrations")?;
    tracing::info!("Migrations complete");

    // Run gRPC server
    let port = std::env::var("PORT").context("PORT not set")?;
    let addr = format!("[::]:{}", port).parse()?;
    tracing::info!("gRPC server started on port: {:?}", port);
    let server = MyService { pool };
    let svc = UsersServiceServer::with_interceptor(server, check_auth);
    tonic::transport::Server::builder()
        .add_service(svc)
        .serve(addr)
        .await?;

    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    token: String,
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
