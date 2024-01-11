use anyhow::Result;
use deadpool_postgres::Object;
use std::str::FromStr;
use uuid::Uuid;

pub struct Token {
    pub id: Uuid,
    pub created: time::OffsetDateTime,
    pub updated: time::OffsetDateTime,
    pub user_id: Uuid,
}

impl TryFrom<tokio_postgres::Row> for Token {
    type Error = anyhow::Error;
    fn try_from(value: tokio_postgres::Row) -> std::result::Result<Self, Self::Error> {
        Ok(Token {
            id: value.try_get("id")?,
            created: value.try_get("created")?,
            updated: value.try_get("updated")?,
            user_id: value.try_get("user_id")?,
        })
    }
}

pub async fn select_token_by_id(conn: &Object, token_id: &str) -> Result<Token> {
    let token: tokio_postgres::Row = conn
        .query_one(
            "select * from tokens where id = $1",
            &[&Uuid::from_str(token_id)?],
        )
        .await?;
    let token: Token = Token::try_from(token)?;
    Ok(token)
}

pub async fn insert_token(conn: &Object, user_id: &str) -> Result<Uuid> {
    let id: Uuid = Uuid::now_v7();
    let user_id = Uuid::from_str(user_id)?;
    conn.execute(
        "insert into tokens (id, user_id) values ($1, $2)",
        &[&id, &user_id],
    )
    .await?;
    Ok(id)
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
