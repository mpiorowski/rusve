use std::str::FromStr;

use crate::proto::users_service_server::UsersService;
use crate::proto::UserId;
use crate::proto::{AuthRequest, Empty, PaymentId, User, UserIds};
use crate::{users_db, MyService};
use anyhow::Result;
use futures_util::TryStreamExt;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};
use uuid::Uuid;

#[tonic::async_trait]
impl UsersService for MyService {
    type GetUsersStream = ReceiverStream<Result<User, Status>>;

    async fn auth(&self, request: Request<AuthRequest>) -> Result<Response<User>, Status> {
        let start = std::time::Instant::now();

        let mut conn = self.pool.get().await.map_err(|e| {
            tracing::error!("Failed to get connection: {:?}", e);
            Status::internal(e.to_string())
        })?;
        let request = request.into_inner();

        let user = users_db::auth_user(&mut conn, request).await.map_err(|e| {
            tracing::error!("Failed to auth user: {:?}", e);
            Status::unauthenticated(e.to_string())
        })?;
        if user.deleted.is_some() {
            return Err(Status::unauthenticated("Unauthenticated"));
        }

        tracing::info!("Auth: {:?}", start.elapsed());
        Ok(Response::new(user))
    }

    async fn get_users(
        &self,
        request: Request<UserIds>,
    ) -> Result<Response<Self::GetUsersStream>, Status> {
        let start = std::time::Instant::now();

        let mut conn = self.pool.get().await.map_err(|e| {
            tracing::error!("Failed to get connection: {:?}", e);
            Status::internal(e.to_string())
        })?;

        let user_ids: Vec<Uuid> = request
            .into_inner()
            .user_ids
            .iter()
            .map(|id| Uuid::from_str(&id))
            .collect::<Result<Vec<Uuid>, _>>()
            .map_err(|e| {
                tracing::error!("Failed to parse user ids: {:?}", e);
                Status::invalid_argument(e.to_string())
            })?;

        let users = users_db::get_users(&mut conn, user_ids)
            .await
            .map_err(|e| {
                tracing::error!("Failed to get users: {:?}", e);
                Status::internal(e.to_string())
            })?;

        let (tx, rx) = mpsc::channel(128);
        tokio::spawn(async move {
            futures_util::pin_mut!(users);
            loop {
                let user = match users.try_next().await {
                    Ok(Some(user)) => user,
                    Ok(None) => break,
                    Err(e) => {
                        tracing::error!("Failed to get user: {:?}", e);
                        if let Err(e) = tx.send(Err(Status::internal(e.to_string()))).await {
                            tracing::error!("Failed to send error: {:?}", e);
                        }
                        break;
                    }
                };
                let user: User = match user.try_into() {
                    Ok(user) => user,
                    Err(e) => {
                        tracing::error!("Failed to convert user: {:?}", e);
                        if let Err(e) = tx.send(Err(Status::internal(e.to_string()))).await {
                            tracing::error!("Failed to send error: {:?}", e);
                        }
                        break;
                    }
                };
                if let Err(e) = tx.send(Ok(user)).await {
                    tracing::error!("Failed to send user: {:?}", e);
                    break;
                }
            }
            tracing::info!("GetUsers: {:?}", start.elapsed());
        });
        Ok(Response::new(ReceiverStream::new(rx)))
    }

    async fn get_user(&self, request: Request<UserId>) -> Result<Response<User>, Status> {
        let start = std::time::Instant::now();

        let mut conn = self.pool.get().await.map_err(|e| {
            tracing::error!("Failed to get connection: {:?}", e);
            Status::internal(e.to_string())
        })?;

        let user_id = Uuid::from_str(&request.into_inner().user_id).map_err(|e| {
            tracing::error!("Failed to parse user id: {:?}", e);
            Status::invalid_argument(e.to_string())
        })?;
        let user = users_db::get_user(&mut conn, &user_id).await.map_err(|e| {
            tracing::error!("Failed to get user: {:?}", e);
            Status::internal(e.to_string())
        })?;

        tracing::info!("GetUser: {:?}", start.elapsed());
        Ok(Response::new(user))
    }

    async fn update_user(&self, request: Request<User>) -> Result<Response<Empty>, Status> {
        let start = std::time::Instant::now();

        let mut conn = self.pool.get().await.map_err(|e| {
            tracing::error!("Failed to get connection: {:?}", e);
            Status::internal(e.to_string())
        })?;

        let request = request.into_inner();
        let user_id = Uuid::from_str(&request.id).map_err(|e| {
            tracing::error!("Failed to parse user id: {:?}", e);
            Status::invalid_argument(e.to_string())
        })?;
        let avatar_id = if let Some(avatar_id) = request.avatar_id {
            Some(Uuid::from_str(&avatar_id).map_err(|e| {
                tracing::error!("Failed to parse avatar id: {:?}", e);
                Status::invalid_argument(e.to_string())
            })?)
        } else {
            None
        };
        users_db::update_user(&mut conn, &user_id, &request.name, &avatar_id)
            .await
            .map_err(|e| {
                tracing::error!("Failed to update user: {:?}", e);
                Status::internal(e.to_string())
            })?;

        tracing::info!("UpdateUser: {:?}", start.elapsed());
        Ok(Response::new(Empty {}))
    }

    async fn update_payment_id(
        &self,
        request: Request<PaymentId>,
    ) -> Result<Response<Empty>, Status> {
        let start = std::time::Instant::now();

        let mut conn = self.pool.get().await.map_err(|e| {
            tracing::error!("Failed to get connection: {:?}", e);
            Status::internal(e.to_string())
        })?;
        let tx = conn.transaction().await.map_err(|e| {
            tracing::error!("Failed to start transaction: {:?}", e);
            Status::internal(e.to_string())
        })?;

        let request = request.into_inner();
        let user_id = Uuid::from_str(&request.user_id).map_err(|e| {
            tracing::error!("Failed to parse user id: {:?}", e);
            Status::invalid_argument(e.to_string())
        })?;

        users_db::update_payment_id(&tx, &user_id, &request.payment_id)
            .await
            .map_err(|e| {
                tracing::error!("Failed to update payment id: {:?}", e);
                Status::internal(e.to_string())
            })?;

        tx.commit().await.map_err(|e| {
            tracing::error!("Failed to commit transaction: {:?}", e);
            Status::internal(e.to_string())
        })?;

        tracing::info!("UpdatePaymentId: {:?}", start.elapsed());
        Ok(Response::new(Empty {}))
    }
}
