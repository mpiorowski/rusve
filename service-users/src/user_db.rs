use std::str::FromStr;

use anyhow::Result;
use deadpool_postgres::Object;
use time::format_description::well_known::Iso8601;
use tokio_postgres::types::Timestamp;
use uuid::Uuid;

use crate::proto::User;

pub enum StringOrUuid {
    String(String),
    Uuid(Uuid),
}

impl TryFrom<tokio_postgres::Row> for User {
    type Error = anyhow::Error;

    fn try_from(value: tokio_postgres::Row) -> std::result::Result<Self, Self::Error> {
        let id: Uuid = value.try_get("id")?;
        let created: time::OffsetDateTime = value.try_get("created")?;
        let created: String = created.format(&Iso8601::DEFAULT)?.to_string();
        let updated: time::OffsetDateTime = value.try_get("updated")?;
        let updated: String = updated.format(&Iso8601::DEFAULT)?.to_string();
        let deleted: Timestamp<time::OffsetDateTime> = value.try_get("deleted")?;
        let deleted: String = match deleted {
            Timestamp::PosInfinity => "infinity".to_string(),
            Timestamp::NegInfinity => "-infinity".to_string(),
            Timestamp::Value(date) => date.format(&Iso8601::DEFAULT)?.to_string(),
        };

        let email: String = value.try_get("email")?;
        let sub: String = value.try_get("sub")?;
        let role: i32 = value.try_get("role")?;
        let avatar: String = value.try_get("avatar")?;
        let subscription_id: String = value.try_get("subscription_id")?;
        let subscription_end: Timestamp<time::OffsetDateTime> =
            value.try_get("subscription_end")?;
        let mut subscription_active: bool = false;
        let subscription_end: String = match subscription_end {
            Timestamp::PosInfinity => "infinity".to_string(),
            Timestamp::NegInfinity => "-infinity".to_string(),
            Timestamp::Value(date) => {
                if date < time::OffsetDateTime::now_utc() + time::Duration::days(2) {
                    subscription_active = true;
                }
                date.format(&Iso8601::DEFAULT)?.to_string()
            }
        };
        let subscription_check: Timestamp<time::OffsetDateTime> =
            value.try_get("subscription_check")?;
        let subscription_check: String = match subscription_check {
            Timestamp::PosInfinity => "infinity".to_string(),
            Timestamp::NegInfinity => "-infinity".to_string(),
            Timestamp::Value(date) => date.format(&Iso8601::DEFAULT)?.to_string(),
        };

        Ok(User {
            id: id.to_string(),
            created,
            updated,
            deleted,
            email,
            sub,
            role,
            avatar,
            subscription_id,
            subscription_end,
            subscription_check,
            subscription_active,
        })
    }
}

pub async fn select_user_by_id(conn: &Object, user_id: StringOrUuid) -> Result<User> {
    let user_id = match user_id {
        StringOrUuid::String(user_uuid) => Uuid::from_str(&user_uuid)?,
        StringOrUuid::Uuid(user_uuid) => user_uuid,
    };
    let user: tokio_postgres::Row = conn
        .query_one(
            "select * from users where id = $1 and deleted = 'infinity'",
            &[&user_id],
        )
        .await?;
    let user: User = User::try_from(user)?;
    Ok(user)
}

pub async fn create_user(conn: &Object, sub: &str, email: &str, avatar: &str) -> Result<User> {
    let row = conn
        .query_opt(
            "select * from users where sub = $1 and email = $2",
            &[&sub, &email],
        )
        .await?;
    let user = match row {
        Some(_) => {
            conn.query_one(
                "update users set updated = now() where email = $1 and sub = $2 returning *",
                &[&email, &sub],
            )
            .await
        }
        None => {
            let id = Uuid::now_v7();
            let role: i32 = 1;
            conn.query_one(
                "insert into users (id, email, sub, role, avatar) values ($1, $2, $3, $4, $5) returning *",
                &[&id, &email, &sub, &role, &avatar],
            )
            .await
        }
    }?;
    let user = User::try_from(user)?;
    Ok(user)
}
