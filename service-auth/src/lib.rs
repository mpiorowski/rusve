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
    Ok(Env {
        port: std::env::var("PORT")?,
        rust_log: std::env::var("RUST_LOG")?,
        database_url: std::env::var("DATABASE_URL")?,
        server_url: std::env::var("SERVER_URL")?,
        client_url: std::env::var("CLIENT_URL")?,
        google_client_id: std::env::var("GOOGLE_CLIENT_ID")?,
        google_client_secret: std::env::var("GOOGLE_CLIENT_SECRET")?,
        github_client_id: std::env::var("GITHUB_CLIENT_ID")?,
        github_client_secret: std::env::var("GITHUB_CLIENT_SECRET")?,
        jwt_secret: std::env::var("JWT_SECRET")?,
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

