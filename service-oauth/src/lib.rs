use anyhow::{Context, Result};
use deadpool_postgres::{Manager, ManagerConfig, Pool, RecyclingMethod};
use rustls::RootCertStore;
use rustls_native_certs::load_native_certs;
use std::str::FromStr;
use tokio_postgres_rustls::MakeRustlsConnect;

pub fn connect_to_db() -> Result<deadpool_postgres::Pool> {
    let database_url = std::env::var("DATABASE_URL")?;
    let tokio_config = tokio_postgres::Config::from_str(&database_url)?;
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
