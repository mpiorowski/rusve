use crate::proto::utils_service_server::UtilsService;
use crate::proto::{File, Id, Empty};
use crate::{files_utils, MyService};
use anyhow::Result;
use futures_util::TryStreamExt;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};

#[tonic::async_trait]
impl UtilsService for MyService {
    type GetFilesStream = ReceiverStream<Result<File, Status>>;

    async fn get_files(
        &self,
        request: Request<Id>,
    ) -> Result<Response<Self::GetFilesStream>, Status> {
        let start = std::time::Instant::now();

        let conn = self.pool.get().await.map_err(|e| {
            tracing::error!("Failed to get connection from pool: {:?}", e);
            Status::internal("Failed to get connection from pool")
        })?;

        let request = request.into_inner();
        let files = crate::files_db::get_files_by_target_id(&conn, &request.id)
            .await
            .map_err(|e| {
                tracing::error!("Failed to get files by target id: {:?}", e);
                Status::internal("Failed to get files by target id")
            })?;

        let (tx, rx) = mpsc::channel(128);
        tokio::spawn(async move {
            futures_util::pin_mut!(files);
            loop {
                let file = match files.try_next().await {
                    Ok(Some(file)) => file,
                    Ok(None) => break,
                    Err(e) => {
                        tracing::error!("Failed to get file: {:?}", e);
                        if let Err(e) = tx.send(Err(Status::internal("Failed to get file"))).await {
                            tracing::error!("Failed to send error: {:?}", e);
                        }
                        break;
                    }
                };
                let file: File = match file.try_into() {
                    Ok(file) => file,
                    Err(e) => {
                        tracing::error!("Failed to convert file: {:?}", e);
                        if let Err(e) = tx
                            .send(Err(Status::internal("Failed to convert file")))
                            .await
                        {
                            tracing::error!("Failed to send error: {:?}", e);
                        }
                        break;
                    }
                };
                if let Err(e) = tx.send(Ok(file)).await {
                    tracing::error!("Failed to send file: {:?}", e);
                    break;
                }
            }
            tracing::info!("GetFiles: {:?}", start.elapsed());
        });
        Ok(Response::new(ReceiverStream::new(rx)))
    }
    async fn get_file(&self, request: Request<Id>) -> Result<Response<File>, Status> {
        let start = std::time::Instant::now();

        let conn = self.pool.get().await.map_err(|e| {
            tracing::error!("Failed to get connection from pool: {:?}", e);
            Status::internal("Failed to get connection from pool")
        })?;

        let request = request.into_inner();

        let mut file = crate::files_db::get_file_by_id(&conn, &request.id)
            .await
            .map_err(|e| {
                tracing::error!("Failed to get file by id: {:?}", e);
                Status::internal("Failed to get file by id")
            })?;

        let file_buffer = files_utils::get_file_buffer(&self.env, &file.id, &file.file_name)
            .await
            .map_err(|e| {
                tracing::error!("Failed to get file buffer: {:?}", e);
                Status::internal("Failed to get file buffer")
            })?;

        file.file_buffer = file_buffer;

        tracing::info!("GetFile: {:?}", start.elapsed());
        Ok(Response::new(file))
    }
    async fn upload_file(&self, request: Request<File>) -> Result<Response<File>, Status> {
        let start = std::time::Instant::now();

        let mut conn = self.pool.get().await.map_err(|e| {
            tracing::error!("Failed to get connection from pool: {:?}", e);
            Status::internal("Failed to get connection from pool")
        })?;
        let tx = conn.transaction().await.map_err(|e| {
            tracing::error!("Failed to start transaction: {:?}", e);
            Status::internal("Failed to start transaction")
        })?;

        // get file from request
        let file = request.into_inner();
        let file_buffer = file.file_buffer.to_owned();

        let file = crate::files_db::insert_file(&tx, &file)
            .await
            .map_err(|e| {
                tracing::error!("Failed to create file: {:?}", e);
                Status::internal("Failed to create file")
            })?;

        files_utils::upload_file(&self.env, &file.id, &file.file_name, file_buffer)
            .await
            .map_err(|e| {
                tracing::error!("Failed to upload file: {:?}", e);
                Status::internal("Failed to upload file")
            })?;

        tx.commit().await.map_err(|e| {
            tracing::error!("Failed to commit transaction: {:?}", e);
            Status::internal("Failed to commit transaction")
        })?;

        tracing::info!("CreateFile: {:?}", start.elapsed());
        Ok(Response::new(file))
    }
    async fn delete_file(&self, request: Request<Id>) -> Result<Response<Empty>, Status> {
        let start = std::time::Instant::now();

        let mut conn = self.pool.get().await.map_err(|e| {
            tracing::error!("Failed to get connection from pool: {:?}", e);
            Status::internal("Failed to get connection from pool")
        })?;
        let tx = conn.transaction().await.map_err(|e| {
            tracing::error!("Failed to start transaction: {:?}", e);
            Status::internal("Failed to start transaction")
        })?;

        let request = request.into_inner();

        let file: File = crate::files_db::delete_file(&tx, &request.id)
            .await
            .map_err(|e| {
                tracing::error!("Failed to delete file: {:?}", e);
                Status::internal("Failed to delete file")
            })?;

        files_utils::delete_file(&self.env, &file.id, &file.file_name)
            .await
            .map_err(|e| {
                tracing::error!("Failed to delete file: {:?}", e);
                Status::internal("Failed to delete file")
            })?;

        tx.commit().await.map_err(|e| {
            tracing::error!("Failed to commit transaction: {:?}", e);
            Status::internal("Failed to commit transaction")
        })?;

        tracing::info!("DeleteFile: {:?}", start.elapsed());
        Ok(Response::new(Empty {}))
    }
}
