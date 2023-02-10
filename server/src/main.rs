mod proto;
mod utils;

use anyhow::Result;
use futures::TryStreamExt;
use sqlx::Row;
use proto::{
    notes_service_server::{NotesService, NotesServiceServer},
    Note, NoteId, UserId,
};
use sqlx::{postgres::PgPoolOptions, query};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{transport::Server, Request, Response, Status};
use utils::{check_env, connect_db};

#[derive(Debug, Default)]
pub struct MyNotes {}

#[tonic::async_trait]
impl NotesService for MyNotes {
    type GetNotesStream = ReceiverStream<Result<Note, Status>>;

    async fn get_notes(
        &self,
        request: Request<UserId>,
    ) -> Result<Response<Self::GetNotesStream>, Status> {
        println!("ListFeatures = {:?}", request);

        let pool = connect_db()
            .await
            .or_else(|e| Err(Status::internal(e.to_string())))?;
        let (tx, rx) = mpsc::channel(4);

        let user_id = request.into_inner().user_id;

        tokio::spawn(async move {
            let mut notes_stream = query("SELECT * FROM notes WHERE user_id = $1")
                .bind(user_id)
                .fetch(&pool);
            while let Some(note) = notes_stream.try_next().await.unwrap() {
                let id = note.try_get("id").unwrap();
                let user_id = note.try_get("user_id").unwrap();
                let title = note.try_get("title").unwrap();
                let content = note.try_get("content").unwrap();
                let created = note.try_get("created").unwrap();
                let updated = note.try_get("updated").unwrap();
                let deleted = note.try_get("deleted").unwrap();
                let note = Note {
                    id,
                    user_id,
                    title,
                    content,
                    created,
                    updated,
                    deleted,
                    user: None,
                };
                tx.send(Ok(note)).await.unwrap();
            }
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }
    async fn create_note(&self, request: Request<Note>) -> Result<Response<Note>, Status> {
        todo!()
    }
    async fn delete_note(&self, request: Request<NoteId>) -> Result<Response<Note>, Status> {
        unimplemented!()
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting server...");
    // Create a connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:12345@db-notes/notes")
        .await?;
    println!("Connected to database");

    let port = check_env("PORT")?;
    let addr = ("0.0.0.0:".to_owned() + &port).parse()?;
    let notes = MyNotes::default();

    Server::builder()
        .add_service(NotesServiceServer::new(notes))
        .serve(addr)
        .await?;

    Ok(())
}
