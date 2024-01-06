use crate::proto::{Count, Empty, File, Id, Page};
use tokio_stream::{wrappers::ReceiverStream, StreamExt};
use tonic::{Request, Response, Status};

pub async fn count_files_by_target_id(
    pool: &deadpool_postgres::Pool,
    request: Request<Empty>,
) -> Result<Response<Count>, Status> {
    let start = std::time::Instant::now();
    let metadata = request.metadata();
    let target_id = rusve_utils::auth(metadata)?.id;

    let conn = pool.get().await.map_err(|e| {
        tracing::error!("Failed to get connection: {:?}", e);
        Status::internal("Failed to get connection")
    })?;

    let count = crate::file_db::count_files_by_target_id(&conn, &target_id)
        .await
        .map_err(|e| {
            tracing::error!("Failed to count files: {:?}", e);
            Status::internal("Failed to count files")
        })?;

    tracing::info!("count_files_by_target_id: {:?}", start.elapsed());
    return Ok(Response::new(Count { count }));
}

pub async fn get_files_by_target_id(
    pool: &deadpool_postgres::Pool,
    request: Request<Page>,
) -> Result<Response<ReceiverStream<Result<File, Status>>>, Status> {
    let start = std::time::Instant::now();
    let metadata = request.metadata();
    let target_id = rusve_utils::auth(metadata)?.id;

    let conn = pool.get().await.map_err(|e| {
        tracing::error!("Failed to get connection from pool: {:?}", e);
        Status::internal("Failed to get connection from pool")
    })?;

    let request = request.into_inner();
    let files =
        crate::file_db::get_files_by_target_id(&conn, &target_id, request.offset, request.limit)
            .await
            .map_err(|e| {
                tracing::error!("Failed to get files by target id: {:?}", e);
                Status::internal("Failed to get files by target id")
            })?;

    let (tx, rx) = tokio::sync::mpsc::channel(128);
    tokio::spawn(async move {
        futures_util::pin_mut!(files);
        loop {
            let file = match tokio_stream::StreamExt::try_next(&mut files).await {
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
        tracing::info!("get_files_by_target_id: {:?}", start.elapsed());
    });
    Ok(Response::new(ReceiverStream::new(rx)))
}

pub async fn get_file_by_id(
    env: &rusve_utils::Env,
    pool: &deadpool_postgres::Pool,
    request: Request<Id>,
) -> Result<Response<File>, Status> {
    let start = std::time::Instant::now();
    let metadata = request.metadata();
    let target_id = rusve_utils::auth(metadata)?.id;

    let conn = pool.get().await.map_err(|e| {
        tracing::error!("Failed to get connection from pool: {:?}", e);
        Status::internal("Failed to get connection from pool")
    })?;

    let request = request.into_inner();
    let mut file = crate::file_db::get_file_by_id(&conn, &request.id, &target_id)
        .await
        .map_err(|e| {
            tracing::error!("Failed to get file by id: {:?}", e);
            Status::internal("Failed to get file by id")
        })?;

    let file_buffer = crate::file_utils::get_file_buffer(&env, &file.id, &file.file_name)
        .await
        .map_err(|e| {
            tracing::error!("Failed to get file buffer: {:?}", e);
            Status::internal("Failed to get file buffer")
        })?;

    file.file_buffer = file_buffer;

    tracing::info!("get_file_by_id: {:?}", start.elapsed());
    Ok(Response::new(file))
}

pub async fn upload_file(
    env: &rusve_utils::Env,
    pool: &deadpool_postgres::Pool,
    request: Request<tonic::Streaming<File>>,
) -> Result<Response<ReceiverStream<Result<File, Status>>>, Status> {
    let start = std::time::Instant::now();
    let metadata = request.metadata();
    let target_id = rusve_utils::auth(metadata)?.id;

    let mut conn = pool.get().await.map_err(|e| {
        tracing::error!("Failed to get connection from pool: {:?}", e);
        Status::internal("Failed to get connection from pool")
    })?;
    let tr = conn.transaction().await.map_err(|e| {
        tracing::error!("Failed to start transaction: {:?}", e);
        Status::internal("Failed to start transaction")
    })?;

    // get file from stream
    let mut stream = request.into_inner();
    let mut buffer = vec![];
    let mut file: File = Default::default();
    while let Some(file_stream) = stream.try_next().await.map_err(|e| {
        tracing::error!("Failed to get file stream: {:?}", e);
        Status::internal("Failed to get file stream")
    })? {
        buffer.extend(&file_stream.file_buffer);
        if file.file_name.is_empty() {
            file = file_stream;
        }
    }

    let file = crate::file_db::insert_file(&tr, &file, &target_id)
        .await
        .map_err(|e| {
            tracing::error!("Failed to create file: {:?}", e);
            Status::internal("Failed to create file")
        })?;

    crate::file_utils::upload_file(&env, &file.id, &file.file_name, buffer)
        .await
        .map_err(|e| {
            tracing::error!("Failed to upload file: {:?}", e);
            Status::internal("Failed to upload file")
        })?;

    tr.commit().await.map_err(|e| {
        tracing::error!("Failed to commit transaction: {:?}", e);
        Status::internal("Failed to commit transaction")
    })?;

    tracing::info!("upload_file: {:?}", start.elapsed());
    let (tx, rx) = tokio::sync::mpsc::channel(1);
    tx.send(Ok(file))
        .await
        .map_err(|e| Status::internal(format!("Failed to send file: {:?}", e)))?;
    Ok(Response::new(ReceiverStream::new(rx)))
}

pub async fn delete_file_by_id(
    env: &rusve_utils::Env,
    pool: &deadpool_postgres::Pool,
    request: Request<Id>,
) -> Result<Response<Empty>, Status> {
    let start = std::time::Instant::now();
    let metadata = request.metadata();
    let target_id = rusve_utils::auth(metadata)?.id;

    let mut conn = pool.get().await.map_err(|e| {
        tracing::error!("Failed to get connection from pool: {:?}", e);
        Status::internal("Failed to get connection from pool")
    })?;
    let tr = conn.transaction().await.map_err(|e| {
        tracing::error!("Failed to start transaction: {:?}", e);
        Status::internal("Failed to start transaction")
    })?;

    let request = request.into_inner();
    let file = crate::file_db::delete_file(&tr, &request.id, &target_id)
        .await
        .map_err(|e| {
            tracing::error!("Failed to delete file: {:?}", e);
            Status::internal("Failed to delete file")
        })?;

    crate::file_utils::delete_file(&env, &file.id, &file.file_name)
        .await
        .map_err(|e| {
            tracing::error!("Failed to delete file: {:?}", e);
            Status::internal("Failed to delete file")
        })?;

    tr.commit().await.map_err(|e| {
        tracing::error!("Failed to commit transaction: {:?}", e);
        Status::internal("Failed to commit transaction")
    })?;

    tracing::info!("delete_file_by_id: {:?}", start.elapsed());
    Ok(Response::new(Empty {}))
}
