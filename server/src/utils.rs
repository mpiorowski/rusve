use anyhow::Result;
use sqlx::{PgPool, postgres::PgPoolOptions};

pub fn check_env(env_str: &str) -> Result<String> {
    let env = std::env::var(env_str);
    match env {
        Ok(env) => Ok(env),
        Err(_) => Err(anyhow::anyhow!("{} is not set", env_str)),
    }
}

pub async fn connect_db() -> Result<PgPool> {
    let db_url = check_env("DATABASE_URL")?;
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await;
    match pool {
        Ok(pool) => Ok(pool),
        Err(e) => Err(anyhow::anyhow!("Failed to connect to database: {}", e)),
    }
}
