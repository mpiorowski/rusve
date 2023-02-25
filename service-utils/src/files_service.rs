use crate::{
    proto::{files_service_server::FilesService, File, FileId, FileType, TargetId},
     MyService,
};
use anyhow::Result;
use futures_util::TryStreamExt;
use sqlx::types::time::OffsetDateTime;
use sqlx::{postgres::PgRow, query, types::Uuid, Row};
use tokio::sync::mpsc;
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
                data: Vec::new(),
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
        println!("GetFiles = {:?}", request);

        let start = std::time::Instant::now();
        let pool = self.pool.clone();

        let (tx, rx) = mpsc::channel(4);
        let target_id = request.into_inner();

        tokio::spawn(async move {
            let mut files_stream = query("SELECT * FROM files WHERE \"targetId\" = $1 and deleted is null order by created desc")
                .bind(&target_id.target_id)
                .fetch(&pool);

            loop {
                match files_stream.try_next().await {
                    Ok(None) => {
                        let elapsed = start.elapsed();
                        println!("Elapsed: {:.2?}", elapsed);
                        break;
                    }
                    Ok(file) => {
                        let file = map_file(file);
                        if let Err(e) = file {
                            tx.send(Err(Status::internal(e.to_string()))).await.unwrap();
                            break;
                        } else {
                            let file = file.unwrap();
                            tx.send(Ok(file)).await.unwrap();
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

    async fn create_file(&self, request: Request<File>) -> Result<Response<File>, Status> {
        #[cfg(debug_assertions)]
        println!("CreateFile = {:?}", request);
        let start = std::time::Instant::now();

        let pool = self.pool.clone();

        let file = request.into_inner();

        let row =
            query("INSERT INTO files (\"targetId\", name, type) VALUES ($1, $2, $3) RETURNING *")
                .bind(&file.target_id)
                .bind(&file.name)
                .bind(&file.r#type)
                .fetch_one(&pool)
                .await
                .map_err(|e| Status::internal(e.to_string()))?;

        let file = map_file(Some(row)).map_err(|e| Status::internal(e.to_string()))?;
        let elapsed = start.elapsed();
        println!("Elapsed: {:.2?}", elapsed);
        return Ok(Response::new(file));
    }

    async fn delete_file(&self, request: Request<FileId>) -> Result<Response<File>, Status> {
        println!("DeleteFile = {:?}", request);
        let start = std::time::Instant::now();

        let pool = self.pool.clone();

        let request = request.into_inner();
        let row = query("UPDATE files SET deleted = now() WHERE id = $1 RETURNING *")
            .bind(&request.file_id)
            .fetch_one(&pool)
            .await
            .map_err(|e| Status::not_found(e.to_string()))?;

        let file = map_file(Some(row)).map_err(|e| Status::internal(e.to_string()))?;
        let elapsed = start.elapsed();
        println!("Elapsed: {:.2?}", elapsed);
        return Ok(Response::new(file));
    }
}
