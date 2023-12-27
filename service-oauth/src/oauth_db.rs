use anyhow::Result;
use deadpool_postgres::{GenericClient, Object};
use time::OffsetDateTime;
use uuid::Uuid;

pub struct Pkce {
    pub id: Uuid,
    pub created: OffsetDateTime,
    pub csrf_token: String,
    pub pkce_verifier: String,
}

impl TryFrom<tokio_postgres::Row> for Pkce {
    type Error = anyhow::Error;

    fn try_from(value: tokio_postgres::Row) -> std::result::Result<Self, Self::Error> {
        let id: Uuid = value.try_get("id")?;
        let created: OffsetDateTime = value.try_get("created")?;
        let csrf_token: String = value.try_get("csrf_token")?;
        let pkce_verifier: String = value.try_get("pkce_verifier")?;

        Ok(Pkce {
            id,
            created,
            csrf_token,
            pkce_verifier,
        })
    }
}

pub struct Token {
    pub id: Uuid,
    pub created: OffsetDateTime,
    pub access_token: String,
    pub refresh_token: String,
}

impl TryFrom<tokio_postgres::Row> for Token {
    type Error = anyhow::Error;

    fn try_from(value: tokio_postgres::Row) -> std::result::Result<Self, Self::Error> {
        let id: Uuid = value.try_get("id")?;
        let created: OffsetDateTime = value.try_get("created")?;
        let access_token: String = value.try_get("access_token")?;
        let refresh_token: String = value.try_get("refresh_token")?;

        Ok(Token {
            id,
            created,
            access_token,
            refresh_token,
        })
    }
}

#[derive(Debug)]
pub struct User {
    pub id: String,
    pub email: String,
    pub sub: String,
    pub role: i32,
}

impl TryFrom<tokio_postgres::Row> for User {
    type Error = anyhow::Error;

    fn try_from(value: tokio_postgres::Row) -> std::result::Result<Self, Self::Error> {
        let id: Uuid = value.try_get("id")?;
        let email: String = value.try_get("email")?;
        let sub: String = value.try_get("sub")?;
        let role: i32 = value.try_get("role")?;

        Ok(User {
            id: id.to_string(),
            email,
            sub,
            role,
        })
    }
}

pub async fn select_pkce_by_csrf(client: &Object, csrf_token: &str) -> Result<Option<Pkce>> {
    let row = client
        .query_opt("select * from pkce where csrf_token = $1", &[&csrf_token])
        .await?;
    match row {
        Some(row) => Ok(Some(Pkce::try_from(row)?)),
        None => Ok(None),
    }
}

pub async fn create_pkce(client: &Object, csrf_token: &str, pkce_verifier: &str) -> Result<Pkce> {
    let uuid = Uuid::now_v7();
    let row = client
        .query_one(
            "insert into pkce (id, csrf_token, pkce_verifier) values ($1, $2, $3) returning *",
            &[&uuid, &csrf_token, &pkce_verifier],
        )
        .await?;
    Pkce::try_from(row)
}

// 10 minutes
pub async fn delete_old_pkces(client: &Object) -> Result<()> {
    client
        .execute(
            "delete from pkce where created < now() - interval '10 minutes'",
            &[],
        )
        .await?;
    Ok(())
}

pub async fn create_token(
    client: &Object,
    user_id: &str,
    access_token: &str,
    refresh_token: &str,
) -> Result<Token> {
    let id = Uuid::now_v7();
    let user_id = Uuid::parse_str(user_id)?;
    let row = client
        .query_one(
            "insert into tokens (id, user_id, access_token, refresh_token) values ($1, $2, $3, $4) returning *",
            &[&id, &user_id, &access_token, &refresh_token],
        )
        .await?;
    Token::try_from(row)
}

// 7 days
pub async fn delete_old_tokens(client: &Object) -> Result<()> {
    client
        .execute(
            "delete from tokens where created < now() - interval '7 days'",
            &[],
        )
        .await?;
    Ok(())
}

pub async fn auth_user(conn: &Object, sub: &str, email: &str) -> Result<User> {
    let row = conn
        .query_opt(
            "select * from users where sub = $1 or email = $2",
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
                "insert into users (id, email, sub, role) values ($1, $2, $3, $4) returning *",
                &[&id, &email, &sub, &role],
            )
            .await
        }
    }?;
    let user = User::try_from(user)?;
    Ok(user)
}
