use crate::files_db::{get_file_by_id, get_files_by_target_id};
use crate::proto::utils_service_server::UtilsService;
use crate::proto::{File, FileId, FileType, TargetId};
use crate::MyService;
use anyhow::Result;
use google_cloud_default::WithAuthExt;
use google_cloud_storage::client::{Client, ClientConfig};
use google_cloud_storage::http::objects::delete::DeleteObjectRequest;
use google_cloud_storage::http::objects::download::Range;
use google_cloud_storage::http::objects::get::GetObjectRequest;
use google_cloud_storage::http::objects::upload::{Media, UploadObjectRequest, UploadType};
use tokio::io::AsyncWriteExt;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};
use uuid::Uuid;
use futures_util::stream::StreamExt;

use crate::models::*;

impl TryFrom<DieselFile> for File {
    type Error = Status;

    fn try_from(file: DieselFile) -> Result<Self, Self::Error> {
        let file = File {
            id: file.id,
            created: file.created.to_string(),
            updated: file.updated.to_string(),
            deleted: file.deleted.map(|d| d.to_string()),
            target_id: file.target_id,
            name: file.name,
            r#type: FileType::from_str_name(&file.type_)
                .ok_or(Status::internal("Invalid file type"))?
                .into(),
            buffer: Vec::new(),
            url: "".to_string(),
        };
        Ok(file)
    }
}

impl TryFrom<Result<DieselFile, diesel::result::Error>> for File {
    type Error = Status;

    fn try_from(file: Result<DieselFile, diesel::result::Error>) -> Result<Self, Self::Error> {
        let file = match file {
            Ok(file) => file,
            Err(e) => return Err(Status::internal(e.to_string())),
        };
        let file: File = match File::try_from(file) {
            Ok(file) => file,
            Err(e) => return Err(e),
        };
        Ok(file)
    }
}

#[tonic::async_trait]
impl UtilsService for MyService {
    type GetFilesStream = ReceiverStream<Result<File, Status>>;

    async fn get_files(
        &self,
        request: Request<TargetId>,
    ) -> Result<Response<Self::GetFilesStream>, Status> {
        println!("GetFiles");
        let start = std::time::Instant::now();

        let conn = self
            .pool
            .get()
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        let request = request.into_inner();
        let r#type = FileType::from_i32(request.r#type)
            .ok_or(Status::internal("Invalid file type"))?
            .as_str_name();
        let mut rows = get_files_by_target_id(conn, request.target_id, r#type).await?;

        let (tx, rx) = mpsc::channel(4);
        tokio::spawn(async move {
            while let Some(file) = rows.next().await {
                let mut file: File = match File::try_from(file) {
                    Ok(file) => file,
                    Err(e) => {
                        tx.send(Err(Status::internal(e.to_string()))).await.unwrap();
                        break;
                    }
                };
                let buffer = Vec::new();
                file.buffer = buffer;
                tx.send(Ok(file)).await.unwrap();
            }
            println!("Elapsed: {:.2?}", start.elapsed());
        });
        Ok(Response::new(ReceiverStream::new(rx)))
    }
    async fn get_file(&self, request: Request<FileId>) -> Result<Response<File>, Status> {
        println!("GetFile");
        let start = std::time::Instant::now();

        let conn = self
            .pool
            .get()
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        let request = request.into_inner();

        let file = get_file_by_id(conn, request.file_id, request.target_id).await?;
        let mut file = File::try_from(file)?;

        let env = std::env::var("ENV").unwrap();
        let bucket = std::env::var("BUCKET").unwrap();
        let mut buffer = Vec::new();

        let file_id = Uuid::from_slice(&file.id)
            .map_err(|e| Status::internal(e.to_string()))?
            .to_string();
        if env == "development" {
            let file_path = format!("/app/files/{}/{}", &file_id, &file.name);
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
                        object: format!("{}/{}", &file_id, &file.name),
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
        println!("CreateFile");
        let start = std::time::Instant::now();

        let conn = self
            .pool
            .get()
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        // get file from request
        let file = request.into_inner();
        let file_buffer = file.buffer;
        let r#type = FileType::from_i32(file.r#type)
            .ok_or(anyhow::anyhow!("Invalid file type"))
            .map_err(|e| Status::internal(e.to_string()))?
            .as_str_name();

        // save file to db
        let new_file = InsertFile {
            id: Uuid::now_v7().as_bytes().to_vec(),
            name: &file.name,
            type_: r#type,
            target_id: file.target_id,
        };
        let file: File = crate::files_db::create_file(conn, new_file)
            .await?
            .try_into()?;

        let env = std::env::var("ENV").unwrap();
        let bucket = std::env::var("BUCKET").unwrap();
        let file_id = Uuid::from_slice(&file.id)
            .map_err(|e| Status::internal(e.to_string()))?
            .to_string();
        if env == "development" {
            // save file to disk
            tokio::fs::create_dir_all(format!("/app/files/{}", &file_id)).await?;
            let file_path = format!("/app/files/{}/{}", &file_id, &file.name);
            let mut new_file = tokio::fs::File::create(file_path).await?;
            new_file.write_all(&file_buffer).await?;
        } else if env == "production" {
            // save to GCP storage
            let config = ClientConfig::default()
                .with_auth()
                .await
                .map_err(|e| Status::internal(e.to_string()))?;
            let client = Client::new(config);
            let file_path = format!("{}/{}", &file_id, &file.name);
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
                return Err(Status::internal(e.to_string()));
            }
        }
        println!("Elapsed: {:.2?}", start.elapsed());
        Ok(Response::new(file))
    }
    async fn delete_file(&self, request: Request<FileId>) -> Result<Response<File>, Status> {
        println!("DeleteFile");
        let start = std::time::Instant::now();

        let conn = self
            .pool
            .get()
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        let request = request.into_inner();

        let file: File = crate::files_db::delete_file(conn, request.file_id, request.target_id)
            .await?
            .try_into()?;

        let env = std::env::var("ENV").unwrap();
        let bucket = std::env::var("BUCKET").unwrap();

        let file_id = Uuid::from_slice(&file.id)
            .map_err(|e| Status::internal(e.to_string()))?
            .to_string();

        if env == "development" {
            // delete file from disk
            tokio::fs::remove_dir_all(format!("/app/files/{}", &file_id)).await?;
        } else if env == "production" {
            // delete from GCP storage
            let config = ClientConfig::default()
                .with_auth()
                .await
                .map_err(|e| Status::internal(e.to_string()))?;
            let client = Client::new(config);
            let del = client
                .delete_object(&DeleteObjectRequest {
                    bucket: bucket.to_string(),
                    object: format!("{}/{}", &file_id, &file.name),
                    ..Default::default()
                })
                .await;
            if let Err(e) = del {
                return Err(Status::internal(e.to_string()));
            }
        }

        println!("Elapsed: {:.2?}", start.elapsed());
        Ok(Response::new(file))
    }
}
