
pub async fn get_files_by_target_id(
    mut conn: Object<AsyncPgConnection>,
    user_uuid: Uuid,
) -> Result<impl Stream<Item = QueryResult<DieselNote>>, Status> {
    let note = notes
        .filter(deleted.is_null())
        .filter(user_id.eq(&user_uuid))
        .order(created.desc())
        .select(DieselNote::as_select())
        .load_stream(&mut conn)
        .await
        .map_err(|e| Status::internal(e.to_string()))?;
    Ok(note)
}
