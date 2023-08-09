use crate::{
    notes_db,
    proto::{notes_service_server::NotesService, Empty, Note, NoteId, UserId},
    MyService,
};
use anyhow::Result;
use futures_util::TryStreamExt;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};

#[tonic::async_trait]
impl NotesService for MyService {
    type GetNotesStream = ReceiverStream<Result<Note, Status>>;

    async fn get_notes(
        &self,
        request: Request<UserId>,
    ) -> Result<Response<Self::GetNotesStream>, Status> {
        let start = std::time::Instant::now();

        let conn = self.pool.get().await.map_err(|e| {
            tracing::error!("Failed to get connection: {:?}", e);
            Status::internal(e.to_string())
        })?;

        let request = request.into_inner();
        let user_id = uuid::Uuid::parse_str(&request.user_id).map_err(|e| {
            tracing::error!("Failed to parse user_id: {:?}", e);
            Status::invalid_argument(e.to_string())
        })?;

        let notes = notes_db::get_notes(&conn, &user_id).await.map_err(|e| {
            tracing::error!("Failed to get notes: {:?}", e);
            Status::internal(e.to_string())
        })?;

        let (tx, rx) = mpsc::channel(128);
        tokio::spawn(async move {
            futures_util::pin_mut!(notes);
            loop {
                let note = match notes.try_next().await {
                    Ok(Some(note)) => note,
                    Ok(None) => break,
                    Err(e) => {
                        tracing::error!("Failed to get note: {:?}", e);
                        if let Err(e) = tx.send(Err(Status::internal(e.to_string()))).await {
                            tracing::error!("Failed to send error: {:?}", e);
                        }
                        break;
                    }
                };
                let note: Note = match note.try_into() {
                    Ok(note) => note,
                    Err(e) => {
                        tracing::error!("Failed to convert note: {:?}", e);
                        if let Err(e) = tx.send(Err(Status::internal(e.to_string()))).await {
                            tracing::error!("Failed to send error: {:?}", e);
                        }
                        break;
                    }
                };
                if let Err(e) = tx.send(Ok(note)).await {
                    tracing::error!("Failed to send note: {:?}", e);
                    break;
                }
            }
            tracing::info!("GetNotes: {:?}", start.elapsed());
        });
        Ok(Response::new(ReceiverStream::new(rx)))
    }

    async fn create_note(&self, request: Request<Note>) -> Result<Response<Note>, Status> {
        let start = std::time::Instant::now();

        let conn = self.pool.get().await.map_err(|e| {
            tracing::error!("Failed to get connection: {:?}", e);
            Status::internal(e.to_string())
        })?;

        let note = request.into_inner();
        let note = notes_db::create_note(&conn, &note).await.map_err(|e| {
            tracing::error!("Failed to insert note: {:?}", e);
            Status::internal(e.to_string())
        })?;

        tracing::info!("CreateNote: {:?}", start.elapsed());
        return Ok(Response::new(note));
    }

    async fn delete_note(&self, request: Request<NoteId>) -> Result<Response<Empty>, Status> {
        let start = std::time::Instant::now();

        let conn = self.pool.get().await.map_err(|e| {
            tracing::error!("Failed to get connection: {:?}", e);
            Status::internal(e.to_string())
        })?;

        let note = request.into_inner();
        let note_id = uuid::Uuid::parse_str(&note.note_id).map_err(|e| {
            tracing::error!("Failed to parse note_id: {:?}", e);
            Status::invalid_argument(e.to_string())
        })?;

        notes_db::delete_note(&conn, &note_id).await.map_err(|e| {
            tracing::error!("Failed to delete note: {:?}", e);
            Status::internal(e.to_string())
        })?;

        tracing::info!("DeleteNote: {:?}", start.elapsed());
        return Ok(Response::new(Empty {}));
    }
}
