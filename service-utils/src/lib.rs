use anyhow::{Context, Result};
use deadpool_postgres::{Manager, ManagerConfig, Pool, RecyclingMethod};
use std::str::FromStr;
mod proto;

#[derive(Clone)]
pub struct Env {
    pub port: String,
    pub rust_log: String,
    pub database_url: String,
    pub target: String,
    pub sendgrid_api_key: String,
    pub s3_access_key: String,
    pub s3_secret_key: String,
    pub s3_endpoint: String,
    pub s3_bucket_name: String,
}

pub fn init_envs() -> Result<Env> {
    Ok(Env {
        port: std::env::var("PORT")?,
        rust_log: std::env::var("RUST_LOG")?,
        database_url: std::env::var("DATABASE_URL")?,
        target: std::env::var("TARGET")?,
        sendgrid_api_key: std::env::var("SENDGRID_API_KEY")?,
        s3_access_key: std::env::var("S3_ACCESS_KEY")?,
        s3_secret_key: std::env::var("S3_SECRET_KEY")?,
        s3_endpoint: std::env::var("S3_ENDPOINT")?,
        s3_bucket_name: std::env::var("S3_BUCKET_NAME")?,
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

pub async fn connect_to_bucket(env: &Env) -> Result<s3::Bucket> {
    let s3_access_key = env.s3_access_key.clone();
    let s3_secret_key = env.s3_secret_key.clone();
    let s3_endpoint = env.s3_endpoint.clone();
    let s3_bucket_name = env.s3_bucket_name.clone();

    let credentials = s3::creds::Credentials::new(
        Option::from(s3_access_key).as_deref(), // access_key
        Option::from(s3_secret_key).as_deref(), // secret_key
        None,
        None,
        None,
    )?;

    let region = s3::Region::Custom {
        region: "auto".to_owned(),
        endpoint: s3_endpoint,
    };

    let bucket = s3::Bucket::new(&s3_bucket_name, region, credentials)?.with_path_style();
    Ok(bucket)
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Claims {
    pub id: String,
}
pub fn auth(metadata: &tonic::metadata::MetadataMap) -> Result<Claims, tonic::Status> {
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
