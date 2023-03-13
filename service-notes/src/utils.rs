use anyhow::Result;
use jsonwebtoken::{encode, EncodingKey, Header, Validation, DecodingKey};
use serde::{Deserialize, Serialize};
use tonic::metadata::{MetadataMap, MetadataValue};

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

pub fn create_auth_metadata(user_id: &String) -> Result<MetadataMap> {
    let secret = check_env("SECRET")?;
    let token = encode_token(user_id, secret.as_ref())?;
    let metadata_value = MetadataValue::try_from("Bearer ".to_owned() + &token)?;
    let mut metadata = MetadataMap::new();
    metadata.insert("authorization", metadata_value);
    Ok(metadata)
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

pub fn encode_token(user_id: &String, secret: &[u8]) -> Result<String> {
    let current_time = time::OffsetDateTime::now_utc();
    let claims = Claims {
        user_id: user_id.to_string(),
        exp: (current_time + time::Duration::hours(1)).unix_timestamp() as usize,
    };
    let token = encode(
        &Header::new(jsonwebtoken::Algorithm::HS256),
        &claims,
        &EncodingKey::from_secret(secret),
    )?;
    Ok(token)
}
