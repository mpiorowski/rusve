use anyhow::Result;

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

pub async fn fetch_auth_token(service_uri: &str) -> Result<String> {
    let client = reqwest::Client::new();
    let res = client
        .get("http://metadata.google.internal/computeMetadata/v1/instance/service-accounts/default/identity?audience=https://".to_owned() + service_uri)
        .header("Metadata-Flavor", "Google")
        .send()
        .await?;

    let token = res.text().await?;
    let token = "Bearer ".to_owned() + &token;

    Ok(token)
}
