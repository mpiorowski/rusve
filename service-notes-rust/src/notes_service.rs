use crate::{
    proto::{notes_service_server::NotesService, Note, NoteId, UserId},
    MyService,
};
use anyhow::Result;
use sqlx::{types::Uuid, Row};
use time::OffsetDateTime;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};

trait SqlxError {
    fn into_status(self) -> Status;
}

impl SqlxError for sqlx::Error {
    fn into_status(self) -> Status {
        match self {
            sqlx::Error::Database(e) => Status::internal(e.message()),
            sqlx::Error::RowNotFound => Status::not_found("Note not found"),
            sqlx::Error::ColumnNotFound(_) => Status::not_found("Note not found"),
            _ => Status::internal("Unknown error"),
        }
    }
}

struct PgNote {
    id: Uuid,
    user_id: Uuid,
    created: OffsetDateTime,
    updated: OffsetDateTime,
    deleted: Option<OffsetDateTime>,
    title: String,
    content: String,
}

impl TryFrom<Option<sqlx::postgres::PgRow>> for Note {
    type Error = anyhow::Error;

    fn try_from(row: Option<sqlx::postgres::PgRow>) -> Result<Self, Self::Error> {
        match row {
            Some(row) => {
                let pg_note = PgNote {
                    id: row.try_get("id")?,
                    user_id: row.try_get("user_id")?,
                    created: row.try_get("created")?,
                    updated: row.try_get("updated")?,
                    deleted: row.try_get("deleted")?,
                    title: row.try_get("title")?,
                    content: row.try_get("content")?,
                };
                let note = Note {
                    id: pg_note.id.to_string(),
                    user_id: pg_note.user_id.to_string(),
                    title: pg_note.title,
                    content: pg_note.content,
                    created: pg_note.created.to_string(),
                    updated: pg_note.updated.to_string(),
                    deleted: pg_note.deleted.map(|d| d.to_string()),
                    user: None,
                };
                Ok(note)
            }
            None => Err(anyhow::anyhow!("Note not found")),
        }
    }
}

#[tonic::async_trait]
impl NotesService for MyService {
    type GetNotesStream = ReceiverStream<Result<Note, Status>>;

    async fn get_notes(
        &self,
        request: Request<UserId>,
    ) -> Result<Response<Self::GetNotesStream>, Status> {
        #[cfg(debug_assertions)]
        println!("GetNotes = {:?}", request);
        let start = std::time::Instant::now();

        let client = self
            .pool
            .get()
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        let user_id = request.into_inner().user_id;
        let user_id = Uuid::parse_str(&user_id).map_err(|e| Status::internal(e.to_string()))?;

        let rows = client
            .query(
                "SELECT * FROM notes WHERE user_id = $1 and deleted is null order by created desc",
                &[&user_id],
            )
            .await
            .map_err(|e| Status::internal(e.to_string()))?;
        println!("Prepare: {:?}", start.elapsed());

        let (tx, rx) = mpsc::channel(128);
        tokio::spawn(async move {
            // TODO: use try_get
            for row in rows {
                let pg_note = PgNote {
                    id: row.get("id"),
                    user_id: row.get("user_id"),
                    title: row.get("title"),
                    content: row.get("content"),
                    created: row.get("created"),
                    updated: row.get("updated"),
                    deleted: row.get("deleted"),
                };
                let note = Note {
                    id: pg_note.id.to_string(),
                    user_id: pg_note.user_id.to_string(),
                    title: pg_note.title,
                    content: pg_note.content,
                    created: pg_note.created.to_string(),
                    updated: pg_note.updated.to_string(),
                    deleted: pg_note.deleted.map(|d| d.to_string()),
                    user: None,
                };
                match tx.send(Ok(note)).await {
                    Ok(_) => {}
                    Err(e) => {
                        println!("Error: {:?}", e);
                        break;
                    }
                }
            }
            let elapsed = start.elapsed();
            println!("Elapsed: {:.2?}", elapsed);
        });
        Ok(Response::new(ReceiverStream::new(rx)))
    }

    async fn create_note(&self, request: Request<Note>) -> Result<Response<Note>, Status> {
        #[cfg(debug_assertions)]
        println!("CreateNote = {:?}", request);
        let start = std::time::Instant::now();
        let mut client = self
            .pool
            .get()
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        // start transaction
        let tx = client
            .transaction()
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        let note = request.into_inner();
        let user_id =
            Uuid::parse_str(&note.user_id).map_err(|e| Status::internal(e.to_string()))?;

        tx.query_one(
            "insert into notes (title, content, user_id) values ($1, $2, $3) returning *",
            &[&note.title, &note.content, &user_id],
        )
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

        let elapsed = start.elapsed();
        println!("Elapsed: {:.2?}", elapsed);
        return Ok(Response::new(note));
    }

    async fn delete_note(&self, request: Request<NoteId>) -> Result<Response<Note>, Status> {
        println!("DeleteNote = {:?}", request);
        let start = std::time::Instant::now();

        // start transaction
        let mut client = self
            .pool
            .get()
            .await
            .map_err(|e| Status::internal(e.to_string()))?;
        let tx = client
            .transaction()
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        let request = request.into_inner();
        let note_uuid =
            Uuid::parse_str(&request.note_id).map_err(|e| Status::internal(e.to_string()))?;
        let user_uuid =
            Uuid::parse_str(&request.user_id).map_err(|e| Status::internal(e.to_string()))?;

        let row = tx
            .query_one(
                "UPDATE notes SET deleted = NOW() WHERE id = $1 AND user_id = $2 RETURNING *",
                &[&note_uuid, &user_uuid],
            )
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        let pg_note = PgNote {
            id: row.get("id"),
            user_id: row.get("user_id"),
            title: row.get("title"),
            content: row.get("content"),
            created: row.get("created"),
            updated: row.get("updated"),
            deleted: row.get("deleted"),
        };

        let note = Note {
            id: pg_note.id.to_string(),
            user_id: pg_note.user_id.to_string(),
            title: pg_note.title,
            content: pg_note.content,
            created: pg_note.created.to_string(),
            updated: pg_note.updated.to_string(),
            deleted: pg_note.deleted.map(|d| d.to_string()),
            user: None,
        };

        // commit transaction
        tx.commit()
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        let elapsed = start.elapsed();
        println!("Elapsed: {:.2?}", elapsed);
        return Ok(Response::new(note));
    }
}
