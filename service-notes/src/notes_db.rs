use anyhow::Result;
use deadpool_postgres::Object;
use time::format_description::well_known::Iso8601;
use tokio_postgres::RowStream;
use uuid::Uuid;

use crate::proto::Note;

impl TryFrom<tokio_postgres::Row> for Note {
    type Error = anyhow::Error;

    fn try_from(value: tokio_postgres::Row) -> std::result::Result<Self, Self::Error> {
        let id: Uuid = value.try_get("id")?;
        let created: time::OffsetDateTime = value.try_get("created")?;
        let updated: time::OffsetDateTime = value.try_get("updated")?;
        let deleted: Option<time::OffsetDateTime> = value.try_get("deleted")?;
        let user_id: Uuid = value.try_get("user_id")?;
        let title: String = value.try_get("title")?;
        let content: String = value.try_get("content")?;

        let created: String = created.format(&Iso8601::DEFAULT)?.to_string();
        let updated: String = updated.format(&Iso8601::DEFAULT)?.to_string();
        let deleted: Option<String> = deleted
            .map(|d| d.format(&Iso8601::DEFAULT))
            .transpose()?
            .map(|d| d.to_string());

        Ok(Note {
            id: id.to_string(),
            created,
            updated,
            deleted,
            user_id: user_id.to_string(),
            title,
            content,
        })
    }
}

pub async fn get_notes(conn: &Object, user_id: &Uuid) -> Result<RowStream> {
    let stmt = conn
        .prepare("select * from notes where user_id = $1 and deleted is null")
        .await?;
    let rows = conn.query_raw(&stmt, &[&user_id]).await?;
    Ok(rows)
}

pub async fn create_note(conn: &Object, note: &Note) -> Result<Note> {
    let res: tokio_postgres::Row;
    let user_id: Uuid = note.user_id.parse()?;
    if note.id.is_empty() {
        res = conn
            .query_one(
                "insert into notes (user_id, title, content) values ($1, $2, $3) returning *",
                &[&user_id, &note.title, &note.content],
            )
            .await?;
    } else {
        let id: Uuid = note.id.parse()?;
        res = conn
            .query_one(
                "update notes set title = $1, content = $2 where id = $3 and user_id = $4 returning *",
                &[&note.title, &note.content, &id, &user_id],
            )
            .await?;
    }
    let note = Note::try_from(res)?;
    Ok(note)
}

pub async fn delete_note(conn: &Object, id: &Uuid) -> Result<Note> {
    let res = conn
        .query_one(
            "update notes set deleted = now() where id = $1 returning *",
            &[&id],
        )
        .await?;
    let note = Note::try_from(res)?;
    Ok(note)
}
