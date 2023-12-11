use anyhow::Result;
use deadpool_postgres::{Object, Transaction};
use time::format_description::well_known::Iso8601;
use tokio_postgres::RowStream;
use uuid::Uuid;

use crate::proto::{AuthRequest, User, UserRole};

impl TryFrom<tokio_postgres::Row> for User {
    type Error = anyhow::Error;

    fn try_from(value: tokio_postgres::Row) -> std::result::Result<Self, Self::Error> {
        let id: Uuid = value.try_get("id")?;
        let created: time::OffsetDateTime = value.try_get("created")?;
        let updated: time::OffsetDateTime = value.try_get("updated")?;
        let deleted: Option<time::OffsetDateTime> = value.try_get("deleted")?;
        let sub: String = value.try_get("sub")?;
        let email: String = value.try_get("email")?;
        let name: String = value.try_get("name")?;
        let role: String = value.try_get("role")?;
        let payment_id: Option<String> = value.try_get("payment_id")?;
        let avatar_id: Option<Uuid> = value.try_get("avatar_id")?;

        let created: String = created.format(&Iso8601::DEFAULT)?.to_string();
        let updated: String = updated.format(&Iso8601::DEFAULT)?.to_string();
        let deleted: Option<String> = deleted
            .map(|d| d.format(&Iso8601::DEFAULT))
            .transpose()?
            .map(|d| d.to_string());
        let role: i32 = UserRole::from_str_name(&role)
            .ok_or_else(|| anyhow::anyhow!("Invalid role: {}", role))?
            .into();
        let avatar_id: Option<String> = avatar_id.map(|id| id.to_string());

        Ok(User {
            id: id.to_string(),
            created,
            updated,
            deleted,
            sub,
            email,
            name,
            role,
            payment_id,
            avatar_id,
        })
    }
}

pub async fn auth_user(conn: &Object, request: AuthRequest) -> Result<User> {
    conn.execute(
            "insert into users (sub, email, role) values ($1, $2, $3) on conflict (sub) do update set updated = now()",
            &[&request.sub, &request.email, &UserRole::RoleUser.as_str_name()],
        ).await?;

    let user = conn
        .query_one(
            "select * from users where email = $1 and sub = $2 and deleted is null",
            &[&request.email, &request.sub],
        )
        .await?;

    let user: User = User::try_from(user)?;
    Ok(user)
}

pub async fn get_users(conn: &Object, user_ids: Vec<Uuid>) -> Result<RowStream> {
    let users = conn
        .query_raw(
            "select * from users where id = any($1) and deleted is null",
            &[&user_ids],
        )
        .await?;

    Ok(users)
}

pub async fn get_user(conn: &Object, user_id: &Uuid) -> Result<User> {
    let user = conn
        .query_one(
            "select * from users where id = $1 and deleted is null",
            &[&user_id],
        )
        .await?;

    let user: User = User::try_from(user)?;
    Ok(user)
}

pub async fn update_user(
    conn: &Object,
    user_id: &Uuid,
    name: &str,
    avatar_id: &Option<Uuid>,
) -> Result<User> {
    let user = conn
        .query_one(
            "update users set name = $1, avatar_id = $2 where id = $3 returning *",
            &[&name, &avatar_id, &user_id],
        )
        .await?;

    let user: User = User::try_from(user)?;
    Ok(user)
}

pub async fn update_payment_id(
    tr: &Transaction<'_>,
    user_id: &Uuid,
    payment_id: &str,
) -> Result<User> {
    let user = tr
        .query_one(
            "update users set payment_id = $1 where id = $2 returning *",
            &[&payment_id, &user_id],
        )
        .await?;

    let user: User = User::try_from(user)?;
    Ok(user)
}
