use crate::{
    proto::{
        notes_service_server::NotesService, users_service_client::UsersServiceClient, Count, Empty,
        Id, Note, NoteResponse, Page,
    },
    MyService,
};
use anyhow::Result;
use futures_util::TryStreamExt;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{transport::Endpoint, Request, Response, Status};

#[tonic::async_trait]
impl NotesService for MyService {
    type GetNotesByUserIdStream = ReceiverStream<Result<NoteResponse, Status>>;

    async fn count_notes_by_user_id(
        &self,
        request: Request<Empty>,
    ) -> Result<Response<Count>, Status> {
        let start = std::time::Instant::now();
        let metadata = request.metadata();
        let user_id = rusve_notes::auth(metadata, &self.env.jwt_secret)?.id;

        let conn = self.pool.get().await.map_err(|e| {
            tracing::error!("Failed to get connection: {:?}", e);
            Status::internal("Failed to get connection")
        })?;

        let count = crate::note_db::count_notes_by_user_id(&conn, &user_id)
            .await
            .map_err(|e| {
                tracing::error!("Failed to count notes: {:?}", e);
                Status::internal("Failed to count notes")
            })?;

        tracing::info!("CountNotesByUserId: {:?}", start.elapsed());
        return Ok(Response::new(Count { count }));
    }

    async fn get_notes_by_user_id(
        &self,
        request: Request<Page>,
    ) -> Result<Response<Self::GetNotesByUserIdStream>, Status> {
        let start = std::time::Instant::now();
        let metadata = request.metadata();
        let user_id = rusve_notes::auth(metadata, &self.env.jwt_secret)?.id;

        let conn = self.pool.get().await.map_err(|e| {
            tracing::error!("Failed to get connection: {:?}", e);
            Status::internal("Failed to get connection")
        })?;

        let page = request.into_inner();
        let notes_stream =
            crate::note_db::get_notes_by_user_id(&conn, &user_id, page.offset, page.limit)
                .await
                .map_err(|e| {
                    tracing::error!("Failed to get notes: {:?}", e);
                    Status::internal("Failed to get notes")
                })?;

        // Showcase of how to connect to another service
        let endpoint: Endpoint = self.env.users_url.parse().map_err(|err| {
            tracing::error!("Failed to parse users url: {:?}", err);
            Status::internal("Failed to parse users url")
        })?;
        let client = UsersServiceClient::connect(endpoint).await.map_err(|err| {
            tracing::error!("Failed to connect to users service: {:?}", err);
            Status::internal("Failed to connect to users service")
        })?;

        let (tx, rx) = mpsc::channel(128);
        // TODO: can we avoid cloning here?
        let jwt_secret = self.env.jwt_secret.clone();
        let shared_client = tokio::sync::Mutex::new(client);
        tokio::spawn(async move {
            futures_util::pin_mut!(notes_stream);
            while let Ok(Some(note)) = notes_stream.try_next().await {
                let tx = tx.clone();
                // tokio::spawn(async move {
                let note: Note = match note.try_into() {
                    Ok(note) => note,
                    Err(e) => {
                        tracing::error!("Failed to get note: {:?}", e);
                        return;
                    }
                };

                // Totally overengineered way to get user profile
                // This is to show how to connect to another service
                // Using a shared client inside async block
                let mut request = tonic::Request::new(crate::proto::Empty {});
                let metadata = request.metadata_mut();
                rusve_notes::generate_metadata(metadata, note.user_id.as_str(), &jwt_secret)
                    .unwrap_or_else(|e| {
                        tracing::error!("Failed to generate metadata: {:?}", e);
                        return;
                    });
                let mut new_client = shared_client.lock().await;
                let user_profile = match new_client.get_profile_by_user_id(request).await {
                    Ok(response) => response.into_inner(),
                    Err(e) => {
                        tracing::error!("Failed to get user profile: {:?}", e);
                        return;
                    }
                };

                let note_response = NoteResponse {
                    note: Some(note),
                    profile: Some(user_profile),
                };
                if let Err(e) = tx.send(Ok(note_response)).await {
                    tracing::error!("Failed to send note: {:?}", e);
                }
                //});
            }
            // loop {
            //     let note = match notes_stream.try_next().await {
            //         Ok(Some(note)) => note,
            //         Ok(None) => break,
            //         Err(e) => {
            //             tracing::error!("Failed to get note: {:?}", e);
            //             if let Err(e) = tx.send(Err(Status::internal("Failed to get note"))).await {
            //                 tracing::error!("Failed to send error: {:?}", e);
            //             }
            //             break;
            //         }
            //     };
            //     let note: Note = match note.try_into() {
            //         Ok(note) => note,
            //         Err(e) => {
            //             tracing::error!("Failed to convert note: {:?}", e);
            //             if let Err(e) = tx
            //                 .send(Err(Status::internal("Failed to convert note")))
            //                 .await
            //             {
            //                 tracing::error!("Failed to send error: {:?}", e);
            //             }
            //             return;
            //         }
            //     };
            //     if let Err(e) = tx.send(Ok(note)).await {
            //         tracing::error!("Failed to send note: {:?}", e);
            //     }
            // }
            tracing::info!("GetNotesByUserId: {:?}", start.elapsed());
        });
        Ok(Response::new(ReceiverStream::new(rx)))
    }

    async fn get_note_by_id(&self, request: Request<Id>) -> Result<Response<Note>, Status> {
        let start = std::time::Instant::now();
        let metadata = request.metadata();
        let user_id = rusve_notes::auth(metadata, &self.env.jwt_secret)?.id;

        let conn = self.pool.get().await.map_err(|e| {
            tracing::error!("Failed to get connection: {:?}", e);
            Status::internal("Failed to get connection")
        })?;

        let id = request.into_inner();
        let note = crate::note_db::get_note_by_id(&conn, &id.id, &user_id)
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
        let user_id = rusve_notes::auth(metadata, &self.env.jwt_secret)?.id;

        let mut note = request.into_inner();
        crate::note_validation::Validation::validate(&note)?;

        let conn = self.pool.get().await.map_err(|e| {
            tracing::error!("Failed to get connection: {:?}", e);
            Status::internal("Failed to get connection")
        })?;

        if note.id.is_empty() {
            note = crate::note_db::insert_note(&conn, &user_id, &note)
                .await
                .map_err(|e| {
                    tracing::error!("Failed to insert note: {:?}", e);
                    Status::internal("Failed to insert note")
                })?;
        } else {
            note = crate::note_db::update_note(&conn, &user_id, &note)
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
        let user_id = rusve_notes::auth(metadata, &self.env.jwt_secret)?.id;

        let conn = self.pool.get().await.map_err(|e| {
            tracing::error!("Failed to get connection: {:?}", e);
            Status::internal("Failed to get connection")
        })?;

        let id = request.into_inner();
        crate::note_db::delete_note_by_id(&conn, &id.id, &user_id)
            .await
            .map_err(|e| {
                tracing::error!("Failed to delete note: {:?}", e);
                Status::internal("Failed to delete note")
            })?;

        tracing::info!("DeleteNote: {:?}", start.elapsed());
        return Ok(Response::new(Empty {}));
    }
}
