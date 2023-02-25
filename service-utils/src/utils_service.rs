use crate::{
    proto::{files_service_server::FilesService, File, FileType, TargetId, UserId},
    utils::{check_env, fetch_auth_metadata},
    CachedToken, MyService,
};
use anyhow::Result;
use futures::TryStreamExt;
use sqlx::types::time::OffsetDateTime;
use sqlx::{postgres::PgRow, query, types::Uuid, Row};
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};

fn map_file(row: Option<PgRow>) -> Result<File> {
    match row {
        Some(row) => {
            let id: Uuid = row.try_get("id")?;
            let created: OffsetDateTime = row.try_get("created")?;
            let updated: OffsetDateTime = row.try_get("updated")?;
            let deleted: Option<OffsetDateTime> = row.try_get("deleted")?;

            let target_id: Uuid = row.try_get("targetId")?;
            let name: String = row.try_get("name")?;

            let r#type: String = row.try_get("type")?;
            let file_type =
                FileType::from_str_name(&r#type).ok_or(anyhow::anyhow!("Invalid file type"))?;

            let file = File {
                id: id.to_string(),
                created: created.to_string(),
                updated: updated.to_string(),
                deleted: deleted.map(|d| d.to_string()),
                target_id: target_id.to_string(),
                name: name.to_string(),
                r#type: file_type.into(),
            };
            return Ok(file);
        }
        None => Err(anyhow::anyhow!("File not found")),
    }
}

#[tonic::async_trait]
impl FilesService for MyService {
    type GetFilesStream = ReceiverStream<Result<File, Status>>;

    async fn get_files(
        &self,
        request: Request<TargetId>,
    ) -> Result<Response<Self::GetFilesStream>, Status> {
        #[cfg(debug_assertions)]
        println!("GetNotes = {:?}", request);
        let start = std::time::Instant::now();

        let pool = self.pool.clone();

        let (tx, rx) = mpsc::channel(4);
        let user_id = request.into_inner().user_id;
        let uuid = Uuid::parse_str(&user_id).map_err(|e| Status::internal(e.to_string()))?;

        // User service
        let mut users_conn = self.users_conn.clone();
        let cached_token: Arc<Mutex<CachedToken>> = self.cached_token.clone();
        let uri_users = check_env("URI_USERS").map_err(|e| Status::internal(e.to_string()))?;
        let metadata = fetch_auth_metadata(cached_token, &uri_users)
            .await
            .map_err(|e| {
                Status::internal(format!("Error fetching auth metadata: {}", e.to_string()))
            })?;

        tokio::spawn(async move {
            let mut notes_stream = query("SELECT * FROM notes WHERE \"userId\" = $1 and deleted is null order by created desc")
                .bind(uuid)
                .fetch(&pool);

            loop {
                match notes_stream.try_next().await {
                    Ok(None) => {
                        let elapsed = start.elapsed();
                        println!("Elapsed: {:.2?}", elapsed);
                        break;
                    }
                    Ok(note) => {
                        let note = map_note(note);
                        if let Err(note) = note {
                            tx.send(Err(Status::internal(note.to_string())))
                                .await
                                .unwrap();
                            break;
                        } else {
                            let note = note.unwrap();

                            // Get user
                            let request = Request::from_parts(
                                metadata.clone(),
                                Default::default(),
                                UserId {
                                    user_id: note.user_id.to_owned(),
                                },
                            );
                            println!("Request: {:?}", request);

                            let response = users_conn.get_user(request).await;
                            if let Err(e) = response {
                                println!("Error: {}", e);
                                tx.send(Err(Status::internal(e.to_string()))).await.unwrap();
                                break;
                            }
                            let response = response.unwrap();
                            let user = response.into_inner();
                            let mut note = note;
                            note.user = Some(user);
                            tx.send(Ok(note)).await.unwrap();
                        }
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

    async fn create_file(&self, request: Request<Note>) -> Result<Response<Note>, Status> {
        #[cfg(debug_assertions)]
        println!("CreateNote = {:?}", request);
        let start = std::time::Instant::now();

        let pool = self.pool.clone();

        let note = request.into_inner();
        let user_id =
            Uuid::parse_str(&note.user_id).map_err(|e| Status::internal(e.to_string()))?;

        let row =
            query("INSERT INTO notes (title, content, \"userId\") VALUES ($1, $2, $3) RETURNING *")
                .bind(note.title)
                .bind(note.content)
                .bind(user_id)
                .fetch_one(&pool)
                .await
                .map_err(|e| Status::internal(e.to_string()))?;

        let note = map_note(Some(row)).map_err(|e| Status::internal(e.to_string()))?;
        let elapsed = start.elapsed();
        println!("Elapsed: {:.2?}", elapsed);
        return Ok(Response::new(note));
    }

    async fn delete_file(&self, request: Request<NoteId>) -> Result<Response<Note>, Status> {
        println!("DeleteNote = {:?}", request);
        let start = std::time::Instant::now();

        let pool = self.pool.clone();

        let request = request.into_inner();
        let note_uuid =
            Uuid::parse_str(&request.note_id).map_err(|e| Status::internal(e.to_string()))?;
        let user_uuid =
            Uuid::parse_str(&request.user_id).map_err(|e| Status::internal(e.to_string()))?;

        let row =
            query("UPDATE notes SET deleted = NOW() WHERE id = $1 AND \"userId\" = $2 RETURNING *")
                .bind(note_uuid)
                .bind(user_uuid)
                .fetch_one(&pool)
                .await
                .map_err(|e| Status::not_found(e.to_string()))?;

        let note = map_note(Some(row)).map_err(|e| Status::internal(e.to_string()))?;
        let elapsed = start.elapsed();
        println!("Elapsed: {:.2?}", elapsed);
        return Ok(Response::new(note));
    }
}
