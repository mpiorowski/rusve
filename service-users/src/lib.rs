use diesel::Connection;
use anyhow::{Context, Result};
use diesel_async::{
    pooled_connection::{deadpool::Pool, AsyncDieselConnectionManager},
    AsyncPgConnection,
};
mod proto;

pub fn establish_connection(database_url: &str) -> Result<Pool<AsyncPgConnection>> {
    let config = AsyncDieselConnectionManager::<AsyncPgConnection>::new(database_url);
    let pool = Pool::builder(config)
        .build()
        .context("Error creating pool")?;
    return Ok(pool);
}

pub fn establish_connection_sync(database_url: &str) -> Result<diesel::pg::PgConnection> {
    let conn = diesel::pg::PgConnection::establish(&database_url)?;
    return Ok(conn);
}
