use crate::{
    notes_db::{delete_note, get_notes_by_user_uuid, upsert_5000_note},
    proto::{notes_service_server::NotesService, Empty, Note, NoteId, UserId},
    MyService,
};
use anyhow::Result;
use futures_util::StreamExt;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};

use crate::models::*;

impl TryFrom<DieselNote> for Note {
    type Error = anyhow::Error;

    fn try_from(note: DieselNote) -> Result<Self, Self::Error> {
        let note = Note {
            id: String::from_utf8(note.id)?,
            user_id: String::from_utf8(note.user_id)?,
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
        let user_uuid: Vec<u8> = user_uuid.as_bytes().to_vec();

        let mut rows = get_notes_by_user_uuid(conn, user_uuid).await?;

        println!("Prepare: {:?}", start.elapsed());

        let (tx, rx) = mpsc::channel(128);
        tokio::spawn(async move {
            while let Some(row) = rows.next().await {
                let note = match row {
                    Ok(note) => note,
                    Err(e) => {
                        tx.send(Err(Status::internal(e.to_string()))).await.unwrap();
                        break;
                    }
                };
                let note: Note = match Note::try_from(note) {
                    Ok(note) => note,
                    Err(e) => {
                        tx.send(Err(Status::internal(e.to_string()))).await.unwrap();
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

    async fn create_note(&self, request: Request<Note>) -> Result<Response<Empty>, Status> {
        #[cfg(debug_assertions)]
        println!("CreateNote = {:?}", request);
        let start = std::time::Instant::now();

        let conn = self
            .pool
            .get()
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        let note = request.into_inner();
        let user_uuid: Vec<u8> = note.user_id.as_bytes().to_vec();

        let new_note = UpsertNote {
            id: None,
            user_id: &user_uuid,
            title: &note.title,
            content: &note.content,
        };
        upsert_5000_note(conn, new_note).await?;

        println!("Elapsed: {:.2?}", start.elapsed());
        return Ok(Response::new(Empty {}));
    }

    async fn delete_note(&self, request: Request<NoteId>) -> Result<Response<Empty>, Status> {
        println!("DeleteNote = {:?}", request);
        let start = std::time::Instant::now();

        let conn = self
            .pool
            .get()
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        let request = request.into_inner();
        let note_uuid: Vec<u8> = request.note_id.as_bytes().to_vec();
        let user_uuid: Vec<u8> = request.user_id.as_bytes().to_vec();

        delete_note(conn, user_uuid, note_uuid).await?;

        println!("Elapsed: {:.2?}", start.elapsed());
        return Ok(Response::new(Empty {}));
    }
}
