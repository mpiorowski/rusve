use anyhow::{Context, Result};
use deadpool_postgres::{Manager, ManagerConfig, Pool, RecyclingMethod};
use rustls::RootCertStore;
use rustls_native_certs::load_native_certs;
use std::str::FromStr;
use tokio_postgres_rustls::MakeRustlsConnect;

#[derive(Clone)]
pub struct Env {
    pub port: String,
    pub rust_log: String,
    pub database_url: String,
    pub server_url: String,
    pub client_url: String,
    pub google_client_id: String,
    pub google_client_secret: String,
    pub github_client_id: String,
    pub github_client_secret: String,
    pub jwt_secret: String,
}

pub fn init_envs() -> Result<Env> {
    let port = std::env::var("PORT").context("PORT not set")?;
    let rust_log = std::env::var("RUST_LOG").context("RUST_LOG not set")?;
    let database_url = std::env::var("DATABASE_URL").context("DATABASE_URL not set")?;
    let server_url = std::env::var("SERVER_URL").context("SERVER_URL not set")?;
    let client_url = std::env::var("CLIENT_URL").context("CLIENT_URL not set")?;
    let google_client_id = std::env::var("GOOGLE_CLIENT_ID").context("GOOGLE_CLIENT_ID not set")?;
    let google_client_secret =
        std::env::var("GOOGLE_CLIENT_SECRET").context("GOOGLE_CLIENT_SECRET not set")?;
    let github_client_id = std::env::var("GITHUB_CLIENT_ID").context("GITHUB_CLIENT_ID not set")?;
    let github_client_secret =
        std::env::var("GITHUB_CLIENT_SECRET").context("GITHUB_CLIENT_SECRET not set")?;
    let jwt_secret = std::env::var("JWT_SECRET").context("JWT_SECRET not set")?;

    Ok(Env {
        port,
        rust_log,
        database_url,
        server_url,
        client_url,
        google_client_id,
        google_client_secret,
        github_client_id,
        github_client_secret,
        jwt_secret,
    })
}

pub fn connect_to_db(env: &Env) -> Result<deadpool_postgres::Pool> {
    let tokio_config = tokio_postgres::Config::from_str(env.database_url.as_str())?;
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

