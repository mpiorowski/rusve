use anyhow::Result;
use sqlx::types::time::OffsetDateTime;
use sqlx::{postgres::PgRow, query, types::Uuid, Row};
use tonic::{Request, Response, Status};

use crate::proto::users_service_server::UsersService;
use crate::proto::UserRole;
use crate::proto::{AuthRequest, User};
use crate::MyService;

fn map_user(row: Option<PgRow>) -> Result<User> {
    match row {
        Some(row) => {
            let id: Uuid = row.try_get("id")?;
            let sub: String = row.try_get("sub")?;
            let email: String = row.try_get("email")?;
            let created: OffsetDateTime = row.try_get("created")?;
            let updated: OffsetDateTime = row.try_get("updated")?;
            let deleted: Option<OffsetDateTime> = row.try_get("deleted")?;

            Ok(User {
                id: id.to_string(),
                role: UserRole::as_str_name(&UserRole::RoleUser).to_string(),
                sub,
                email,
                created: created.to_string(),
                updated: updated.to_string(),
                deleted: deleted.map(|d| d.to_string()),
            })
        }
        None => Err(anyhow::anyhow!("User not found")),
    }
}

#[tonic::async_trait]
impl UsersService for MyService {
    async fn auth(&self, request: Request<AuthRequest>) -> Result<Response<User>, Status> {
        println!("Auth: {:?}", request);
        let start = std::time::Instant::now();
        let self_pool = self.pool.clone();

        let request = request.into_inner();
        let email = request.email;
        let sub = request.sub;

        let row =
            query("update users set updated = now() where email = $1 and sub = $2 returning *")
                .bind(&email)
                .bind(&sub)
                .fetch_optional(&self_pool)
                .await
                .map_err(|e| Status::internal(e.to_string()))?;

        match row {
            Some(row) => {
                let user = map_user(Some(row)).map_err(|e| Status::internal(e.to_string()))?;
                println!("Elapsed: {:?}", start.elapsed());
                Ok(Response::new(user))
            }
            None => {
                let row =
                    query("insert into users (email, sub, role) values ($1, $2, $3) returning *")
                        .bind(&email)
                        .bind(&sub)
                        .bind(UserRole::as_str_name(&UserRole::RoleUser))
                        .fetch_one(&self_pool)
                        .await
                        .map_err(|e| Status::internal(e.to_string()))?;

                let user = map_user(Some(row)).map_err(|e| Status::internal(e.to_string()))?;
                println!("Elapsed: {:?}", start.elapsed());
                Ok(Response::new(user))
            }
        }
    }
}
