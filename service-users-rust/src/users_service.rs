use crate::proto::users_service_server::UsersService;
use crate::proto::{AuthRequest, Empty, PaymentId, User, UserIds};
use crate::proto::{UserId, UserRole};
use crate::MyService;
use anyhow::Result;
use sqlx::types::time::OffsetDateTime;
use sqlx::{postgres::PgRow, query, types::Uuid, Row};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tokio_stream::StreamExt;
use tonic::{Request, Response, Status};

trait IntoStatus {
    fn into_status(self) -> Status;
}

impl IntoStatus for sqlx::Error {
    fn into_status(self) -> Status {
        match self {
            sqlx::Error::Database(e) => Status::internal(e.message()),
            sqlx::Error::RowNotFound => Status::not_found("Row not found"),
            sqlx::Error::ColumnNotFound(_) => Status::not_found("Column not found"),
            _ => Status::internal("Unknown error"),
        }
    }
}

impl IntoStatus for anyhow::Error {
    fn into_status(self) -> Status {
        Status::internal(self.to_string())
    }
}

impl TryFrom<Option<PgRow>> for PgUser {
    type Error = anyhow::Error;

    fn try_from(value: Option<PgRow>) -> Result<Self> {
        match value {
            Some(row) => {
                let pg_user = PgUser {
                    id: row.try_get("id")?,
                    created: row.try_get("created")?,
                    updated: row.try_get("updated")?,
                    deleted: row.try_get("deleted")?,
                    email: row.try_get("email")?,
                    role: row.try_get("role")?,
                    sub: row.try_get("sub")?,
                    name: row.try_get("name")?,
                    avatar: row.try_get("avatar")?,
                    payment_id: row.try_get("payment_id")?,
                };
                Ok(pg_user)
            }
            None => Err(anyhow::anyhow!("User not found")),
        }
    }
}

impl TryFrom<PgUser> for User {
    type Error = anyhow::Error;

    fn try_from(user: PgUser) -> Result<Self> {
        let user = User {
            id: user.id.to_string(),
            created: user.created.to_string(),
            updated: user.updated.to_string(),
            deleted: user.deleted.map(|d| d.to_string()),
            email: user.email,
            role: UserRole::from_str_name(&user.role)
                .unwrap_or(UserRole::RoleUser)
                .into(),
            sub: user.sub,
            name: Some(user.name),
            avatar: user.avatar.map(|a| a.to_string()),
            payment_id: Some(user.payment_id),
        };
        Ok(user)
    }
}

impl TryFrom<Option<PgRow>> for User {
    type Error = anyhow::Error;

    fn try_from(value: Option<PgRow>) -> Result<Self> {
        let pg_user = PgUser::try_from(value)?;
        let user = User::try_from(pg_user)?;
        Ok(user)
    }
}

struct PgUser {
    id: Uuid,
    created: OffsetDateTime,
    updated: OffsetDateTime,
    deleted: Option<OffsetDateTime>,
    email: String,
    role: String,
    sub: String,
    name: String,
    avatar: Option<Uuid>,
    payment_id: String,
}

#[tonic::async_trait]
impl UsersService for MyService {
    type GetUsersStream = ReceiverStream<Result<User, Status>>;

    async fn auth(&self, request: Request<AuthRequest>) -> Result<Response<User>, Status> {
        let start = std::time::Instant::now();
        let pool = self.pool.clone();
        let mut tx = pool.begin().await.map_err(sqlx::Error::into_status)?;

        let request = request.into_inner();

        let row =
            query("update users set updated = now() where email = $1 and sub = $2 returning *")
                .bind(&request.email)
                .bind(&request.sub)
                .fetch_optional(&mut tx)
                .await
                .map_err(sqlx::Error::into_status)?;

        match row {
            Some(row) => {
                let user: PgUser = Some(row).try_into().map_err(anyhow::Error::into_status)?;
                if user.deleted.is_some() {
                    return Err(Status::unauthenticated("Unauthenticated"));
                }
                let user: User = user.try_into().map_err(anyhow::Error::into_status)?;
                tx.commit().await.map_err(sqlx::Error::into_status)?;
                println!("Elapsed: {:?}", start.elapsed());
                Ok(Response::new(user))
            }
            None => {
                let row =
                    query("insert into users (email, role, sub) values ($1, $2, $3) returning *")
                        .bind(&request.email)
                        .bind(UserRole::as_str_name(&UserRole::RoleUser))
                        .bind(&request.sub)
                        .fetch_one(&mut tx)
                        .await
                        .map_err(sqlx::Error::into_status)?;

                let user = Some(row).try_into().map_err(anyhow::Error::into_status)?;
                tx.commit().await.map_err(sqlx::Error::into_status)?;
                println!("Elapsed: {:?}", start.elapsed());
                Ok(Response::new(user))
            }
        }
    }

