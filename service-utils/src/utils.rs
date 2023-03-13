use anyhow::Result;
use jsonwebtoken::{Validation, DecodingKey};
use serde::{Deserialize, Serialize};

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


#[derive(Debug, Deserialize, Serialize)]
struct Claims {
    user_id: String,
    exp: usize,
}

pub fn decode_token(token: &str, secret: &[u8]) -> Result<String, jsonwebtoken::errors::Error> {
    let validation = Validation::new(jsonwebtoken::Algorithm::HS256);
    let decoding_key = DecodingKey::from_secret(secret);
    let token_data = jsonwebtoken::decode::<Claims>(token, &decoding_key, &validation)?;
    Ok(token_data.claims.user_id)
}
