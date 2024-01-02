use anyhow::Result;
use deadpool_postgres::Object;
use std::str::FromStr;
use uuid::Uuid;

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

pub async fn create_token(client: &Object, user_id: &str) -> Result<Token> {
    let id = Uuid::now_v7();
    let user_id = Uuid::parse_str(user_id)?;
    let row = client
        .query_one(
            "insert into tokens (id, user_id) values ($1, $2) returning *",
            &[&id, &user_id],
        )
        .await?;
    Token::try_from(row)
}

pub async fn update_token_id(conn: &Object, old_id: &Uuid) -> Result<Uuid> {
    let new_id: Uuid = Uuid::now_v7();
    conn.execute(
        "update tokens set id = $1, created = now() where id = $2",
        &[&new_id, &old_id],
    )
    .await?;
    Ok(new_id)
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
