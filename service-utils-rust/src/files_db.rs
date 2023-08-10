use anyhow::Result;
use deadpool_postgres::{Object, Transaction};
use time::format_description::well_known::Iso8601;
use tokio_postgres::RowStream;
use uuid::Uuid;

use crate::proto::{File, FileType};

impl TryFrom<tokio_postgres::Row> for File {
    type Error = anyhow::Error;

    fn try_from(value: tokio_postgres::Row) -> std::result::Result<Self, Self::Error> {
        let id: Uuid = value.try_get("id")?;
        let created: time::OffsetDateTime = value.try_get("created")?;
        let updated: time::OffsetDateTime = value.try_get("updated")?;
        let deleted: Option<time::OffsetDateTime> = value.try_get("deleted")?;
        let target_id: Uuid = value.try_get("target_id")?;
        let name: String = value.try_get("name")?;
        let r#type: String = value.try_get("type")?;

        let created: String = created.format(&Iso8601::DEFAULT)?.to_string();
        let updated: String = updated.format(&Iso8601::DEFAULT)?.to_string();
        let deleted: Option<String> = deleted
            .map(|d| d.format(&Iso8601::DEFAULT))
            .transpose()?
            .map(|d| d.to_string());
        let r#type = FileType::from_str_name(&r#type)
            .ok_or_else(|| anyhow::anyhow!("Invalid file type: {}", r#type))?;
        let r#type: i32 = r#type.into();

        Ok(File {
            id: id.to_string(),
            created,
            updated,
            deleted,
            target_id: target_id.to_string(),
            name,
            r#type,
            buffer: Vec::new(),
            url: String::new(),
        })
    }
}

pub async fn get_files_by_target_id(
    conn: &Object,
    target_id: &Uuid,
) -> Result<RowStream> {
    let stmt = conn
        .prepare(
            "select * from files where target_id = $1 and deleted is null order by created desc",
        )
        .await?;
    let files = conn.query_raw(&stmt, &[&target_id]).await?;

    Ok(files)
}

pub async fn get_file_by_id(conn: &Object, file_id: &Uuid, target_id: &Uuid) -> Result<File> {
    let file = conn
        .query_one(
            "select * from files where id = $1 and target_id = $2 and deleted is null",
            &[&file_id, &target_id],
        )
        .await?;

    Ok(file.try_into()?)
}

pub async fn create_file(conn: &Transaction<'_>, file: &File) -> Result<File> {
    let target_id = Uuid::parse_str(&file.target_id)?;
    let file = conn
        .query_one(
            "insert into files (target_id, name, type) values ($1, $2, $3) returning *",
            &[&target_id, &file.name, &file.r#type],
        )
        .await?;

    Ok(file.try_into()?)
}

pub async fn delete_file(conn: &Transaction<'_>, file_id: &Uuid, target_id: &Uuid) -> Result<File> {
    let file = conn
        .query_one(
            "update files set deleted = now() where id = $1 and target_id = $2 returning *",
            &[&file_id, &target_id],
        )
        .await?;

    Ok(file.try_into()?)
}
