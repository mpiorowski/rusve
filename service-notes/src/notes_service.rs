use crate::{
    proto::{notes_service_server::NotesService, Note, NoteId, UserId},
    utils::create_auth_metadata,
    MyService,
};
use anyhow::Result;
use futures_util::TryStreamExt;
use sqlx::types::time::OffsetDateTime;
use sqlx::{postgres::PgRow, query, types::Uuid, Row};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};

trait SqlxError {
    fn into_status(self) -> Status;
}

impl SqlxError for sqlx::Error {
    fn into_status(self) -> Status {
        match self {
            sqlx::Error::Database(e) => Status::internal(e.message()),
            sqlx::Error::RowNotFound => Status::not_found("Note not found"),
            sqlx::Error::ColumnNotFound(_) => Status::not_found("Note not found"),
            _ => Status::internal("Unknown error"),
        }
    }
}

impl TryFrom<Option<PgRow>> for Note {
    type Error = anyhow::Error;

    fn try_from(row: Option<PgRow>) -> Result<Self, Self::Error> {
        match row {
            Some(row) => {
                let id: Uuid = row.try_get("id")?;
                let user_id: Uuid = row.try_get("userId")?;
                let created: OffsetDateTime = row.try_get("created")?;
                let updated: OffsetDateTime = row.try_get("updated")?;
                let deleted: Option<OffsetDateTime> = row.try_get("deleted")?;
                let title = row.try_get("title")?;
                let content = row.try_get("content")?;
                let note = Note {
                    id: id.to_string(),
                    user_id: user_id.to_string(),
                    title,
                    content,
                    created: created.to_string(),
                    updated: updated.to_string(),
                    deleted: deleted.map(|d| d.to_string()),
                    user: None,
                };
                Ok(note)
            }
            None => Err(anyhow::anyhow!("Note not found")),
        }
    }
}

#[tonic::async_trait]
impl NotesService for MyService {
    type GetNotesStream = ReceiverStream<Result<Note, Status>>;
    type GetOnlyNotesStream = ReceiverStream<Result<Note, Status>>;

    async fn get_notes(
        &self,
        request: Request<UserId>,
    ) -> Result<Response<Self::GetNotesStream>, Status> {
        #[cfg(debug_assertions)]
        println!("GetNotes = {:?}", request);
        let start = std::time::Instant::now();

        let pool = self.pool.clone();
        let (tx, rx) = mpsc::channel(4);

        // User service
        let mut users_conn = self.users_conn.clone();

        let user_id = request.into_inner().user_id;
        let user_id = Uuid::parse_str(&user_id).map_err(|e| Status::internal(e.to_string()))?;

        tokio::spawn(async move {
            let mut notes_stream = query("SELECT * FROM notes WHERE \"userId\" = $1 and deleted is null order by created desc")
                .bind(user_id)
                .fetch(&pool);

            loop {
                match notes_stream.try_next().await {
                    Ok(None) => {
                        let elapsed = start.elapsed();
                        println!("Elapsed: {:.2?}", elapsed);
                        break;
                    }
                    Ok(note) => {
                        let mut note: Note = match note.try_into() {
                            Ok(note) => note,
                            Err(e) => {
                                println!("Error: {:?}", e);
                                tx.send(Err(Status::internal(e.to_string()))).await.unwrap();
                                break;
                            }
                        };
                        // Get user
                        let auth_metadata = create_auth_metadata(&note.user_id);
                        if let Err(e) = auth_metadata {
                            println!("Error: {:?}", e);
                            tx.send(Err(Status::internal(e.to_string()))).await.unwrap();
                            break;
                        }
                        let request = Request::from_parts(
                            auth_metadata.unwrap(),
                            Default::default(),
                            UserId {
                                user_id: note.user_id.to_owned(),
                            },
                        );
                        let response = users_conn.get_user(request).await;
                        if let Err(e) = response {
                            tx.send(Err(Status::internal(e.to_string()))).await.unwrap();
                            break;
                        }
                        let response = response.unwrap();
                        let user = response.into_inner();
                        note.user = Some(user);
                        println!("note = {:?}", note);
                        tx.send(Ok(note)).await.unwrap();
                    }
                    Err(e) => {
                        println!("Error: {:?}", e);
                        tx.send(Err(Status::internal(e.to_string()))).await.unwrap();
                        break;
                    }
                }
            }
        });
        Ok(Response::new(ReceiverStream::new(rx)))
    }

