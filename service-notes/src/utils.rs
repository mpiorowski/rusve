use crate::CachedToken;
use anyhow::Result;
use std::sync::Arc;
use tokio::sync::Mutex;
use tonic::metadata::MetadataMap;

pub fn check_env(env_str: &str) -> Result<String> {
    let env = std::env::var(env_str);
    match env {
        Ok(env) => Ok(env),
        Err(_) => Err(anyhow::anyhow!(
            "Environment variable not found: {}",
            env_str
        )),
    }
}

pub async fn fetch_auth_metadata(
    cached_token: Arc<Mutex<CachedToken>>,
    service_uri: &str,
) -> Result<MetadataMap> {
    let mut metadata = MetadataMap::new();
    let env = check_env("ENV")?;
    if env != "production" {
        return Ok(metadata);
    }
    // check if token is expired
    let mut cached_token = cached_token.lock().await;
    if cached_token.expires > time::OffsetDateTime::now_utc() {
        println!("Using cached token");
        metadata.insert("authorization", cached_token.token.parse().unwrap());
        return Ok(metadata);
    }
    let client = reqwest::Client::new();
    let res = client
        .get("http://metadata.google.internal/computeMetadata/v1/instance/service-accounts/default/identity?audience=".to_owned() + service_uri)
        .header("Metadata-Flavor", "Google")
        .send()
        .await?;

    let token = res.text().await?;
    cached_token.token = token.clone();
    cached_token.expires = time::OffsetDateTime::now_utc() + time::Duration::hours(1);
    let token = "Bearer ".to_owned() + &token;
    metadata.insert("authorization", token.parse().unwrap());
    println!("Metadata: {:?}", metadata);

    Ok(metadata)
}
