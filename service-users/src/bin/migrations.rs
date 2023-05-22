use anyhow::{Context, Result};
use sqlx;
use tokio;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Running sqlx migrations...");
    let database_url = "postgresql:///?host=localhost&user=postgres&password=12345&dbname=users";
    let pool_sqlx = sqlx::postgres::PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .with_context(|| format!("Failed to connect to database: {}", database_url))?;
    sqlx::migrate!("./migrations")
        .run(&pool_sqlx)
        .await
        .context("Failed to run migrations")?;
    println!("Migration run successfully");
    Ok(())
}
