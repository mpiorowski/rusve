use crate::proto::Email;
use anyhow::Result;
use deadpool_postgres::Object;
use time::format_description::well_known::Iso8601;
use tokio_postgres::{types::Timestamp, RowStream};
use uuid::Uuid;

impl TryFrom<tokio_postgres::Row> for Email {
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

        let target_id: Uuid = value.try_get("target_id")?;
        let email_to: String = value.try_get("email_to")?;
        let email_from: String = value.try_get("email_from")?;
        let email_from_name: String = value.try_get("email_from_name")?;
        let email_subject: String = value.try_get("email_subject")?;
        let email_body: String = value.try_get("email_body")?;

        Ok(Email {
            id: id.to_string(),
            created,
            updated,
            deleted,
            target_id: target_id.to_string(),
            email_to,
            email_from,
            email_from_name,
            email_subject,
            email_body,
        })
    }
}

fn slice_iter<'a>(
    s: &'a [&'a (dyn tokio_postgres::types::ToSql + Sync)],
) -> impl ExactSizeIterator<Item = &'a dyn tokio_postgres::types::ToSql> + 'a {
    s.iter().map(|s| *s as _)
}

pub async fn count_emails_by_target_id(conn: &Object, target_id: &str) -> Result<i64> {
    let target_id = Uuid::parse_str(target_id)?;
    let stmt = conn
        .prepare("select count(*) from emails where target_id = $1 and deleted = 'infinity'")
        .await?;
    let count = conn.query_one(&stmt, &[&target_id]).await?;

    Ok(count.get(0))
}

pub async fn get_emails_by_target_id(
    conn: &Object,
    target_id: &str,
    offset: i64,
    limit: i64,
) -> Result<RowStream> {
    let stmt = conn
        .prepare(
            "select * from emails where target_id = $1 and deleted = 'infinity' order by created desc offset $2 limit $3",
        )
        .await?;

    let rows = conn
        .query_raw(
            &stmt,
            slice_iter(&[&Uuid::parse_str(target_id)?, &offset, &limit]),
        )
        .await?;
    Ok(rows)
}

pub async fn insert_email(
    conn: &deadpool_postgres::Transaction<'_>,
    target_id: &str,
    email: &Email,
) -> Result<Email> {
    let id = Uuid::now_v7();
    let target_id = Uuid::parse_str(target_id)?;
    let email = conn
        .query_one(
            "insert into emails (id, target_id, email_to, email_from, email_from_name, email_subject, email_body) values ($1, $2, $3, $4, $5, $6, $7) returning *",
            &[&id, &target_id, &email.email_to, &email.email_from, &email.email_from_name, &email.email_subject, &email.email_body],
        )
        .await?;

    email.try_into()
}