    async fn get_only_notes(
        &self,
        request: Request<UserId>,
    ) -> Result<Response<Self::GetOnlyNotesStream>, Status> {
        #[cfg(debug_assertions)]
        println!("GetNotes = {:?}", request);
        let start = std::time::Instant::now();

        let pool = self.pool.clone();
        let (tx, rx) = mpsc::channel(100);

        let user_id = request.into_inner().user_id;
        let user_id = Uuid::parse_str(&user_id).map_err(|e| Status::internal(e.to_string()))?;

        let mut notes_stream = query(
            "SELECT * FROM notes WHERE \"userId\" = $1 and deleted is null order by created desc",
        )
        .bind(user_id)
        .fetch(&pool);

        tokio::spawn(async move {
            loop {
                match notes_stream.try_next().await {
                    Ok(None) => {
                        let elapsed = start.elapsed();
                        println!("Elapsed: {:.2?}", elapsed);
                        break;
                    }
                    Ok(note) => {
                        let note: Note = match note.try_into() {
                            Ok(note) => note,
                            Err(e) => {
                                tx.send(Err(Status::internal(e.to_string()))).await.unwrap();
                                break;
                            }
                        };
                        tx.send(Ok(note)).await.unwrap();
                    }
                    Err(e) => {
                        println!("Error: {:?}", e);
                        tx.send(Err(Status::internal(e.to_string()))).await.unwrap();
                        break;
                    }
                }
            }
        });
        Ok(Response::new(ReceiverStream::new(rx)))
    }

    async fn create_note(&self, request: Request<Note>) -> Result<Response<Note>, Status> {
        #[cfg(debug_assertions)]
        println!("CreateNote = {:?}", request);
        let start = std::time::Instant::now();

        // start transaction
        let pool = self.pool.clone();
        let mut tx = pool.begin().await.map_err(sqlx::Error::into_status)?;

        let note = request.into_inner();
        let user_id =
            Uuid::parse_str(&note.user_id).map_err(|e| Status::internal(e.to_string()))?;

        // do it 100 times
        query("INSERT INTO notes (title, content, \"userId\") VALUES ($1, $2, $3) RETURNING *")
            .bind(note.title.clone())
            .bind(note.content.clone())
            .bind(user_id)
            .fetch_one(&mut tx)
            .await
            .map_err(sqlx::Error::into_status)?;

        // commit transaction
        tx.commit().await.map_err(sqlx::Error::into_status)?;

        let elapsed = start.elapsed();
        println!("Elapsed: {:.2?}", elapsed);
        return Ok(Response::new(note));
    }

    async fn delete_note(&self, request: Request<NoteId>) -> Result<Response<Note>, Status> {
        println!("DeleteNote = {:?}", request);
        let start = std::time::Instant::now();

        // start transaction
        let pool = self.pool.clone();
        let mut tx = pool.begin().await.map_err(sqlx::Error::into_status)?;

        let request = request.into_inner();
        let note_uuid =
            Uuid::parse_str(&request.note_id).map_err(|e| Status::internal(e.to_string()))?;
        let user_uuid =
            Uuid::parse_str(&request.user_id).map_err(|e| Status::internal(e.to_string()))?;

        let row =
            query("UPDATE notes SET deleted = NOW() WHERE id = $1 AND \"userId\" = $2 RETURNING *")
                .bind(note_uuid)
                .bind(user_uuid)
                .fetch_one(&mut tx)
                .await
                .map_err(sqlx::Error::into_status)?;

        let note: Note = match Some(row).try_into() {
            Ok(note) => note,
            Err(e) => {
                return Err(Status::internal(e.to_string()));
            }
        };

        // commit transaction
        tx.commit().await.map_err(sqlx::Error::into_status)?;

        let elapsed = start.elapsed();
        println!("Elapsed: {:.2?}", elapsed);
        return Ok(Response::new(note));
    }
}
