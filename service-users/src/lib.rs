use anyhow::{Context, Result};
use deadpool_postgres::{Manager, ManagerConfig, Pool, RecyclingMethod};
use rustls::RootCertStore;
use rustls_native_certs::load_native_certs;
use std::str::FromStr;
use tokio_postgres_rustls::MakeRustlsConnect;
mod proto;

#[derive(Clone)]
pub struct Env {
    pub port: String,
    pub rust_log: String,
    pub database_url: String,
    pub client_url: String,
    pub stripe_api_key: String,
    pub stripe_price_id: String,
    pub jwt_secret: String,
}

pub fn init_envs() -> Result<Env> {
    let port = std::env::var("PORT").context("PORT not set")?;
    let rust_log = std::env::var("RUST_LOG").context("RUST_LOG not set")?;
    let database_url = std::env::var("DATABASE_URL").context("DATABASE_URL not set")?;
    let client_url = std::env::var("CLIENT_URL").context("CLIENT_URL not set")?;
    let stripe_api_key = std::env::var("STRIPE_API_KEY").context("STRIPE_API_KEY not set")?;
    let stripe_price_id = std::env::var("STRIPE_PRICE_ID").context("STRIPE_PRICE_ID not set")?;
    let jwt_secret = std::env::var("JWT_SECRET").context("JWT_SECRET not set")?;

    Ok(Env {
        port,
        rust_log,
        database_url,
        client_url,
        stripe_api_key,
        stripe_price_id,
        jwt_secret,
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


pub fn extract_token(metadata: &tonic::metadata::MetadataMap) -> Result<&str, tonic::Status> {
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
    Ok(token)
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Claims {
    pub token: String,
    pub user_id: String,
}
pub fn decode_token(metadata: &tonic::metadata::MetadataMap) -> Result<Claims, tonic::Status> {
    let token = extract_token(metadata)?;
    let decoding_key = jsonwebtoken::DecodingKey::from_rsa_pem(include_bytes!("../public.key"))
        .map_err(|e| {
            tracing::error!("Failed to parse public key: {:?}", e);
            tonic::Status::unauthenticated("Invalid authorization token")
        })?;
    let token_message = jsonwebtoken::decode::<Claims>(
        token,
        &decoding_key,
        &jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::RS256),
    )
    .map_err(|e| {
        tracing::error!("Failed to decode authorization token: {:?}", e);
        tonic::Status::unauthenticated("Invalid authorization token")
    })?;

    Ok(token_message.claims)
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct OAuthUser {
    pub sub: String,
    pub email: String,
    pub avatar: String,
    pub exp: i64,
}
pub fn decode_oauth_token(metadata: &tonic::metadata::MetadataMap, env: &Env) -> Result<OAuthUser, tonic::Status> {
    let token = extract_token(metadata)?;
    let token_message = jsonwebtoken::decode::<OAuthUser>(
        token,
        &jsonwebtoken::DecodingKey::from_secret(env.jwt_secret.as_bytes()),
        &jsonwebtoken::Validation::default(),
    )
    .map_err(|e| {
        tracing::error!("Failed to decode authorization token: {:?}", e);
        tonic::Status::unauthenticated("Invalid authorization token")
    })?;

    Ok(token_message.claims)
}
