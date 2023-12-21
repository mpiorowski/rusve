use anyhow::{Context, Result};
use google_cloud_default::WithAuthExt;
use google_cloud_storage::{
    client::{Client, ClientConfig},
    http::objects::{
        delete::DeleteObjectRequest,
        download::Range,
        get::GetObjectRequest,
        upload::{Media, UploadObjectRequest, UploadType},
    },
};
use tokio::io::AsyncWriteExt;

pub async fn get_file_buffer(file_id: &str, file_name: &str) -> Result<Vec<u8>> {
    let env = std::env::var("ENV").context("ENV is not set")?;
    let bucket = std::env::var("BUCKET").context("BUCKET is not set")?;

    let mut buffer = Vec::new();
    if env == "development" {
        let file_path = format!("/app/files/{}/{}", file_id, file_name);
        buffer = std::fs::read(file_path).context("Failed to read file")?;
        return Ok(buffer);
    } else if env == "production" {
        let config = ClientConfig::default().with_auth().await?;
        let client = Client::new(config);
        buffer = client
            .download_object(
                &GetObjectRequest {
                    bucket,
                    object: format!("{}/{}", file_id, file_name),
                    ..Default::default()
                },
                &Range::default(),
            )
            .await?;
    }
    Ok(buffer)
}

pub async fn upload_file(file_id: &str, file_name: &str, file_buffer: Vec<u8>) -> Result<()> {
    let env = std::env::var("ENV").context("ENV is not set")?;
    let bucket = std::env::var("BUCKET").context("BUCKET is not set")?;
    if env == "development" {
        // save file to disk
        tokio::fs::create_dir_all(format!("/app/files/{}", &file_id)).await?;
        let file_path = format!("/app/files/{}/{}", &file_id, &file_name);
        let mut new_file = tokio::fs::File::create(file_path).await?;
        new_file.write_all(&file_buffer).await?;
    } else if env == "production" {
        // save to GCP storage
        let config = ClientConfig::default().with_auth().await?;
        let client = Client::new(config);
        let file_path = format!("{}/{}", &file_id, &file_name);
        let upload_type = UploadType::Simple(Media::new(file_path));
        client
            .upload_object(
                &UploadObjectRequest {
                    bucket: bucket.to_string(),
                    ..Default::default()
                },
                file_buffer,
                &upload_type,
            )
            .await?;
    }
    Ok(())
}

pub async fn delete_file(file_id: &str, file_name: &str) -> Result<()> {
    let env = std::env::var("ENV").context("ENV is not set")?;
    let bucket = std::env::var("BUCKET").context("BUCKET is not set")?;
    if env == "development" {
        // delete file from disk
        tokio::fs::remove_dir_all(format!("/app/files/{}", &file_id)).await?;
    } else if env == "production" {
        // delete from GCP storage
        let config = ClientConfig::default().with_auth().await?;
        let client = Client::new(config);
        client
            .delete_object(&DeleteObjectRequest {
                bucket: bucket.to_string(),
                object: format!("{}/{}", &file_id, &file_name),
                ..Default::default()
            })
            .await?;
    }
    Ok(())
}
