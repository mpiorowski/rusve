use anyhow::Result;
use sendgrid::{Destination, Mail, SGClient};
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};

use crate::proto::{Count, Email, Empty, Page};

pub async fn count_emails_by_target_id(
    pool: &deadpool_postgres::Pool,
    request: Request<Empty>,
) -> Result<Response<Count>, Status> {
    let start = std::time::Instant::now();
    let metadata = request.metadata();
    let target_id = rusve_utils::auth(metadata)?.id;

    let conn = pool.get().await.map_err(|e| {
        tracing::error!("Failed to get connection: {:?}", e);
        Status::internal("Failed to get connection")
    })?;

    let count = crate::email_db::count_emails_by_target_id(&conn, &target_id)
        .await
        .map_err(|e| {
            tracing::error!("Failed to count emails: {:?}", e);
            Status::internal("Failed to count emails")
        })?;

    tracing::info!("count_emails_by_target_id: {:?}", start.elapsed());
    Ok(Response::new(Count { count }))
}

pub async fn get_emails_by_target_id(
    pool: &deadpool_postgres::Pool,
    request: Request<Page>,
) -> Result<Response<ReceiverStream<Result<crate::proto::Email, Status>>>, Status> {
    let start = std::time::Instant::now();
    let metadata = request.metadata();
    let target_id = rusve_utils::auth(metadata)?.id;

    let conn = pool.get().await.map_err(|e| {
        tracing::error!("Failed to get connection: {:?}", e);
        Status::internal("Failed to get connection")
    })?;

    let page = request.into_inner();
    let emails_stream =
        crate::email_db::get_emails_by_target_id(&conn, &target_id, page.offset, page.limit)
            .await
            .map_err(|e| {
                tracing::error!("Failed to get emails: {:?}", e);
                Status::internal("Failed to get emails")
            })?;

    let (tx, rx) = tokio::sync::mpsc::channel(128);
    tokio::spawn(async move {
        futures_util::pin_mut!(emails_stream);
        while let Ok(Some(note)) = tokio_stream::StreamExt::try_next(&mut emails_stream).await {
            let tx = tx.clone();
            tokio::spawn(async move {
                let email: Email = match note.try_into() {
                    Ok(note) => note,
                    Err(e) => {
                        tracing::error!("Failed to get note: {:?}", e);
                        return;
                    }
                };
                if let Err(e) = tx.send(Ok(email)).await {
                    tracing::error!("Failed to send email: {:?}", e);
                }
            });
        }
        tracing::info!("get_emails_by_target_id: {:?}", start.elapsed());
    });
    Ok(Response::new(tokio_stream::wrappers::ReceiverStream::new(
        rx,
    )))
}

pub async fn send_email(
    env: &rusve_utils::Env,
    pool: &deadpool_postgres::Pool,
    request: Request<Email>,
) -> Result<Response<Email>, Status> {
    let start = std::time::Instant::now();
    let metadata = request.metadata();
    let target_id = rusve_utils::auth(metadata)?.id;

    let email = request.into_inner();
    crate::email_validation::Validation::validate(&email)?;

    let mut conn = pool.get().await.map_err(|e| {
        tracing::error!("Failed to get connection: {:?}", e);
        Status::internal("Failed to get connection")
    })?;
    let tr = conn.transaction().await.map_err(|e| {
        tracing::error!("Failed to start transaction: {:?}", e);
        Status::internal("Failed to start transaction")
    })?;

    let email = crate::email_db::insert_email(&tr, &target_id, &email)
        .await
        .map_err(|e| {
            tracing::error!("Failed to insert email: {:?}", e);
            Status::internal("Failed to insert email")
        })?;

    let sg = SGClient::new(env.sendgrid_api_key.as_str());
    let mail_info = Mail::new()
        .add_to(Destination {
            address: email.email_to.as_str(),
            name: email.email_to.as_str(),
        })
        .add_from(email.email_from.as_str())
        .add_from_name(email.email_from_name.as_str())
        .add_subject(email.email_subject.as_str())
        .add_html(email.email_body.as_str());

    sg.send(mail_info).await.map_err(|e| {
        tracing::error!("Failed to send email: {:?}", e);
        Status::internal("Failed to send email")
    })?;

    tr.commit().await.map_err(|e| {
        tracing::error!("Failed to commit transaction: {:?}", e);
        Status::internal("Failed to commit transaction")
    })?;

    tracing::info!("send_email: {:?}", start.elapsed());
    Ok(Response::new(email))
}
