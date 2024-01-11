use anyhow::{Context, Result};
use deadpool_postgres::{Manager, ManagerConfig, Pool, RecyclingMethod};
use std::str::FromStr;
mod proto;

#[derive(Clone)]
pub struct Env {
    pub port: String,
    pub rust_log: String,
    pub database_url: String,
    pub sendgrid_api_key: String,
    pub s3_bucket_name: String,
    pub s3_access_key: String,
    pub s3_secret_key: String,
    pub s3_endpoint: String,
    pub jwt_secret: String,
}

pub fn init_envs() -> Result<Env> {
    Ok(Env {
        port: std::env::var("PORT").context("PORT is not set")?,
        rust_log: std::env::var("RUST_LOG").context("RUST_LOG is not set")?,
        database_url: std::env::var("DATABASE_URL").context("DATABASE_URL is not set")?,
        sendgrid_api_key: std::env::var("SENDGRID_API_KEY")
            .context("SENDGRID_API_KEY is not set")?,
        s3_bucket_name: std::env::var("S3_BUCKET_NAME").context("S3_BUCKET_NAME is not set")?,
        s3_access_key: std::env::var("S3_ACCESS_KEY").context("S3_ACCESS_KEY is not set")?,
        s3_secret_key: std::env::var("S3_SECRET_KEY").context("S3_SECRET_KEY is not set")?,
        s3_endpoint: std::env::var("S3_ENDPOINT").context("S3_ENDPOINT is not set")?,
        jwt_secret: std::env::var("JWT_SECRET").context("JWT_SECRET is not set")?,
    })
}

pub fn connect_to_db(env: &Env) -> Result<deadpool_postgres::Pool> {
    let tokio_config = tokio_postgres::Config::from_str(&env.database_url)?;
    let mgr_config = ManagerConfig {
        recycling_method: RecyclingMethod::Fast,
    };
    let certs =
        rustls_native_certs::load_native_certs().context("Failed to load platform certs")?;
    let mut store = rustls::RootCertStore::empty();
    for cert in certs {
        store.add(&rustls::Certificate(cert.0))?;
    }
    let config = rustls::ClientConfig::builder()
        .with_safe_defaults()
        .with_root_certificates(store)
        .with_no_client_auth();
    let tls = tokio_postgres_rustls::MakeRustlsConnect::new(config);
    let mgr = Manager::from_config(tokio_config, tls, mgr_config);
    let pool = Pool::builder(mgr).build()?;
    Ok(pool)
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Claims {
    pub id: String,
}
pub fn auth(
    metadata: &tonic::metadata::MetadataMap,
    jwt_secret: &str,
) -> Result<Claims, tonic::Status> {
    let token = match metadata.get("x-authorization") {
        Some(token) => token,
        None => {
            tracing::error!("Missing authorization token");
            return Err(tonic::Status::unauthenticated(
                "Missing authorization token",
            ));
        }
    };
    let token = token
        .to_str()
        .map_err(|e| {
            tracing::error!("Failed to parse authorization token: {:?}", e);
            tonic::Status::unauthenticated("Invalid authorization token")
        })?
        .strip_prefix("bearer ")
        .ok_or_else(|| {
            tracing::error!("Failed to parse authorization token");
            tonic::Status::unauthenticated("Invalid authorization token")
        })?;

    let token_message = jsonwebtoken::decode::<Claims>(
        token,
        &jsonwebtoken::DecodingKey::from_secret(jwt_secret.as_ref()),
        &jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::HS256),
    )
    .map_err(|e| {
        tracing::error!("Failed to decode authorization token: {:?}", e);
        tonic::Status::unauthenticated("Invalid authorization token")
    })?;

    Ok(token_message.claims)
}