    async fn get_users(
        &self,
        request: Request<UserIds>,
    ) -> Result<Response<Self::GetUsersStream>, Status> {
        let start = std::time::Instant::now();

        let pool = self.pool.clone();
        let (tx, rx) = mpsc::channel(4);

        let user_ids = request.into_inner().user_ids;
        let user_ids = user_ids
            .into_iter()
            .map(|id| Uuid::parse_str(&id).map_err(|e| anyhow::anyhow!(e)))
            .collect::<Result<Vec<Uuid>>>()
            .map_err(anyhow::Error::into_status)?;

        tokio::spawn(async move {
            let mut stream = query("SELECT * FROM users WHERE id = ANY($1)")
                .bind(user_ids)
                .fetch(&pool);

            loop {
                match stream.try_next().await {
                    Ok(None) => {
                        let elapsed = start.elapsed();
                        println!("Elapsed: {:.2?}", elapsed);
                        break;
                    }
                    Ok(user) => {
                        let user = user.try_into().map_err(anyhow::Error::into_status);
                        match user {
                            Err(err) => {
                                tx.send(Err(Status::internal(err.to_string())))
                                    .await
                                    .unwrap();
                                break;
                            }
                            Ok(user) => {
                                tx.send(Ok(user)).await.unwrap();
                            }
                        }
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

    async fn get_user(&self, request: Request<UserId>) -> Result<Response<User>, Status> {
        let start = std::time::Instant::now();
        let pool = self.pool.clone();

        let request = request.into_inner();
        let user_id = request.user_id;
        let user_id = Uuid::parse_str(&user_id).map_err(|e| Status::internal(e.to_string()))?;

        let row = query("select * from users where id = $1")
            .bind(user_id)
            .fetch_optional(&pool)
            .await
            .map_err(sqlx::Error::into_status)?;

        let user = row.try_into().map_err(anyhow::Error::into_status)?;
        println!("Elapsed: {:?}", start.elapsed());
        Ok(Response::new(user))
    }

    async fn create_user(&self, request: Request<User>) -> Result<Response<User>, Status> {
        let start = std::time::Instant::now();

        let pool = self.pool.clone();
        let mut tx = pool.begin().await.map_err(sqlx::Error::into_status)?;

        let request = request.into_inner();
        let user_uuid =
            Uuid::parse_str(&request.id).map_err(|e| Status::internal(e.to_string()))?;

        let avatar_id = request.avatar;
        let avatar_uuid = match avatar_id {
            Some(avatar_id) => Some(
                Uuid::try_parse(&avatar_id)
                    .map_err(|e| Status::invalid_argument(format!("Invalid avatar_id: {}", e)))?,
            ),
            None => None,
        };

        let row = query("update users set name = $1, avatar = $2 where id = $3 returning *")
            .bind(request.name)
            .bind(avatar_uuid)
            .bind(user_uuid)
            .fetch_one(&mut tx)
            .await
            .map_err(sqlx::Error::into_status)?;

        let user = Some(row).try_into().map_err(anyhow::Error::into_status)?;
        tx.commit().await.map_err(sqlx::Error::into_status)?;
        println!("Elapsed: {:?}", start.elapsed());
        Ok(Response::new(user))
    }

    async fn update_payment_id(
        &self,
        request: Request<PaymentId>,
    ) -> Result<Response<Empty>, Status> {
        let start = std::time::Instant::now();

        let pool = self.pool.clone();
        let mut tx = pool.begin().await.map_err(sqlx::Error::into_status)?;

        let request = request.into_inner();
        let user_uuid =
            Uuid::parse_str(&request.user_id).map_err(|e| Status::internal(e.to_string()))?;
        let payment_id = request.payment_id;

        query("update users set payment_id = $1 where id = $2")
            .bind(payment_id)
            .bind(user_uuid)
            .execute(&mut tx)
            .await
            .map_err(sqlx::Error::into_status)?;

        tx.commit().await.map_err(sqlx::Error::into_status)?;
        println!("Elapsed: {:?}", start.elapsed());
        Ok(Response::new(Empty {}))
    }
}
