use crate::{
    proto::{notes_service_server::NotesService, Note, NoteId, UserId},
    MyService,
};
use anyhow::Result;
use deadpool::managed::Object;
use futures_util::{Stream, StreamExt};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};
use uuid::Uuid;

use crate::models::*;
use crate::schema::notes::dsl::*;
use diesel::prelude::*;
use diesel_async::{
    pooled_connection::AsyncDieselConnectionManager, AsyncPgConnection, RunQueryDsl,
};

impl TryFrom<DieselNote> for Note {
    type Error = anyhow::Error;

    fn try_from(note: DieselNote) -> Result<Self, Self::Error> {
        let note = Note {
            id: note.id.to_string(),
            user_id: note.user_id.to_string(),
            title: note.title,
            content: note.content,
            created: note.created.to_string(),
            updated: note.updated.to_string(),
            deleted: note.deleted.map(|d| d.to_string()),
            user: None,
        };
        Ok(note)
    }
}

async fn get_notes(
    mut conn: Object<AsyncDieselConnectionManager<AsyncPgConnection>>,
    user_uuid: Uuid,
) -> Result<impl Stream<Item = QueryResult<DieselNote>>, Status> {
    return Ok(notes
        .filter(deleted.is_null())
        .filter(user_id.eq(&user_uuid))
        .order(created.desc())
        .select(DieselNote::as_select())
        .load_stream(&mut conn)
        .await
        .map_err(|e| Status::internal(e.to_string()))?);
}

#[tonic::async_trait]
impl NotesService for MyService {
    type GetNotesStream = ReceiverStream<Result<Note, Status>>;

    async fn get_notes(
        &self,
        request: Request<UserId>,
    ) -> Result<Response<Self::GetNotesStream>, Status> {
        #[cfg(debug_assertions)]
        println!("GetNotes = {:?}", request);
        let start = std::time::Instant::now();

        let conn = self
            .pool
            .get()
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        println!("Connect: {:?}", start.elapsed());

        let user_uuid = request.into_inner().user_id;
        let user_uuid = Uuid::parse_str(&user_uuid).map_err(|e| Status::internal(e.to_string()))?;

        let mut rows = get_notes(conn, user_uuid).await?;

        println!("Prepare: {:?}", start.elapsed());

        let (tx, rx) = mpsc::channel(128);
        tokio::spawn(async move {
            while let Some(row) = rows.next().await {
                let note = match row {
                    Ok(note) => note,
                    Err(e) => {
                        println!("Error: {:?}", e);
                        break;
                    }
                };
                let note: Note = match Note::try_from(note) {
                    Ok(note) => note,
                    Err(e) => {
                        println!("Error: {:?}", e);
                        break;
                    }
                };
                match tx.send(Ok(note)).await {
                    Ok(_) => {}
                    Err(e) => {
                        println!("Error: {:?}", e);
                        break;
                    }
                }
            }
            // for row in rows {
            //     let note = match Note::try_from(row) {
            //         Ok(note) => note,
            //         Err(e) => {
            //             println!("Error: {:?}", e);
            //             break;
            //         }
            //     };
            //     match tx.send(Ok(note)).await {
            //         Ok(_) => {}
            //         Err(e) => {
            //             println!("Error: {:?}", e);
            //             break;
            //         }
            //     }
            // }
            println!("Elapsed: {:.2?}", start.elapsed());
        });
        Ok(Response::new(ReceiverStream::new(rx)))
    }

    async fn create_note(&self, request: Request<Note>) -> Result<Response<Note>, Status> {
        #[cfg(debug_assertions)]
        println!("CreateNote = {:?}", request);
        let start = std::time::Instant::now();

        let mut con = self
            .pool
            .get()
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        let note = request.into_inner();
        let user_uuid =
            Uuid::parse_str(&note.user_id).map_err(|e| Status::internal(e.to_string()))?;

        let new_note = NewNote {
            user_id: &user_uuid,
            title: &note.title,
            content: &note.content,
        };

        let note = diesel::insert_into(notes)
            .values(&new_note)
            .get_result::<DieselNote>(&mut con)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        let note = Note::try_from(note).map_err(|e| Status::internal(e.to_string()))?;

        println!("Elapsed: {:.2?}", start.elapsed());
        return Ok(Response::new(note));
    }

    async fn delete_note(&self, request: Request<NoteId>) -> Result<Response<Note>, Status> {
        println!("DeleteNote = {:?}", request);
        let start = std::time::Instant::now();

        let mut con = self
            .pool
            .get()
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        let request = request.into_inner();
        let note_uuid =
            Uuid::parse_str(&request.note_id).map_err(|e| Status::internal(e.to_string()))?;
        let user_uuid =
            Uuid::parse_str(&request.user_id).map_err(|e| Status::internal(e.to_string()))?;

        let note = diesel::update(notes)
            .filter(id.eq(note_uuid))
            .filter(user_id.eq(user_uuid))
            // .set(deleted.eq(diesel::dsl::now))
            .set(title.eq("deleted"))
            .get_result::<DieselNote>(&mut con)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        let note = Note::try_from(note).map_err(|e| Status::internal(e.to_string()))?;

        println!("Elapsed: {:.2?}", start.elapsed());
        return Ok(Response::new(note));
    }
}
