use crate::proto::utils_service_server::UtilsService;
use crate::proto::{File, FileId, FileType, TargetId};
use crate::MyService;
use anyhow::Result;
use futures_util::TryStreamExt;
use google_cloud_default::WithAuthExt;
use google_cloud_storage::client::{Client, ClientConfig};
use google_cloud_storage::http::objects::delete::DeleteObjectRequest;
use google_cloud_storage::http::objects::download::Range;
use google_cloud_storage::http::objects::get::GetObjectRequest;
use google_cloud_storage::http::objects::upload::{Media, UploadObjectRequest, UploadType};
use sqlx::types::time::OffsetDateTime;
use sqlx::{postgres::PgRow, query, types::Uuid, Row};
use tokio::io::AsyncWriteExt;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};

impl TryFrom<Option<PgRow>> for File {
    type Error = anyhow::Error;

    fn try_from(row: Option<PgRow>) -> Result<Self, Self::Error> {
        match row {
            Some(row) => {
                let id: Uuid = row.try_get("id")?;
                let created: OffsetDateTime = row.try_get("created")?;
                let updated: OffsetDateTime = row.try_get("updated")?;
                let deleted: Option<OffsetDateTime> = row.try_get("deleted")?;
                let target_id: Uuid = row.try_get("target_id")?;
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
                    name,
                    r#type: file_type.into(),
                    buffer: Vec::new(),
                };
                Ok(file)
            }
            None => Err(anyhow::anyhow!("File not found")),
        }
    }
}

#[tonic::async_trait]
impl UtilsService for MyService {
    type GetFilesStream = ReceiverStream<Result<File, Status>>;

