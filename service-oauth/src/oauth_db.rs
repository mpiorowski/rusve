use anyhow::Result;
use time::{format_description::well_known::Iso8601, OffsetDateTime};
use uuid::Uuid;

pub struct Pkce {
    pub id: Uuid,
    pub created: OffsetDateTime,
    pub created_str: String,
    pub csrf_token: String,
    pub pkce_verifier: String,
}

impl TryFrom<tokio_postgres::Row> for Pkce {
    type Error = anyhow::Error;

    fn try_from(value: tokio_postgres::Row) -> std::result::Result<Self, Self::Error> {
        let id: Uuid = value.try_get("id")?;
        let created: OffsetDateTime = value.try_get("created")?;
        let created_str: String = created.format(&Iso8601::DEFAULT)?.to_string();
        let csrf_token: String = value.try_get("csrf_token")?;
        let pkce_verifier: String = value.try_get("pkce_verifier")?;

        Ok(Pkce {
            id,
            created,
            created_str,
            csrf_token,
            pkce_verifier,
        })
    }
}

pub struct Token {
    pub id: Uuid,
    pub created: OffsetDateTime,
    pub created_str: String,
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: i64,
}

impl TryFrom<tokio_postgres::Row> for Token {
    type Error = anyhow::Error;

    fn try_from(value: tokio_postgres::Row) -> std::result::Result<Self, Self::Error> {
        let id: Uuid = value.try_get("id")?;
        let created: OffsetDateTime = value.try_get("created")?;
        let created_str: String = created.format(&Iso8601::DEFAULT)?.to_string();
        let access_token: String = value.try_get("access_token")?;
        let refresh_token: String = value.try_get("refresh_token")?;
        let expires_in: i64 = value.try_get("expires_in")?;

        Ok(Token {
            id,
            created,
            created_str,
            access_token,
            refresh_token,
            expires_in,
        })
    }
}

pub async fn select_pkce_by_csrf(
    pool: &deadpool_postgres::Pool,
    csrf_token: &str,
) -> Result<Option<Pkce>> {
    let client = pool.get().await?;
    let row = client
        .query_opt("select * from pkce where csrf_token = $1", &[&csrf_token])
        .await?;
    match row {
        Some(row) => Ok(Some(Pkce::try_from(row)?)),
        None => Ok(None),
    }
}

pub async fn create_pkce(
    pool: &deadpool_postgres::Pool,
    csrf_token: &str,
    pkce_verifier: &str,
) -> Result<Pkce> {
    let client = pool.get().await?;
    let uuid = Uuid::now_v7();
    let row = client
        .query_one(
            "insert into pkce (id, csrf_token, pkce_verifier) values ($1, $2, $3) returning *",
            &[&uuid, &csrf_token, &pkce_verifier],
        )
        .await?;
    Ok(Pkce::try_from(row)?)
}

pub async fn delete_pkce_by_id(pool: &deadpool_postgres::Pool, id: Uuid) -> Result<()> {
    let client = pool.get().await?;
    client
        .execute("delete from pkce where id = $1", &[&id])
        .await?;
    Ok(())
}

pub async fn select_token_by_id(pool: &deadpool_postgres::Pool, id: Uuid) -> Result<Option<Token>> {
    let client = pool.get().await?;
    let row = client
        .query_opt("select * from tokens where id = $1", &[&id])
        .await?;
    match row {
        Some(row) => Ok(Some(Token::try_from(row)?)),
        None => Ok(None),
    }
}

pub async fn create_token(
    pool: &deadpool_postgres::Pool,
    user_id: Uuid,
    access_token: &str,
    refresh_token: &str,
    expires_in: i64,
) -> Result<Token> {
    let client = pool.get().await?;
    let uuid = Uuid::now_v7();
    let row = client
        .query_one(
            "insert into tokens (id, user_id, access_token, refresh_token, expires_in) values ($1, $2, $3, $4, $5) returning *",
            &[&uuid, &user_id, &access_token, &refresh_token, &expires_in],
        )
        .await?;
    Ok(Token::try_from(row)?)
}

pub async fn delete_token_by_user_id(pool: &deadpool_postgres::Pool, user_id: &str) -> Result<()> {
    let client = pool.get().await?;
    client
        .execute("delete from tokens where user_id = $1", &[&user_id])
        .await?;
    Ok(())
}
