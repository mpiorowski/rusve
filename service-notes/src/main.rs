mod notes_service;
mod proto;
mod utils;

use anyhow::{Context, Result};
use proto::notes_service_server::NotesServiceServer;
use rcgen::generate_simple_self_signed;
use sqlx::{postgres::PgPoolOptions, PgPool};
use tonic::{
    transport::{Certificate, Channel, ClientTlsConfig, Server},
    Status,
};
use utils::check_env;

use crate::proto::users_service_client::UsersServiceClient;

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
    let channel = create_user_channel().await?;
    let users_conn = UsersServiceClient::new(channel);

    let service = MyService { pool, users_conn };

    println!("Server started on port: {}", port);
    Server::builder()
        .add_service(NotesServiceServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}

async fn create_user_channel() -> Result<Channel> {
    let uri_users = check_env("URI_USERS")?;
    let channel_users = tonic::transport::Channel::from_shared(uri_users.to_owned())
        .context("Failed to create channel to users service")?;
    if check_env("ENV")? == "development" {
        let channel = channel_users
            .connect()
            .await
            .context("Failed to connect to users service")?;
        Ok(channel)
    } else {
        // TODO - env
        let subject_alt_names = vec!["xxx".to_string(), "localhost".to_string()];
        let cert = generate_simple_self_signed(subject_alt_names)
            .unwrap()
            .serialize_pem()
            .unwrap();
        let server_cert = cert.as_bytes();
        let tonic_cert = Certificate::from_pem(server_cert);
        let tls = ClientTlsConfig::new()
            .ca_certificate(tonic_cert)
            .domain_name("xxxx");

        let pem = tokio::fs::read("/etc/ssl/cert.pem")
            .await
            .expect("Failed to read cert.pem");
        let cert = Certificate::from_pem(pem);
        let tls = ClientTlsConfig::new()
            .ca_certificate(cert)
            .domain_name("rust-grpc-notes-jtq3bgjqeq-lz.a.run.app");
        let channel = channel_users
            .tls_config(tls)
            .context("Failed to create tls config to users service")?
            .connect()
            .await
            .context("Failed to connect to users service")?;
        Ok(channel)
    }
}
