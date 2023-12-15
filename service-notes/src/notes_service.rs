use crate::{
    notes_db::{self, insert_note},
    proto::{notes_service_server::NotesService, Empty, Id, Note},
    MyService,
};
use anyhow::Result;
use futures_util::TryStreamExt;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};

#[tonic::async_trait]
impl NotesService for MyService {
    type GetNotesByUserIdStream = ReceiverStream<Result<Note, Status>>;

    async fn get_notes_by_user_id(
        &self,
        request: Request<Empty>,
    ) -> Result<Response<Self::GetNotesByUserIdStream>, Status> {
        let start = std::time::Instant::now();
        let metadata = request.metadata();
        let user_id = rusve_notes::auth(&metadata)?.user_id;

        let conn = self.pool.get().await.map_err(|e| {
            tracing::error!("Failed to get connection: {:?}", e);
            Status::internal("Failed to get connection")
        })?;

        let notes_stream = notes_db::get_notes_by_user_id(&conn, &user_id)
            .await
            .map_err(|e| {
                tracing::error!("Failed to get notes: {:?}", e);
                Status::internal("Failed to get notes")
            })?;

        let (tx, rx) = mpsc::channel(128);
        tokio::spawn(async move {
            futures_util::pin_mut!(notes_stream);
            loop {
                let note = match notes_stream.try_next().await {
                    Ok(Some(note)) => note,
                    Ok(None) => break,
                    Err(e) => {
                        tracing::error!("Failed to get note: {:?}", e);
                        if let Err(e) = tx.send(Err(Status::internal("Failed to get note"))).await {
                            tracing::error!("Failed to send error: {:?}", e);
                        }
                        break;
                    }
                };
                let note: Note = match note.try_into() {
                    Ok(note) => note,
                    Err(e) => {
                        tracing::error!("Failed to convert note: {:?}", e);
                        if let Err(e) = tx
                            .send(Err(Status::internal("Failed to convert note")))
                            .await
                        {
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
            tracing::info!("GetNotesByUserId: {:?}", start.elapsed());
        });
        Ok(Response::new(ReceiverStream::new(rx)))
    }

    async fn get_note_by_id(&self, request: Request<Id>) -> Result<Response<Note>, Status> {
        let start = std::time::Instant::now();
        let metadata = request.metadata();
        let user_id = rusve_notes::auth(&metadata)?.user_id;

        let conn = self.pool.get().await.map_err(|e| {
            tracing::error!("Failed to get connection: {:?}", e);
            Status::internal("Failed to get connection")
        })?;

        let id = request.into_inner();
        let note = notes_db::get_note_by_id(&conn, &id.id, &user_id)
            .await
            .map_err(|e| {
                tracing::error!("Failed to get note: {:?}", e);
                Status::internal("Failed to get note")
            })?;

        tracing::info!("GetNote: {:?}", start.elapsed());
        return Ok(Response::new(note));
    }

    async fn create_note(&self, request: Request<Note>) -> Result<Response<Note>, Status> {
        let start = std::time::Instant::now();
        let metadata = request.metadata();
        let user_id = rusve_notes::auth(&metadata)?.user_id;

        let conn = self.pool.get().await.map_err(|e| {
            tracing::error!("Failed to get connection: {:?}", e);
            Status::internal("Failed to get connection")
        })?;

        let mut note = request.into_inner();

        crate::notes_validation::Validation::validate(&note)?;

        if note.id.is_empty() {
            note = insert_note(&conn, &user_id, &note).await.map_err(|e| {
                tracing::error!("Failed to insert note: {:?}", e);
                Status::internal("Failed to insert note")
            })?;
        } else {
            note = notes_db::update_note(&conn, &user_id, &note)
                .await
                .map_err(|e| {
                    tracing::error!("Failed to update note: {:?}", e);
                    Status::internal("Failed to update note")
                })?;
        }

        tracing::info!("CreateNote: {:?}", start.elapsed());
        return Ok(Response::new(note));
    }

    async fn delete_note_by_id(&self, request: Request<Id>) -> Result<Response<Empty>, Status> {
        let start = std::time::Instant::now();
        let metadata = request.metadata();
        let user_id = rusve_notes::auth(&metadata)?.user_id;

        let conn = self.pool.get().await.map_err(|e| {
            tracing::error!("Failed to get connection: {:?}", e);
            Status::internal("Failed to get connection")
        })?;

        let id = request.into_inner();
        notes_db::delete_note_by_id(&conn, &id.id, &user_id)
            .await
            .map_err(|e| {
                tracing::error!("Failed to delete note: {:?}", e);
                Status::internal("Failed to delete note")
            })?;

        tracing::info!("DeleteNote: {:?}", start.elapsed());
        return Ok(Response::new(Empty {}));
    }
}
