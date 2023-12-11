mod notes_db;
mod notes_service;
mod proto;

use std::ops::DerefMut;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use tonic::{Request, Status};

use crate::proto::notes_service_server::NotesServiceServer;

mod embedded {
    use refinery::embed_migrations;
    embed_migrations!("./migrations");
}

pub struct MyService {
    pool: deadpool_postgres::Pool,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().init();

    let port = std::env::var("PORT").context("PORT not set")?;
    let pool = rusve_notes::connect_to_db().context("Failed to connect to database")?;
    tracing::info!("Connected to database");

    let mut conn = pool.get().await?;
    let client = conn.deref_mut().deref_mut();
    embedded::migrations::runner().run_async(client).await?;
    tracing::info!("Migrations run");

    let addr = format!("[::]:{}", port).parse()?;
    tracing::info!("gRPC server started on port: {:?}", port);

    let server = MyService { pool };
    let svc = NotesServiceServer::with_interceptor(server, check_auth);
    tonic::transport::Server::builder()
        .add_service(svc)
        .serve(addr)
        .await?;

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
