use std::str::FromStr;

use anyhow::Result;
use deadpool_postgres::Object;
use time::format_description::well_known::Iso8601;
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
    subscription_end: &str,
) -> Result<User> {
    let user_id: Uuid = Uuid::from_str(user_id)?;
    let subscription_end: time::OffsetDateTime =
        time::OffsetDateTime::parse(subscription_end, &Iso8601::DEFAULT)?;
    let user: tokio_postgres::Row = conn
        .query_one(
            "update users set subscription_end = $1 where id = $2 returning *",
            &[&subscription_end, &user_id],
        )
        .await?;
    let user: User = User::try_from(user)?;
    Ok(user)
}

pub async fn update_user_subscription_check(
    conn: &Object,
    user_id: &str,
    subscription_check: time::OffsetDateTime,
) -> Result<User> {
    let user_id: Uuid = Uuid::from_str(user_id)?;
    // let subscription_check: time::OffsetDateTime =
    //     time::OffsetDateTime::parse(subscription_check, &Iso8601::DEFAULT)?;
    let user: tokio_postgres::Row = conn
        .query_one(
            "update users set subscription_check = $1 where id = $2 returning *",
            &[&subscription_check, &user_id],
        )
        .await?;
    let user: User = User::try_from(user)?;
    Ok(user)
}
