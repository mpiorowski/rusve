use diesel_async::{pooled_connection::deadpool::Object, AsyncMysqlConnection};
use tonic::Status;

// use futures_core::Stream;
// use diesel::QueryResult;

use crate::{
    models::{DieselNote, UpsertNote},
    schema::notes::dsl::*,
};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;

pub async fn get_notes_by_user_uuid(
    mut conn: Object<AsyncMysqlConnection>,
    user_uuid: Vec<u8>,
) -> Result<Vec<DieselNote>, Status> {
    let note = notes
        .select(DieselNote::as_select())
        .filter(user_id.eq(user_uuid))
        .filter(deleted.is_null())
        .load(&mut conn)
        .await
        .map_err(|e| Status::internal(e.to_string()))?;
    Ok(note)
}

// pub async fn get_notes_by_user_uuid_stream_stream(
//     mut conn: Object<AsyncMysqlConnection>,
//     user_uuid: Vec<u8>,
// ) -> Result<impl Stream<Item = QueryResult<DieselNote>>, Status> {
//     let note = notes
//         .select(DieselNote::as_select())
//         .filter(user_id.eq(user_uuid))
//         .filter(deleted.is_null())
//         .load_stream(&mut conn)
//         .await
//         .map_err(|e| Status::internal(e.to_string()))?;
//     Ok(note)
// }

pub async fn upsert_note(
    conn: &mut Object<AsyncMysqlConnection>,
    new_note: UpsertNote<'_>,
) -> Result<(), Status> {
    // diesel::delete(notes)
    //     .filter(user_id.eq(new_note.user_id))
    //     .execute(conn)
    //     .await
    //     .map_err(|e| Status::internal(e.to_string()))?;
    diesel::insert_into(notes)
        .values(&new_note)
        .execute(conn)
        .await
        .map_err(|e| Status::internal(e.to_string()))?;
    Ok(())
}

pub async fn delete_note(
    mut conn: Object<AsyncMysqlConnection>,
    user_uuid: Vec<u8>,
    note_uuid: Vec<u8>,
) -> Result<(), Status> {
    diesel::update(notes)
        .filter(id.eq(note_uuid))
        .filter(user_id.eq(user_uuid))
        .set(deleted.eq(diesel::dsl::now))
        .execute(&mut conn)
        .await
        .map_err(|e| Status::internal(e.to_string()))?;
    Ok(())
}
