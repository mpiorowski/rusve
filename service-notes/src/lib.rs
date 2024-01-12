use anyhow::{Context, Result};
use deadpool_postgres::{Manager, ManagerConfig, Pool, RecyclingMethod};
use rustls::RootCertStore;
use rustls_native_certs::load_native_certs;
use std::str::FromStr;
use tokio_postgres_rustls::MakeRustlsConnect;
use tonic::metadata::{Ascii, MetadataValue};
mod proto;

#[derive(Clone)]
pub struct Env {
    pub port: String,
    pub rust_log: String,
    pub database_url: String,
    pub users_url: String,
    pub jwt_secret: String,
}

pub fn init_envs() -> Result<Env> {
    Ok(Env {
        port: std::env::var("PORT").context("PORT is not set")?,
        rust_log: std::env::var("RUST_LOG").context("RUST_LOG is not set")?,
        database_url: std::env::var("DATABASE_URL").context("DATABASE_URL is not set")?,
        users_url: std::env::var("USERS_URL").context("USERS_URL is not set")?,
        jwt_secret: std::env::var("JWT_SECRET").context("JWT_SECRET is not set")?,
    })
}

pub fn connect_to_db(env: &Env) -> Result<deadpool_postgres::Pool> {
    let tokio_config = tokio_postgres::Config::from_str(&env.database_url)?;
    let mgr_config = ManagerConfig {
        recycling_method: RecyclingMethod::Fast,
    };
    let certs = load_native_certs().context("Failed to load platform certs")?;
    let mut store = RootCertStore::empty();
    for cert in certs {
        store.add(&rustls::Certificate(cert.0))?;
    }
    let config = rustls::ClientConfig::builder()
        .with_safe_defaults()
        .with_root_certificates(store)
        .with_no_client_auth();
    let tls = MakeRustlsConnect::new(config);
    let mgr = Manager::from_config(tokio_config, tls, mgr_config);
    let pool = Pool::builder(mgr).build()?;
    Ok(pool)
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Claims {
    pub id: String,
    pub exp: i64,
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

pub fn generate_jwt_token(jwt_secret: &str, user_id: &str) -> Result<MetadataValue<Ascii>> {
    let jwt_token = match jsonwebtoken::encode(
        &jsonwebtoken::Header::new(jsonwebtoken::Algorithm::HS256),
        &Claims {
            id: user_id.to_string(),
            // 10 minutes
            exp: time::OffsetDateTime::now_utc().unix_timestamp() + 60 * 10,
        },
        &jsonwebtoken::EncodingKey::from_secret(jwt_secret.as_ref()),
    ) {
        Ok(token) => token,
        Err(e) => {
            tracing::error!("Failed to encode jwt token: {:?}", e);
            return Err(anyhow::anyhow!("Failed to encode jwt token"));
        }
    };
    Ok(format!("bearer {}", jwt_token).parse()?)
}
