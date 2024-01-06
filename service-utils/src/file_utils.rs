use anyhow::Result;
use rusve_utils::Env;

async fn connect_to_bucket(env: &Env) -> Result<s3::Bucket> {
    let s3_access_key = env.s3_access_key.clone();
    let s3_secret_key = env.s3_secret_key.clone();
    let s3_endpoint = env.s3_endpoint.clone();
    let s3_bucket_name = env.s3_bucket_name.clone();

    let credentials = s3::creds::Credentials::new(
        Option::from(s3_access_key).as_deref(), // access_key
        Option::from(s3_secret_key).as_deref(), // secret_key
        None,
        None,
        None,
    )?;

    let region = s3::Region::Custom {
        region: "auto".to_owned(),
        endpoint: s3_endpoint,
    };

    let bucket = s3::Bucket::new(&s3_bucket_name, region, credentials)?.with_path_style();
    Ok(bucket)
}

pub async fn get_file_buffer(env: &Env, file_id: &str, file_name: &str) -> Result<Vec<u8>> {
    let file_path = format!("{}/{}", &file_id, &file_name);
    let bucket = connect_to_bucket(&env).await?;
    let buffer = bucket.get_object(file_path).await?.into();
    Ok(buffer)
}

pub async fn upload_file(
    env: &Env,
    file_id: &str,
    file_name: &str,
    file_buffer: Vec<u8>,
) -> Result<()> {
    let file_path = format!("{}/{}", &file_id, &file_name);
    let bucket = connect_to_bucket(&env).await?;
    bucket.put_object(file_path, &file_buffer).await?;
    Ok(())
}

pub async fn delete_file(env: &Env, file_id: &str, file_name: &str) -> Result<()> {
    let file_path = format!("{}/{}", &file_id, &file_name);
    let bucket = connect_to_bucket(&env).await?;
    bucket.delete_object(file_path).await?;
    Ok(())
}
