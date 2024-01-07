use anyhow::Result;
use deadpool_postgres::{Object, Transaction};
use time::format_description::well_known::Iso8601;
use tokio_postgres::{types::Timestamp, RowStream};
use uuid::Uuid;

use crate::proto::File;

impl TryFrom<tokio_postgres::Row> for File {
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
        let file_name: String = value.try_get("file_name")?;
        let file_size: String = value.try_get("file_size")?;
        let file_type: String = value.try_get("file_type")?;
        let file_target: i32 = value.try_get("file_target")?;

        Ok(File {
            id: id.to_string(),
            created,
            updated,
            deleted,
            target_id: target_id.to_string(),
            file_name,
            file_size,
            file_type,
            file_target,
            file_buffer: Vec::new(),
        })
    }
}

fn slice_iter<'a>(
    s: &'a [&'a (dyn tokio_postgres::types::ToSql + Sync)],
) -> impl ExactSizeIterator<Item = &'a dyn tokio_postgres::types::ToSql> + 'a {
    s.iter().map(|s| *s as _)
}

pub async fn count_files_by_target_id(conn: &Object, target_id: &str) -> Result<i64> {
    let target_id = Uuid::parse_str(target_id)?;
    let count: i64 = conn
        .query_one(
            "select count(*) from files where target_id = $1 and deleted = 'infinity'",
            &[&target_id],
        )
        .await?
        .try_get(0)?;

    Ok(count)
}

pub async fn get_files_by_target_id(
    conn: &Object,
    target_id: &str,
    offset: i64,
    limit: i64,
) -> Result<RowStream> {
    let stmt = conn
        .prepare(
            "select * from files where target_id = $1 and deleted = 'infinity' order by created desc offset $2 limit $3",
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

pub async fn get_file_by_id(conn: &Object, id: &str, target_id: &str) -> Result<File> {
    let id = Uuid::parse_str(id)?;
    let target_id = Uuid::parse_str(target_id)?;
    let file = conn
        .query_one(
            "select * from files where id = $1 and target_id = $2 and deleted = 'infinity'",
            &[&id, &target_id],
        )
        .await?;

    file.try_into()
}

pub async fn insert_file(conn: &Transaction<'_>, file: &File, target_id: &str) -> Result<File> {
    let id = Uuid::now_v7();
    let target_id = Uuid::parse_str(target_id)?;
    let file = conn
        .query_one(
            "insert into files (id, target_id, file_name, file_size, file_type, file_target) values ($1, $2, $3, $4, $5, $6) returning *",
            &[&id, &target_id, &file.file_name, &file.file_size, &file.file_type, &file.file_target],
        )
        .await?;

    file.try_into()
}

pub async fn delete_file_by_id(conn: &Transaction<'_>, id: &str, target_id: &str) -> Result<File> {
    let id = Uuid::parse_str(id)?;
    let target_id = Uuid::parse_str(target_id)?;
    let file = conn
        .query_one(
            "update files set deleted = now() where id = $1 and target_id = $2 returning *",
            &[&id, &target_id],
        )
        .await?;

    file.try_into()
}
