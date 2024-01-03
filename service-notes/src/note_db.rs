use anyhow::Result;
use deadpool_postgres::Object;
use time::format_description::well_known::Iso8601;
use tokio_postgres::{types::Timestamp, RowStream};
use uuid::Uuid;

use crate::proto::Note;

fn slice_iter<'a>(
    s: &'a [&'a (dyn tokio_postgres::types::ToSql + Sync)],
) -> impl ExactSizeIterator<Item = &'a dyn tokio_postgres::types::ToSql> + 'a {
    s.iter().map(|s| *s as _)
}

impl TryFrom<tokio_postgres::Row> for Note {
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
        let user_id: Uuid = value.try_get("user_id")?;
        let title: String = value.try_get("title")?;
        let content: String = value.try_get("content")?;

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

pub async fn count_notes_by_user_id(conn: &Object, user_id: &str) -> Result<i64> {
    let stmt = conn
        .prepare("select count(*) from notes where user_id = $1 and deleted = 'infinity'")
        .await?;
    let row = conn.query_one(&stmt, &[&Uuid::parse_str(user_id)?]).await?;
    let count: i64 = row.try_get(0)?;
    Ok(count)
}

pub async fn get_notes_by_user_id(
    conn: &Object,
    user_id: &str,
    offset: i64,
    limit: i64,
) -> Result<RowStream> {
    let stmt = conn
        .prepare("select * from notes where user_id = $1 and deleted = 'infinity' order by created desc offset $2 limit $3")
        .await?;

    let rows = conn
        .query_raw(
            &stmt,
            slice_iter(&[&Uuid::parse_str(user_id)?, &offset, &limit]),
        )
        .await?;
    Ok(rows)
}

pub async fn get_note_by_id(conn: &Object, id: &str, user_id: &str) -> Result<Note> {
    let id = Uuid::parse_str(id)?;
    let user_id = Uuid::parse_str(user_id)?;
    let res = conn
        .query_one(
            "select * from notes where id = $1 and user_id = $2 and deleted = 'infinity'",
            &[&id, &user_id],
        )
        .await?;
    let note = Note::try_from(res)?;
    Ok(note)
}

pub async fn insert_note(conn: &Object, user_id: &str, note: &Note) -> Result<Note> {
    let user_id = Uuid::parse_str(user_id)?;
    let res = conn
        .query_one(
            "insert into notes (id, user_id, title, content) values ($1, $2, $3, $4) returning *",
            &[&Uuid::now_v7(), &user_id, &note.title, &note.content],
        )
        .await?;
    let note = Note::try_from(res)?;
    Ok(note)
}

pub async fn update_note(conn: &Object, user_id: &str, note: &Note) -> Result<Note> {
    let id = Uuid::parse_str(&note.id)?;
    let user_id = Uuid::parse_str(user_id)?;
    let res = conn
        .query_one(
            "update notes set title = $1, content = $2 where id = $3 and user_id = $4 returning *",
            &[&note.title, &note.content, &id, &user_id],
        )
        .await?;
    let note = Note::try_from(res)?;
    Ok(note)
}

pub async fn delete_note_by_id(conn: &Object, id: &str, user_id: &str) -> Result<Note> {
    let id = Uuid::parse_str(id)?;
    let user_id = Uuid::parse_str(user_id)?;
    let res = conn
        .query_one(
            "update notes set deleted = now() where id = $1 and user_id = $2 returning *",
            &[&id, &user_id],
        )
        .await?;
    let note = Note::try_from(res)?;
    Ok(note)
}
