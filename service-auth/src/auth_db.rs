use std::str::FromStr;

use anyhow::Result;
use deadpool_postgres::{GenericClient, Object};
use time::OffsetDateTime;
use uuid::Uuid;

pub struct Verifier {
    pub id: Uuid,
    pub created: OffsetDateTime,
    pub csrf_token: String,
    pub pkce_verifier: String,
}

impl TryFrom<tokio_postgres::Row> for Verifier {
    type Error = anyhow::Error;

    fn try_from(value: tokio_postgres::Row) -> std::result::Result<Self, Self::Error> {
        let id: Uuid = value.try_get("id")?;
        let created: OffsetDateTime = value.try_get("created")?;
        let csrf_token: String = value.try_get("csrf_token")?;
        let pkce_verifier: String = value.try_get("pkce_verifier")?;

        Ok(Verifier {
            id,
            created,
            csrf_token,
            pkce_verifier,
        })
    }
}

pub struct Token {
    pub id: Uuid,
    pub created: time::OffsetDateTime,
    pub user_id: Uuid,
}

impl TryFrom<tokio_postgres::Row> for Token {
    type Error = anyhow::Error;
    fn try_from(value: tokio_postgres::Row) -> std::result::Result<Self, Self::Error> {
        Ok(Token {
            id: value.try_get("id")?,
            created: value.try_get("created")?,
            user_id: value.try_get("user_id")?,
        })
    }
}

pub async fn select_verifiers_by_csrf(
    client: &Object,
    csrf_token: &str,
) -> Result<Option<Verifier>> {
    let row = client
        .query_opt(
            "select * from verifiers where csrf_token = $1",
            &[&csrf_token],
        )
        .await?;
    match row {
        Some(row) => Ok(Some(Verifier::try_from(row)?)),
        None => Ok(None),
    }
}

pub async fn create_verifiers(
    client: &Object,
    csrf_token: &str,
    pkce_verifier: &str,
) -> Result<Verifier> {
    let uuid = Uuid::now_v7();
    let row = client
        .query_one(
            "insert into verifiers (id, csrf_token, pkce_verifier) values ($1, $2, $3) returning *",
            &[&uuid, &csrf_token, &pkce_verifier],
        )
        .await?;
    Verifier::try_from(row)
}

// 5 minutes
pub async fn delete_old_verifiers(client: &Object) -> Result<()> {
    client
        .execute(
            "delete from verifiers where created < now() - interval '5 minutes'",
            &[],
        )
        .await?;
    Ok(())
}

pub async fn insert_token(client: &Object) -> Result<Token> {
    let id = Uuid::now_v7();
    let user_id = Uuid::from_str("00000000-0000-0000-0000-000000000000")?;
    let row = client
        .query_one(
            "insert into tokens (id, user_id) values ($1, $2) returning *",
            &[&id, &user_id],
        )
        .await?;
    Token::try_from(row)
}
