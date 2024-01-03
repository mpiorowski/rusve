use anyhow::Result;
use deadpool_postgres::{Object, Transaction};
use time::format_description::well_known::Iso8601;
use tokio_postgres::{types::Timestamp, RowStream};
use uuid::Uuid;

use crate::proto::{File, FileType};

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
        let file_type: i32 = value.try_get("file_type")?;

        Ok(File {
            id: id.to_string(),
            created,
            updated,
            deleted,
            target_id: target_id.to_string(),
            file_name,
            file_type,
            file_buffer: Vec::new(),
            file_url: String::new(),
        })
    }
}

pub async fn get_files_by_target_id(conn: &Object, target_id: &str) -> Result<RowStream> {
    let target_id = Uuid::parse_str(target_id)?;
    let stmt = conn
        .prepare(
            "select * from files where target_id = $1 and deleted is null order by created desc",
        )
        .await?;
    let files = conn.query_raw(&stmt, &[&target_id]).await?;

    Ok(files)
}

pub async fn get_file_by_id(conn: &Object, file_id: &str) -> Result<File> {
    let file_id = Uuid::parse_str(file_id)?;
    let file = conn
        .query_one(
            "select * from files where id = $1 and deleted is null",
            &[&file_id],
        )
        .await?;

    file.try_into()
}

pub async fn insert_file(conn: &Transaction<'_>, file: &File) -> Result<File> {
    let target_id = Uuid::parse_str(&file.target_id)?;
    let file = conn
        .query_one(
            "insert into files (target_id, file_name, file_type) values ($1, $2, $3) returning *",
            &[&target_id, &file.file_name, &file.file_type],
        )
        .await?;

    file.try_into()
}

pub async fn delete_file(conn: &Transaction<'_>, file_id: &str) -> Result<File> {
    let file_id = Uuid::parse_str(file_id)?;
    let file = conn
        .query_one(
            "update files set deleted = now() where id = $1 returning *",
            &[&file_id],
        )
        .await?;

    file.try_into()
}
