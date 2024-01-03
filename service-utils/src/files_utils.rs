use anyhow::{Context, Result};
use rusve_users::{connect_to_bucket, Env};
use tokio::io::AsyncWriteExt;

pub async fn get_file_buffer(env: &Env, file_id: &str, file_name: &str) -> Result<Vec<u8>> {
    let target = env.target.clone();
    let file_path = format!("{}/{}", &file_id, &file_name);

    let mut buffer = Vec::new();
    if target == "development" {
        let file_path = format!("/app/files/{}", &file_path);
        buffer = std::fs::read(file_path).context("Failed to read file")?;
        return Ok(buffer);
    } else if target == "production" {
        let bucket = connect_to_bucket(&env).await?;
        buffer = bucket.get_object(file_path).await?.into();
    }
    Ok(buffer)
}

pub async fn upload_file(
    env: &Env,
    file_id: &str,
    file_name: &str,
    file_buffer: Vec<u8>,
) -> Result<()> {
    let target = env.target.clone();
    let file_path = format!("{}/{}", &file_id, &file_name);

    if target == "development" {
        // save file to disk
        tokio::fs::create_dir_all(format!("/app/files/{}", &file_id)).await?;
        let file_path = format!("/app/files/{}", &file_path);
        let mut new_file = tokio::fs::File::create(file_path).await?;
        new_file.write_all(&file_buffer).await?;
    } else if target == "production" {
        // save to bucket
        let bucket = connect_to_bucket(&env).await?;
        bucket.put_object(file_path, &file_buffer).await?;
    }
    Ok(())
}

pub async fn delete_file(env: &Env, file_id: &str, file_name: &str) -> Result<()> {
    let target = env.target.clone();
    let file_path = format!("{}/{}", &file_id, &file_name);
    if target == "development" {
        // delete file from disk
        tokio::fs::remove_dir_all(format!("/app/files/{}", &file_id)).await?;
    } else if target == "production" {
        // delete from storage
        let bucket = connect_to_bucket(&env).await?;
        bucket.delete_object(file_path).await?;
    }
    Ok(())
}
