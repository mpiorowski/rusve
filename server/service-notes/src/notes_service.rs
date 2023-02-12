use anyhow::Context;
use anyhow::Result;
use futures::TryStreamExt;
use sqlx::types::time::OffsetDateTime;
use sqlx::{postgres::PgRow, query, types::Uuid, Row};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};

use crate::{
    proto::{notes_service_server::NotesService, Note, NoteId, UserId},
    MyNotes,
};

fn map_note(row: Option<PgRow>) -> Result<Note> {
    match row {
        Some(row) => {
            let id: Uuid = row.try_get("id").context("Failed to get id")?;
            let user_id: Uuid = row.try_get("userId").context("Failed to get userId")?;
            let created: OffsetDateTime =
                row.try_get("created").context("Failed to get created")?;
            let updated: OffsetDateTime =
                row.try_get("updated").context("Failed to get updated")?;
            let deleted: Option<OffsetDateTime> =
                row.try_get("deleted").context("Failed to get deleted")?;
            let note = Note {
                id: id.to_string(),
                user_id: user_id.to_string(),
                title: row.try_get("title").context("Failed to get title")?,
                content: row.try_get("content").context("Failed to get content")?,
                created: created.to_string(),
                updated: updated.to_string(),
                deleted: deleted.map(|d| d.to_string()),
                user: None,
            };
            Ok(note)
        }
        None => Err(anyhow::anyhow!("Note not found")),
    }
}

#[tonic::async_trait]
impl NotesService for MyNotes {
    type GetNotesStream = ReceiverStream<Result<Note, Status>>;

    async fn get_notes(
        &self,
        request: Request<UserId>,
    ) -> Result<Response<Self::GetNotesStream>, Status> {
        println!("GetNotes = {:?}", request);
        let start = std::time::Instant::now();

        let self_pool = self.pool.clone();
        let (tx, rx) = mpsc::channel(4);
        let user_id = request.into_inner().user_id;
        let uuid = Uuid::parse_str(&user_id).map_err(|e| Status::internal(e.to_string()))?;

        tokio::spawn(async move {
            let mut notes_stream = query("SELECT * FROM notes WHERE \"userId\" = $1 and deleted is null order by created desc")
                .bind(uuid)
                .fetch(&self_pool);

            loop {
                match notes_stream.try_next().await {
                    Ok(None) => {
                        let elapsed = start.elapsed();
                        println!("Elapsed: {:.2?}", elapsed);
                        break;
                    }
                    Ok(note) => {
                        let note = map_note(note).unwrap();
                        tx.send(Ok(note)).await.unwrap();
                    }
                    Err(e) => {
                        tx.send(Err(Status::internal(e.to_string()))).await.unwrap();
                        break;
                    }
                }
            }
        });
        Ok(Response::new(ReceiverStream::new(rx)))
    }

    async fn create_note(&self, request: Request<Note>) -> Result<Response<Note>, Status> {
        println!("CreateNote = {:?}", request);
        let start = std::time::Instant::now();

        let note = request.into_inner();
        let user_id =
            Uuid::parse_str(&note.user_id).map_err(|e| Status::internal(e.to_string()))?;
        let self_pool = self.pool.clone();

        let row =
            query("INSERT INTO notes (title, content, \"userId\") VALUES ($1, $2, $3) RETURNING *")
                .bind(note.title)
                .bind(note.content)
                .bind(user_id)
                .fetch_one(&self_pool)
                .await
                .map_err(|e| Status::internal(e.to_string()))?;

        let note = map_note(Some(row)).map_err(|e| Status::internal(e.to_string()))?;
        let elapsed = start.elapsed();
        println!("Elapsed: {:.2?}", elapsed);
        return Ok(Response::new(note));
    }
    async fn delete_note(&self, request: Request<NoteId>) -> Result<Response<Note>, Status> {
        println!("DeleteNote = {:?}", request);
        let start = std::time::Instant::now();

        let self_pool = self.pool.clone();

        let request = request.into_inner();
        let note_uuid =
            Uuid::parse_str(&request.note_id).map_err(|e| Status::internal(e.to_string()))?;
        let user_uuid =
            Uuid::parse_str(&request.user_id).map_err(|e| Status::internal(e.to_string()))?;

        let row =
            query("UPDATE notes SET deleted = NOW() WHERE id = $1 AND \"userId\" = $2 RETURNING *")
                .bind(note_uuid)
                .bind(user_uuid)
                .fetch_one(&self_pool)
                .await
                .map_err(|e| Status::not_found(e.to_string()))?;

        let note = map_note(Some(row)).map_err(|e| Status::internal(e.to_string()))?;
        let elapsed = start.elapsed();
        println!("Elapsed: {:.2?}", elapsed);
        return Ok(Response::new(note));
    }
}
