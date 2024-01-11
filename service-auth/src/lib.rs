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
    pub auth_url: String,
    pub client_url: String,
    pub google_client_id: String,
    pub google_client_secret: String,
    pub github_client_id: String,
    pub github_client_secret: String,
    pub jwt_secret: String,
}

pub fn init_envs() -> Result<Env> {
    Ok(Env {
        port: std::env::var("PORT").context("PORT is not set")?,
        rust_log: std::env::var("RUST_LOG").context("RUST_LOG is not set")?,
        database_url: std::env::var("DATABASE_URL").context("DATABASE_URL is not set")?,
        auth_url: std::env::var("AUTH_URL").context("AUTH_URL is not set")?,
        client_url: std::env::var("CLIENT_URL").context("CLIENT_URL is not set")?,
        google_client_id: std::env::var("GOOGLE_CLIENT_ID")
            .context("GOOGLE_CLIENT_ID is not set")?,
        google_client_secret: std::env::var("GOOGLE_CLIENT_SECRET")
            .context("GOOGLE_CLIENT_SECRET is not set")?,
        github_client_id: std::env::var("GITHUB_CLIENT_ID")
            .context("GITHUB_CLIENT_ID is not set")?,
        github_client_secret: std::env::var("GITHUB_CLIENT_SECRET")
            .context("GITHUB_CLIENT_SECRET is not set")?,
        jwt_secret: std::env::var("JWT_SECRET").context("JWT_SECRET is not set")?,
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
