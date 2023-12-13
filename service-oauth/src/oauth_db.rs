use anyhow::Result;
use time::format_description::well_known::Iso8601;
use uuid::Uuid;

pub struct Pkce {
    id: String,
    created: String,
    csrf_token: String,
    pub pkce_verifier: String,
}

impl TryFrom<tokio_postgres::Row> for Pkce {
    type Error = anyhow::Error;

    fn try_from(value: tokio_postgres::Row) -> std::result::Result<Self, Self::Error> {
        let id: Uuid = value.try_get("id")?;
        let created: time::OffsetDateTime = value.try_get("created")?;
        let created: String = created.format(&Iso8601::DEFAULT)?.to_string();
        let csrf_token: String = value.try_get("csrf_token")?;
        let pkce_verifier: String = value.try_get("pkce_verifier")?;

        Ok(Pkce {
            id: id.to_string(),
            created,
            csrf_token,
            pkce_verifier,
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
