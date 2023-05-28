use crate::proto::users_service_server::UsersService;
use crate::proto::{AuthRequest, Empty, PaymentId, User, UserIds};
use crate::proto::{UserId, UserRole};
use crate::MyService;
use anyhow::Result;
use std::iter::Iterator;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};
use uuid::Uuid;

use crate::models::*;
use crate::schema::users::dsl::*;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;

impl TryFrom<DieselUser> for User {
    type Error = tonic::Status;

    fn try_from(user: DieselUser) -> Result<Self, Self::Error> {
        Ok(User {
            id: user.id.to_string(),
            created: user.created.to_string(),
            updated: user.updated.to_string(),
            deleted: user.deleted.map(|d| d.to_string()),
            email: user.email,
            role: UserRole::from_str_name(&user.role)
                .unwrap_or(UserRole::RoleUser)
                .into(),
            sub: user.sub,
            name: user.name,
            avatar_id: user.avatar_id.map(|a| a.to_string()),
            payment_id: Some(user.payment_id),
        })
    }
}

#[tonic::async_trait]
impl UsersService for MyService {
    type GetUsersStream = ReceiverStream<Result<User, Status>>;

    async fn auth(&self, request: Request<AuthRequest>) -> Result<Response<User>, Status> {
        #[cfg(debug_assertions)]
        println!("Auth: {:?}", request);
        let start = std::time::Instant::now();

        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| Status::internal(e.to_string()))?;
        let request = request.into_inner();

        let user = diesel::update(users)
            .filter(email.eq(&request.email))
            .filter(sub.eq(&request.sub))
            .set(updated.eq(diesel::dsl::now))
            .get_result::<DieselUser>(&mut conn)
            .await;

        match user {
            Ok(row) => {
                if row.deleted.is_some() {
                    return Err(Status::unauthenticated("Unauthenticated"));
                }
                let user: User = row.try_into()?;
                println!("Elapsed: {:?}", start.elapsed());
                Ok(Response::new(user))
            }
            Err(_) => {
                let user = diesel::insert_into(users)
                    .values((
                        email.eq(&request.email),
                        role.eq(UserRole::as_str_name(&UserRole::RoleUser)),
                        sub.eq(&request.sub),
                    ))
                    .get_result::<DieselUser>(&mut conn)
                    .await
                    .map_err(|e| Status::internal(e.to_string()))?;
                let user: User = user.try_into()?;
                println!("Elapsed: {:?}", start.elapsed());
                Ok(Response::new(user))
            }
        }
    }

    async fn get_users(
        &self,
        request: Request<UserIds>,
    ) -> Result<Response<Self::GetUsersStream>, Status> {
        #[cfg(debug_assertions)]
        println!("GetUsers: {:?}", request);
        let start = std::time::Instant::now();

        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        let user_ids = request.into_inner().user_ids;
        let user_ids = user_ids
            .into_iter()
            .map(|val| Uuid::parse_str(&val).map_err(|e| anyhow::anyhow!(e)))
            .collect::<Result<Vec<Uuid>>>()
            .map_err(|e| Status::internal(e.to_string()))?;

        let results = users
            .filter(id.eq_any(&user_ids))
            .select(DieselUser::as_select())
            .load(&mut conn)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        let (tx, rx) = mpsc::channel(128);
        tokio::spawn(async move {
            for user in results {
                let user: User = match user.try_into() {
                    Ok(user) => user,
                    Err(e) => {
                        tx.send(Err(Status::internal(e.to_string()))).await.unwrap();
                        return;
                    }
                };
                tx.send(Ok(user)).await.unwrap();
            }
            println!("Elapsed: {:?}", start.elapsed());
        });
        Ok(Response::new(ReceiverStream::new(rx)))
    }

    async fn get_user(&self, request: Request<UserId>) -> Result<Response<User>, Status> {
        #[cfg(debug_assertions)]
        println!("GetUserr: {:?}", request);

        let start = std::time::Instant::now();

        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        let request = request.into_inner();
        let user_uuid =
            Uuid::parse_str(&request.user_id).map_err(|e| Status::internal(e.to_string()))?;

        let user: DieselUser = users
            .filter(id.eq(user_uuid))
            .first(&mut conn)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        let user: User = user.try_into()?;

        println!("Elapsed: {:?}", start.elapsed());
        Ok(Response::new(user))
    }

    async fn update_user(&self, request: Request<User>) -> Result<Response<User>, Status> {
        #[cfg(debug_assertions)]
        println!("UpdateUser: {:?}", request);
        let start = std::time::Instant::now();

        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        let request = request.into_inner();
        let user_uuid =
            Uuid::parse_str(&request.id).map_err(|e| Status::internal(e.to_string()))?;

        let avatar_uuid = match request.avatar_id {
            Some(avatar_uuid) => Some(
                Uuid::try_parse(&avatar_uuid)
                    .map_err(|e| Status::invalid_argument(format!("Invalid avatar_id: {}", e)))?,
            ),
            None => None,
        };

        let user = diesel::update(users)
            .filter(id.eq(user_uuid))
            .set((name.eq(&request.name), avatar_id.eq(avatar_uuid)))
            .get_result::<DieselUser>(&mut conn)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        let user: User = user.try_into()?;
        println!("Elapsed: {:?}", start.elapsed());
        Ok(Response::new(user))
    }

    async fn update_payment_id(
        &self,
        request: Request<PaymentId>,
    ) -> Result<Response<Empty>, Status> {
        #[cfg(debug_assertions)]
        println!("UpdatePaymentId: {:?}", request);
        let start = std::time::Instant::now();

        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        let request = request.into_inner();
        let user_uuid =
            Uuid::parse_str(&request.user_id).map_err(|e| Status::internal(e.to_string()))?;

        diesel::update(users)
            .filter(id.eq(&user_uuid))
            .set((payment_id.eq(&request.payment_id),))
            .execute(&mut conn)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        println!("Elapsed: {:?}", start.elapsed());
        Ok(Response::new(Empty {}))
    }
}
