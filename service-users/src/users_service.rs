use crate::proto::users_service_server::UsersService;
use crate::proto::{AuthRequest, User, UserIds};
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

impl TryFrom<Option<PgRow>> for User {
    type Error = anyhow::Error;

    fn try_from(row: Option<PgRow>) -> Result<Self, Self::Error> {
        match row {
            Some(row) => {
                let id: Uuid = row.try_get("id")?;
                let created: OffsetDateTime = row.try_get("created")?;
                let updated: OffsetDateTime = row.try_get("updated")?;
                let deleted: Option<OffsetDateTime> = row.try_get("deleted")?;
                let email: String = row.try_get("email")?;
                let role: String = row.try_get("role")?;
                let role = UserRole::from_str_name(&role).ok_or(anyhow::anyhow!("Invalid role"))?;
                let sub: String = row.try_get("sub")?;
                let name = row.try_get("name")?;
                let avatar: Option<Uuid> = row.try_get("avatar")?;

                Ok(User {
                    id: id.to_string(),
                    created: created.to_string(),
                    updated: updated.to_string(),
                    deleted: deleted.map(|d| d.to_string()),
                    email,
                    role: role.into(),
                    sub,
                    name,
                    avatar: avatar.map(|a| a.to_string()),
                })
            }
            None => Err(anyhow::anyhow!("User not found")),
        }
    }
}

#[tonic::async_trait]
impl UsersService for MyService {
    type GetUsersStream = ReceiverStream<Result<User, Status>>;

    async fn auth(&self, request: Request<AuthRequest>) -> Result<Response<User>, Status> {
        #[cfg(debug_assertions)]
        println!("Auth: {:?}", request);
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
                let user: User = Some(row).try_into().map_err(anyhow::Error::into_status)?;
                tx.commit().await.map_err(sqlx::Error::into_status)?;
                println!("Elapsed: {:?}", start.elapsed());
                Ok(Response::new(user))
            }
            None => {
                let row =
                    query("insert into users (email, sub, role) values ($1, $2, $3) returning *")
                        .bind(&request.email)
                        .bind(&request.sub)
                        .bind(UserRole::as_str_name(&UserRole::RoleUser))
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
        #[cfg(debug_assertions)]
        println!("GetNotes = {:?}", request);
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
        #[cfg(debug_assertions)]
        println!("GetUser: {:?}", request);
        let start = std::time::Instant::now();
        let pool = self.pool.clone();

        let user_id_metadata = request
            .metadata()
            .get("user_id")
            .ok_or_else(|| Status::unauthenticated("Missing user_id metadata"))?
            .to_str()
            .map_err(|e| Status::internal(e.to_string()))?
            .to_owned();

        let request = request.into_inner();
        let user_id = request.user_id;
        if user_id != user_id_metadata {
            return Err(Status::unauthenticated("Invalid user_id"));
        }

        let user_id = Uuid::parse_str(&user_id).map_err(|e| Status::internal(e.to_string()))?;

        let row = query("select * from users where id = $1")
            .bind(&user_id)
            .fetch_optional(&pool)
            .await
            .map_err(sqlx::Error::into_status)?;

        let user = row.try_into().map_err(anyhow::Error::into_status)?;
        println!("Elapsed: {:?}", start.elapsed());
        Ok(Response::new(user))
    }

    async fn create_user(&self, request: Request<User>) -> Result<Response<User>, Status> {
        #[cfg(debug_assertions)]
        println!("CreateUser: {:?}", request);
        let start = std::time::Instant::now();

        let pool = self.pool.clone();
        let mut tx = pool.begin().await.map_err(sqlx::Error::into_status)?;

        let user_id = request
            .metadata()
            .get("user_id")
            .ok_or_else(|| Status::unauthenticated("Missing user_id metadata"))?
            .to_str()
            .map_err(|e| Status::internal(e.to_string()))?;
        let user_uuid = Uuid::parse_str(&user_id).map_err(|e| Status::internal(e.to_string()))?;
        let request = request.into_inner();

        let avatar_id = request.avatar;
        let avatar_uuid = match avatar_id {
            Some(avatar_id) => Some(Uuid::try_parse(&avatar_id).map_err(|e| {
                Status::invalid_argument(format!("Invalid avatar_id: {}", e.to_string()))
            })?),
            None => None,
        };

        let row = query("update users set name = $1, avatar = $2 where id = $3 returning *")
            .bind(&request.name)
            .bind(&avatar_uuid)
            .bind(&user_uuid)
            .fetch_one(&mut tx)
            .await
            .map_err(sqlx::Error::into_status)?;

        let user = Some(row).try_into().map_err(anyhow::Error::into_status)?;
        tx.commit().await.map_err(sqlx::Error::into_status)?;
        println!("Elapsed: {:?}", start.elapsed());
        Ok(Response::new(user))
    }
}
