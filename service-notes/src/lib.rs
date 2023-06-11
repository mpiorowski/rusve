use anyhow::{Context, Result};
use diesel::{Connection, ConnectionResult};
use diesel_async::pooled_connection::deadpool::Pool;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::AsyncMysqlConnection;
use futures_util::future::{BoxFuture, FutureExt};

pub fn establish_connection(database_url: &str) -> Result<Pool<AsyncMysqlConnection>> {
    let config = AsyncDieselConnectionManager::<AsyncMysqlConnection>::new(database_url);
    let pool = Pool::builder(config)
        .build()
        .context("Error creating pool")?;
    Ok(pool)
}

pub fn establish_connection_sync(database_url: &str) -> Result<diesel::mysql::MysqlConnection> {
    let conn = diesel::mysql::MysqlConnection::establish(database_url)?;
    Ok(conn)
}

pub async fn establish_connection_tls(database_url: &str) -> Result<Pool<AsyncMysqlConnection>> {
    let mgr = AsyncDieselConnectionManager::<AsyncMysqlConnection>::new_with_setup(
        database_url,
        create_tls_connection,
    );
    let pool = Pool::builder(mgr)
        .max_size(10)
        .build()
        .context("Error creating pool")?;
    Ok(pool)
}

fn create_tls_connection(database_url: &str) -> BoxFuture<ConnectionResult<AsyncMysqlConnection>> {
    let fut = async {
        let opts = mysql_async::Opts::from_url(database_url).map_err(|e| {
            diesel::ConnectionError::BadConnection(format!("Invalid database URL: {}", e))
        })?;
        let client = mysql_async::Conn::new(opts).await.map_err(|e| {
            diesel::ConnectionError::BadConnection(format!("Could not connect to database: {}", e))
        })?;
        AsyncMysqlConnection::try_from(client).await
    };
    fut.boxed()
}
