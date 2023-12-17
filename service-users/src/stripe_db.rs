use std::str::FromStr;

use anyhow::Result;
use deadpool_postgres::Object;
use uuid::Uuid;

use crate::proto::User;

pub async fn update_user_subscription_id(
    conn: &Object,
    user_id: &str,
    subscription_id: &str,
) -> Result<User> {
    let user_id: Uuid = Uuid::from_str(user_id)?;
    let user: tokio_postgres::Row = conn
        .query_one(
            "update users set subscription_id = $1 where id = $2 returning *",
            &[&subscription_id, &user_id],
        )
        .await?;
    let user: User = User::try_from(user)?;
    Ok(user)
}

pub async fn update_user_subscription_end(
    conn: &Object,
    user_id: &str,
    subscription_end: i64,
) -> Result<User> {
    let user_id: Uuid = Uuid::from_str(user_id)?;
    let subscription_end: time::OffsetDateTime =
        time::OffsetDateTime::from_unix_timestamp(subscription_end)?;
    let user: tokio_postgres::Row = conn
        .query_one(
            "update users set subscription_end = $1 where id = $2 returning *",
            &[&subscription_end, &user_id],
        )
        .await?;
    let user: User = User::try_from(user)?;
    Ok(user)
}

pub async fn update_user_subscription_check(conn: &Object, user_id: &str) -> Result<User> {
    let user_id: Uuid = Uuid::from_str(user_id)?;
    let user: tokio_postgres::Row = conn
        .query_one(
            "update users set subscription_check = now() where id = $1 returning *",
            &[&user_id],
        )
        .await?;
    let user: User = User::try_from(user)?;
    Ok(user)
}

pub async fn remove_user_subscription_check(conn: &Object, user_id: &str) -> Result<User> {
    let user_id: Uuid = Uuid::from_str(user_id)?;
    let user: tokio_postgres::Row = conn
        .query_one(
            "update users set subscription_check = '-infinity' where id = $1 returning *",
            &[&user_id],
        )
        .await?;
    let user: User = User::try_from(user)?;
    Ok(user)
}
