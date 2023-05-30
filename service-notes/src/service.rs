use crate::{
    db::{delete_note, get_notes_by_user_uuid, upsert_note},
    proto::{notes_service_server::NotesService, Note, NoteId, UserId},
    MyService,
};
use anyhow::Result;
use futures_util::StreamExt;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};
use uuid::Uuid;

use crate::models::*;

impl TryFrom<DieselNote> for Note {
    type Error = anyhow::Error;

    fn try_from(note: DieselNote) -> Result<Self, Self::Error> {
        let note = Note {
            id: note.id.to_string(),
            user_id: note.user_id.to_string(),
            title: note.title,
            content: note.content,
            created: note.created.to_string(),
            updated: note.updated.to_string(),
            deleted: note.deleted.map(|d| d.to_string()),
            user: None,
        };
        Ok(note)
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

        let conn = self
            .pool
            .get()
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        println!("Connect: {:?}", start.elapsed());

        let user_uuid = request.into_inner().user_id;
        let user_uuid = Uuid::parse_str(&user_uuid).map_err(|e| Status::internal(e.to_string()))?;

        let mut rows = get_notes_by_user_uuid(conn, user_uuid).await?;

        println!("Prepare: {:?}", start.elapsed());

        let (tx, rx) = mpsc::channel(128);
        tokio::spawn(async move {
            while let Some(row) = rows.next().await {
                let note = match row {
                    Ok(note) => note,
                    Err(e) => {
                        tx.send(Err(Status::internal(e.to_string())))
                            .await
                            .unwrap();
                        break;
                    }
                };
                let note: Note = match Note::try_from(note) {
                    Ok(note) => note,
                    Err(e) => {
                        tx.send(Err(Status::internal(e.to_string())))
                            .await
                            .unwrap();
                        break;
                    }
                };
                match tx.send(Ok(note)).await {
                    Ok(_) => {}
                    Err(e) => {
                        println!("Error: {:?}", e);
                        break;
                    }
                }
            }
            println!("Elapsed: {:.2?}", start.elapsed());
        });
        Ok(Response::new(ReceiverStream::new(rx)))
    }

    async fn create_note(&self, request: Request<Note>) -> Result<Response<Note>, Status> {
        #[cfg(debug_assertions)]
        println!("CreateNote = {:?}", request);
        let start = std::time::Instant::now();

        let conn = self
            .pool
            .get()
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        let note = request.into_inner();
        let user_uuid =
            Uuid::parse_str(&note.user_id).map_err(|e| Status::internal(e.to_string()))?;

        let note = UpsertNote {
            id: None,
            user_id: &user_uuid,
            title: &note.title,
            content: &note.content,
        };
        let note = upsert_note(conn, note).await?;
        let note: Note = Note::try_from(note).map_err(|e| Status::internal(e.to_string()))?;

        println!("Elapsed: {:.2?}", start.elapsed());
        return Ok(Response::new(note));
    }

    async fn delete_note(&self, request: Request<NoteId>) -> Result<Response<Note>, Status> {
        println!("DeleteNote = {:?}", request);
        let start = std::time::Instant::now();

        let conn = self
            .pool
            .get()
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        let request = request.into_inner();
        let note_uuid =
            Uuid::parse_str(&request.note_id).map_err(|e| Status::internal(e.to_string()))?;
        let user_uuid =
            Uuid::parse_str(&request.user_id).map_err(|e| Status::internal(e.to_string()))?;

        let note = delete_note(conn, user_uuid, note_uuid).await?;
        let note = Note::try_from(note).map_err(|e| Status::internal(e.to_string()))?;

        println!("Elapsed: {:.2?}", start.elapsed());
        return Ok(Response::new(note));
    }
}