    async fn get_files(
        &self,
        request: Request<TargetId>,
    ) -> Result<Response<Self::GetFilesStream>, Status> {
        #[cfg(debug_assertions)]
        println!("GetFiles: {:?}", request);
        let start = std::time::Instant::now();
        let pool = self.pool.clone();
        let (tx, rx) = mpsc::channel(4);

        let request = request.into_inner();
        let target_uuid =
            Uuid::parse_str(&request.target_id).map_err(|e| Status::internal(e.to_string()))?;
        let r#type = FileType::from_i32(request.r#type)
            .ok_or(Status::internal("Invalid file type"))?
            .as_str_name();

        tokio::spawn(async move {
            let mut files_stream = query("SELECT * FROM files WHERE target_id = $1 and type = $2 and deleted is null order by created desc")
                .bind(target_uuid)
                .bind(r#type)
                .fetch(&pool);

            loop {
                match files_stream.try_next().await {
                    Ok(None) => {
                        let elapsed = start.elapsed();
                        println!("Elapsed: {:.2?}", elapsed);
                        break;
                    }
                    Ok(file) => {
                        let mut file: File = match file.try_into() {
                            Ok(file) => file,
                            Err(e) => {
                                tx.send(Err(Status::internal(e.to_string()))).await.unwrap();
                                break;
                            }
                        };
                        let env = std::env::var("ENV").unwrap();
                        let bucket = std::env::var("BUCKET").unwrap();
                        let mut buffer = Vec::new();
                        if env == "development" {
                            let file_path = format!("/app/files/{}/{}", &file.id, &file.name);
                            buffer = match std::fs::read(file_path) {
                                Ok(buffer) => buffer,
                                Err(e) => {
                                    tx.send(Err(Status::internal(e.to_string()))).await.unwrap();
                                    break;
                                }
                            };
                        } else if env == "production" {
                            let config = match ClientConfig::default().with_auth().await {
                                Ok(config) => config,
                                Err(e) => {
                                    tx.send(Err(Status::internal(e.to_string()))).await.unwrap();
                                    break;
                                }
                            };
                            let client = Client::new(config);
                            let data = client
                                .download_object(
                                    &GetObjectRequest {
                                        bucket,
                                        object: format!("{}/{}", &file.id, &file.name),
                                        ..Default::default()
                                    },
                                    &Range::default(),
                                )
                                .await;
                            buffer = match data {
                                Ok(data) => data,
                                Err(e) => {
                                    tx.send(Err(Status::internal(e.to_string()))).await.unwrap();
                                    break;
                                }
                            };
                        }
                        file.buffer = buffer;
                        tx.send(Ok(file)).await.unwrap();
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

    async fn get_file(&self, request: Request<FileId>) -> Result<Response<File>, Status> {
        #[cfg(debug_assertions)]
        println!("GetFile: {:?}", request);
        let start = std::time::Instant::now();

        let pool = self.pool.clone();
        let request = request.into_inner();
        let uuid =
            Uuid::parse_str(&request.file_id).map_err(|e| Status::internal(e.to_string()))?;
        let target_uuid =
            Uuid::parse_str(&request.target_id).map_err(|e| Status::internal(e.to_string()))?;
        let row = query("SELECT * FROM files WHERE id = $1 and target_id = $2 and deleted is null")
            .bind(uuid)
            .bind(target_uuid)
            .fetch_one(&pool)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        let mut file: File = match Some(row).try_into() {
            Ok(file) => file,
            Err(e) => return Err(Status::internal(e.to_string())),
        };

        let env = std::env::var("ENV").unwrap();
        let bucket = std::env::var("BUCKET").unwrap();
        let mut buffer = Vec::new();

        if env == "development" {
            let file_path = format!("/app/files/{}/{}", &file.id, &file.name);
            buffer = match std::fs::read(file_path) {
                Ok(buffer) => buffer,
                Err(e) => return Err(Status::internal(e.to_string())),
            };
        } else if env == "production" {
            let config = match ClientConfig::default().with_auth().await {
                Ok(config) => config,
                Err(e) => return Err(Status::internal(e.to_string())),
            };
            let client = Client::new(config);
            let data = client
                .download_object(
                    &GetObjectRequest {
                        bucket,
                        object: format!("{}/{}", &file.id, &file.name),
                        ..Default::default()
                    },
                    &Range::default(),
                )
                .await;
            buffer = match data {
                Ok(data) => data,
                Err(e) => return Err(Status::internal(e.to_string())),
            };
        }
        file.buffer = buffer;

        let elapsed = start.elapsed();
        println!("Elapsed: {:.2?}", elapsed);
        Ok(Response::new(file))
    }

    async fn create_file(&self, request: Request<File>) -> Result<Response<File>, Status> {
        #[cfg(debug_assertions)]
        println!("CreateFile");
        let start = std::time::Instant::now();

        // start transaction
        let pool = self.pool.clone();
        let mut tx = pool
            .begin()
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        // get file from request
        let file = request.into_inner();
        let file_buffer = file.buffer;
        let uuid = Uuid::parse_str(&file.target_id).map_err(|e| Status::internal(e.to_string()))?;
        let r#type = FileType::from_i32(file.r#type)
            .ok_or(anyhow::anyhow!("Invalid file type"))
            .map_err(|e| Status::internal(e.to_string()))?
            .as_str_name();

        // save file to db
        let row =
            query("INSERT INTO files (target_id, name, type) VALUES ($1, $2, $3) RETURNING *")
                .bind(uuid)
                .bind(file.name)
                .bind(r#type)
                .fetch_one(&mut tx)
                .await
                .map_err(|e| Status::internal(e.to_string()))?;
        let file: File = match Some(row).try_into() {
            Ok(file) => file,
            Err(e) => return Err(Status::internal(e.to_string())),
        };

        let env = std::env::var("ENV").unwrap();
        let bucket = std::env::var("BUCKET").unwrap();
        if env == "development" {
            // save file to disk
            let file_path = format!("/app/files/{}/{}", file.id, file.name);
            tokio::fs::create_dir_all(format!("/app/files/{}", file.id)).await?;
            let mut new_file = tokio::fs::File::create(file_path).await?;
            new_file.write_all(&file_buffer).await?;
        } else if env == "production" {
            // save to GCP storage
            let config = ClientConfig::default()
                .with_auth()
                .await
                .map_err(|e| Status::internal(e.to_string()))?;
            let client = Client::new(config);
            let file_path = format!("{}/{}", file.id, file.name);
            let upload_type = UploadType::Simple(Media::new(file_path));
            let uploaded = client
                .upload_object(
                    &UploadObjectRequest {
                        bucket: bucket.to_string(),
                        ..Default::default()
                    },
                    file_buffer,
                    &upload_type,
                )
                .await;
            if let Err(e) = uploaded {
                // TODO - do i need it? if yes, use it everywhere
                tx.rollback().await.unwrap();
                return Err(Status::internal(e.to_string()));
            }
        }

        // commit transaction
        tx.commit()
            .await
            .map_err(|e| Status::internal(e.to_string()))?;
        let elapsed = start.elapsed();
        println!("Elapsed: {:.2?}", elapsed);
        return Ok(Response::new(file));
    }

    async fn delete_file(&self, request: Request<FileId>) -> Result<Response<File>, Status> {
        #[cfg(debug_assertions)]
        println!("DeleteFile: {:?}", request);
        let start = std::time::Instant::now();

        // start transaction
        let pool = self.pool.clone();
        let mut tx = pool
            .begin()
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        let request = request.into_inner();
        let file_id =
            Uuid::parse_str(&request.file_id).map_err(|e| Status::internal(e.to_string()))?;
        let target_id =
            Uuid::parse_str(&request.target_id).map_err(|e| Status::internal(e.to_string()))?;
        let row =
            query("UPDATE files SET deleted = now() WHERE id = $1 and target_id = $2 RETURNING *")
                .bind(file_id)
                .bind(target_id)
                .fetch_one(&mut tx)
                .await
                .map_err(|e| Status::not_found(e.to_string()))?;
        let file: File = match Some(row).try_into() {
            Ok(file) => file,
            Err(e) => return Err(Status::internal(e.to_string())),
        };

        let env = std::env::var("ENV").unwrap();
        let bucket = std::env::var("BUCKET").unwrap();
        if env == "development" {
            // delete file from disk
            tokio::fs::remove_dir_all(format!("/app/files/{}", file.id)).await?;
        } else if env == "production" {
            // delete from GCP storage
            let config = ClientConfig::default()
                .with_auth()
                .await
                .map_err(|e| Status::internal(e.to_string()))?;
            let client = Client::new(config);
            let deleted = client
                .delete_object(&DeleteObjectRequest {
                    bucket: bucket.to_string(),
                    object: format!("{}/{}", file.id, file.name),
                    ..Default::default()
                })
                .await;
            if let Err(e) = deleted {
                return Err(Status::internal(e.to_string()));
            }
        }

        // commit transaction
        tx.commit()
            .await
            .map_err(|e| Status::internal(e.to_string()))?;
        let elapsed = start.elapsed();
        println!("Elapsed: {:.2?}", elapsed);
        return Ok(Response::new(file));
    }
}
