use deadpool::managed::Object;
use diesel::QueryResult;
use diesel_async::{pooled_connection::AsyncDieselConnectionManager, AsyncPgConnection};
use futures_core::Stream;
use tonic::Status;
use uuid::Uuid;

use crate::{
    models::{DieselNote, UpsertNote},
    schema::notes::dsl::*,
};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;

pub async fn get_notes_by_user_uuid(
    mut conn: Object<AsyncDieselConnectionManager<AsyncPgConnection>>,
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

pub async fn upsert_note(
    mut conn: Object<AsyncDieselConnectionManager<AsyncPgConnection>>,
    new_note: UpsertNote<'_>,
) -> Result<DieselNote, Status> {
    let note = diesel::insert_into(notes)
        .values(&new_note)
        .on_conflict(id)
        .do_update()
        .set((title.eq(new_note.title), content.eq(new_note.content)))
        .get_result::<DieselNote>(&mut conn)
        .await
        .map_err(|e| Status::internal(e.to_string()))?;
    Ok(note)
}

pub async fn delete_note(
    mut conn: Object<AsyncDieselConnectionManager<AsyncPgConnection>>,
    user_uuid: Uuid,
    note_uuid: Uuid,
) -> Result<DieselNote, Status> {
    let note = diesel::update(notes)
        .filter(id.eq(note_uuid))
        .filter(user_id.eq(user_uuid))
        .set(deleted.eq(diesel::dsl::now))
        .get_result::<DieselNote>(&mut conn)
        .await
        .map_err(|e| Status::internal(e.to_string()))?;
    Ok(note)
}
