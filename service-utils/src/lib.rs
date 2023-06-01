use anyhow::{Context, Result};
use diesel::Connection;
use diesel_async::pooled_connection::deadpool::Pool;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::AsyncPgConnection;

pub fn establish_connection(database_url: &str) -> Result<Pool<AsyncPgConnection>> {
    let config = AsyncDieselConnectionManager::<AsyncPgConnection>::new(database_url);
    let pool = Pool::builder(config)
        .build()
        .context("Error creating pool")?;
    Ok(pool)
}

pub fn establish_connection_sync(database_url: &str) -> Result<diesel::pg::PgConnection> {
    let conn = diesel::pg::PgConnection::establish(database_url)?;
    Ok(conn)
}
