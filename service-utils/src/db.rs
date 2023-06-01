use diesel::ExpressionMethods;
use diesel::QueryResult;
use diesel_async::RunQueryDsl;
use diesel_async::{pooled_connection::deadpool::Object, AsyncPgConnection};
use futures_util::Stream;
use tonic::Status;
use uuid::Uuid;

use crate::models::InsertFile;
use crate::{models::DieselFile, schema::files::dsl::*};
use diesel::prelude::*;

pub async fn get_files_by_target_id(
    mut conn: Object<AsyncPgConnection>,
    target_uuid: Uuid,
    r#type: &str,
) -> Result<impl Stream<Item = QueryResult<DieselFile>>, Status> {
    let data = files
        .filter(deleted.is_null())
        .filter(target_id.eq(target_uuid))
        .filter(type_.eq(r#type))
        .order(created.desc())
        .select(DieselFile::as_select())
        .load_stream(&mut conn)
        .await
        .map_err(|e| Status::internal(e.to_string()))?;
    Ok(data)
}

pub async fn get_file_by_id(
    mut conn: Object<AsyncPgConnection>,
    file_uuid: Uuid,
    target_uuid: Uuid,
) -> Result<DieselFile, Status> {
    let data = files
        .filter(deleted.is_null())
        .filter(id.eq(file_uuid))
        .filter(target_id.eq(target_uuid))
        .order(created.desc())
        .select(DieselFile::as_select())
        .get_result(&mut conn)
        .await
        .map_err(|e| Status::internal(e.to_string()))?;
    Ok(data)
}

pub async fn create_file(
    mut conn: Object<AsyncPgConnection>,
    file: InsertFile<'_>,
) -> Result<DieselFile, Status> {
    let data = diesel::insert_into(files)
        .values(file)
        .get_result(&mut conn)
        .await
        .map_err(|e| Status::internal(e.to_string()))?;
    Ok(data)
}

pub async fn delete_file(
    mut conn: Object<AsyncPgConnection>,
    file_uuid: Uuid,
    target_uuid: Uuid,
) -> Result<DieselFile, Status> {
    let data = diesel::update(files)
        .filter(id.eq(file_uuid))
        .filter(target_id.eq(target_uuid))
        .set(deleted.eq(time::OffsetDateTime::now_utc()))
        .get_result(&mut conn)
        .await
        .map_err(|e| Status::internal(e.to_string()))?;
    Ok(data)
}
