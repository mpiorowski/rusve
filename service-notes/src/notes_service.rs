use crate::{
    notes_db::{delete_note, get_notes_by_user_uuid, upsert_note},
    proto::{notes_service_server::NotesService, Empty, Note, NoteId, UserId},
    MyService,
};
use anyhow::Result;
use futures_util::StreamExt;
use mysql_async::prelude::WithParams;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};
use uuid::Uuid;

use mysql_async::prelude::*;

use crate::models::*;

struct MysqlNote {
    pub id: Vec<u8>,
    pub created: time::Duration,
    pub updated: time::Duration,
    pub deleted: Option<time::Duration>,
    pub user_id: Vec<u8>,
    pub title: String,
    pub content: String,
}

impl TryFrom<DieselNote> for Note {
    type Error = anyhow::Error;

    fn try_from(note: DieselNote) -> Result<Self, Self::Error> {
        let note = Note {
            id: note.id,
            user_id: note.user_id,
            title: note.title,
            content: note.content,
            created: note.created.to_string(),
            updated: note.updated.to_string(),
            deleted: note.deleted.map(|d| d.to_string()),
        };
        Ok(note)
    }
}

impl TryFrom<MysqlNote> for Note {
    type Error = anyhow::Error;

    fn try_from(note: MysqlNote) -> Result<Self, Self::Error> {
        let note = Note {
            id: note.id,
            user_id: note.user_id,
            title: note.title,
            content: note.content,
            created: note.created.to_string(),
            updated: note.updated.to_string(),
            deleted: note.deleted.map(|d| d.to_string()),
        };
        Ok(note)
    }
}

#[tonic::async_trait]
impl NotesService for MyService {
    type GetNotesStream = ReceiverStream<Result<Note, Status>>;

    async fn get_notes(
        &self,
        request: Request<UserId>,
    ) -> Result<Response<Self::GetNotesStream>, Status> {
        #[cfg(debug_assertions)]
        println!("GetNotes");
        let start = std::time::Instant::now();

        let conn = self
            .pool
            .get()
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        let mut mysql_conn = self
            .mysql_pool
            .get_conn()
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        println!("Connect: {:?}", start.elapsed());

        let request = request.into_inner();

        impl FromRow for MysqlNote {
            fn from_row_opt(
                row: mysql_async::Row,
            ) -> std::result::Result<Self, mysql_async::FromRowError> {
                let note = mysql_async::from_row::<MysqlNote>(row);
                Ok(MysqlNote {
                    id: note.id,
                    created: note.created,
                    updated: note.updated,
                    deleted: note.deleted,
                    user_id: note.user_id,
                    title: note.title,
                    content: note.content,
                })
            }
        }

        impl FromRow for Note {
            fn from_row_opt(
                row: mysql_async::Row,
            ) -> std::result::Result<Self, mysql_async::FromRowError> {
                let note = mysql_async::from_row::<MysqlNote>(row);
                Ok(Note {
                    id: note.id,
                    created: note.created.to_string(),
                    updated: note.updated.to_string(),
                    deleted: note.deleted.map(|d| d.to_string()),
                    user_id: note.user_id,
                    title: note.title,
                    content: note.content,
                })
            }
        }

        let notes = get_notes_by_user_uuid(conn, request.user_id)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        // let notes = "SELECT * FROM notes where user_id = :user_id"
        //     .with(params! {
        //         "user_id" => request.user_id,
        //     })
        //     .map(
        //         &mut mysql_conn,
        //         |(id, created, updated, deleted, user_id, title, content)| MysqlNote {
        //             id,
        //             created,
        //             updated,
        //             deleted,
        //             user_id,
        //             title,
        //             content,
        //         },
        //     )
        //     .await
        //     .map_err(|e| Status::internal(e.to_string()))?;

        // let mut rows = "SELECT * FROM notes where user_id = :user_id"
        //     .with(params! {
        //         "user_id" => request.user_id,
        //     })
        //     .stream::<Note, _>(mysql_conn)
        //     .await
        //     .map_err(|e| Status::internal(e.to_string()))?;

        println!("Query: {:?}", start.elapsed());

        let (tx, rx) = mpsc::channel(128);
        tokio::spawn(async move {
            // while let Some(note) = notes.next().await {
            for note in notes {
                // let note = match note {
                //     Ok(note) => note,
                //     Err(e) => {
                //         tx.send(Err(Status::internal(e.to_string()))).await.unwrap();
                //         break;
                //     }
                // };
                let note: Note = match Note::try_from(note) {
                    Ok(note) => note,
                    Err(e) => {
                        tx.send(Err(Status::internal(e.to_string()))).await.unwrap();
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
            println!("Elapsed: {:.2?}", start.elapsed());
        });

        // let notes = get_notes_by_user_uuid(conn, request.user_id).await?;

        // println!("Prepare: {:?}", start.elapsed());

        // let (tx, rx) = mpsc::channel(128);
        // tokio::spawn(async move {
        //     // while let Some(row) = rows.next().await {
        //     for note in notes {
        //         // let note = match row {
        //         //     Ok(note) => note,
        //         //     Err(e) => {
        //         //         tx.send(Err(Status::internal(e.to_string()))).await.unwrap();
        //         //         break;
        //         //     }
        //         // };
        //         let note: Note = match Note::try_from(note) {
        //             Ok(note) => note,
        //             Err(e) => {
        //                 tx.send(Err(Status::internal(e.to_string()))).await.unwrap();
        //                 break;
        //             }
        //         };
        //         match tx.send(Ok(note)).await {
        //             Ok(_) => {}
        //             Err(e) => {
        //                 println!("Error: {:?}", e);
        //                 break;
        //             }
        //         }
        //     }
        //     println!("Elapsed: {:.2?}", start.elapsed());
        // });
        Ok(Response::new(ReceiverStream::new(rx)))
    }

    async fn create_note(&self, request: Request<Note>) -> Result<Response<Empty>, Status> {
        #[cfg(debug_assertions)]
        println!("CreateNote");
        let start = std::time::Instant::now();

        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        let note = request.into_inner();

        for _ in 0..5000 {
            let new_note = UpsertNote {
                id: &Uuid::now_v7().as_bytes().to_vec(),
                user_id: &note.user_id,
                title: &note.title,
                content: &note.content,
            };
            upsert_note(&mut conn, new_note).await?;
        }

        println!("Elapsed: {:.2?}", start.elapsed());
        return Ok(Response::new(Empty {}));
    }

    async fn delete_note(&self, request: Request<NoteId>) -> Result<Response<Empty>, Status> {
        #[cfg(debug_assertions)]
        println!("DeleteNote");
        let start = std::time::Instant::now();

        let conn = self
            .pool
            .get()
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        let request = request.into_inner();

        delete_note(conn, request.note_id, request.user_id).await?;

        println!("Elapsed: {:.2?}", start.elapsed());
        return Ok(Response::new(Empty {}));
    }
}
