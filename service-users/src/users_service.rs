use crate::profile_validation::Validation;
use crate::proto::users_service_server::UsersService;
use crate::proto::{AuthResponse, Empty, Profile};
use crate::users_db::{select_token_by_id, select_user_by_id, StringOrUuid};
use crate::{users_db, MyService};
use anyhow::Result;
use tonic::{Request, Response, Status};

#[tonic::async_trait]
impl UsersService for MyService {
    async fn auth(&self, request: Request<Empty>) -> Result<Response<AuthResponse>, Status> {
        let start = std::time::Instant::now();
        let metadata = request.metadata();
        let token = rusve_users::auth(metadata)?.token;

        let conn = self.pool.get().await.map_err(|e| {
            tracing::error!("Failed to get connection: {:?}", e);
            Status::internal("Failed to get connection")
        })?;

        let token = select_token_by_id(&conn, &token).await.map_err(|e| {
            tracing::error!("Failed to auth token: {:?}", e);
            Status::unauthenticated("Failed to auth token")
        })?;

        // check if token has expired
        if token.created + time::Duration::days(7) < time::OffsetDateTime::now_utc() {
            tracing::error!("Token has expired");
            return Err(Status::unauthenticated("Unauthenticated"));
        }

        // create new token
        let token_id = users_db::update_token_uuid(&conn, &token.user_id)
            .await
            .map_err(|e| {
                tracing::error!("Failed to update token: {:?}", e);
                Status::internal("Failed to update token")
            })?;

        // get user
        let mut user = users_db::select_user_by_id(&conn, StringOrUuid::Uuid(token.user_id))
            .await
            .map_err(|e| {
                tracing::error!("Failed to auth user: {:?}", e);
                Status::unauthenticated("Failed to auth user")
            })?;
        if user.deleted != "infinity" {
            tracing::error!("User is deleted");
            return Err(Status::unauthenticated("Unauthenticated"));
        }

        // check if user is subscribed
        let subscribed = crate::stripe_service::check_subscription(&conn, &user)
            .await
            .map_err(|e| {
                tracing::error!("Failed to update subscription: {:?}", e);
                Status::internal("Failed to update subscription")
            })?;
        user.subscription_active = subscribed;

        tracing::info!("Auth: {:?}", start.elapsed());
        Ok(Response::new(AuthResponse {
            user: user.into(),
            token: token_id.to_string(),
        }))
    }

    async fn get_profile_by_user_id(
        &self,
        request: Request<crate::proto::Empty>,
    ) -> Result<Response<crate::proto::Profile>, Status> {
        let start = std::time::Instant::now();
        let metadata = request.metadata();
        let user_id = rusve_users::auth(metadata)?.user_id;

        let conn = self.pool.get().await.map_err(|e| {
            tracing::error!("Failed to get connection: {:?}", e);
            Status::internal("Failed to get connection")
        })?;

        let profile = users_db::select_profile_by_user_id(&conn, &user_id)
            .await
            .map_err(|e| {
                tracing::error!("Failed to get profile: {:?}", e);
                Status::internal("Failed to get profile")
            })?;

        tracing::info!("GetProfileByUserId: {:?}", start.elapsed());
        Ok(Response::new(profile.unwrap_or_default()))
    }

    async fn create_profile(&self, request: Request<Profile>) -> Result<Response<Profile>, Status> {
        let start = std::time::Instant::now();
        let metadata = request.metadata();
        let user_id = rusve_users::auth(metadata)?.user_id;

        let conn = self.pool.get().await.map_err(|e| {
            tracing::error!("Failed to get connection: {:?}", e);
            Status::internal("Failed to get connection")
        })?;

        let mut profile = request.into_inner();
        Validation::validate(&profile)?;

        if profile.id.is_empty() {
            profile = users_db::insert_profile(&conn, &user_id, &profile)
                .await
                .map_err(|e| {
                    tracing::error!("Failed to insert profile: {:?}", e);
                    Status::internal("Failed to insert profile")
                })?;
        } else {
            profile = users_db::update_profile(&conn, &user_id, &profile)
                .await
                .map_err(|e| {
                    tracing::error!("Failed to update profile: {:?}", e);
                    Status::internal("Failed to update profile")
                })?;
        }

        tracing::info!("UpdateUser: {:?}", start.elapsed());
        Ok(Response::new(profile))
    }

    async fn create_stripe_checkout(
        &self,
        request: Request<crate::proto::Empty>,
    ) -> Result<Response<crate::proto::StripeUrlResponse>, Status> {
        let start = std::time::Instant::now();
        let metadata = request.metadata();
        let user_id = rusve_users::auth(metadata)?.user_id;

        let conn = self.pool.get().await.map_err(|e| {
            tracing::error!("Failed to get connection: {:?}", e);
            Status::internal("Failed to get connection")
        })?;
        let user = select_user_by_id(&conn, StringOrUuid::String(user_id))
            .await
            .map_err(|e| {
                tracing::error!("Failed to auth user: {:?}", e);
                Status::unauthenticated("Failed to auth user")
            })?;

        let url = crate::stripe_service::create_checkout(&conn, user)
            .await
            .map_err(|e| {
                tracing::error!("Failed to create checkout session: {:?}", e);
                Status::internal("Failed to create checkout session")
            })?;

        tracing::info!("CreateStripeCheckout: {:?}", start.elapsed());
        Ok(Response::new(crate::proto::StripeUrlResponse { url }))
    }

    async fn create_stripe_portal(
        &self,
        request: Request<crate::proto::Empty>,
    ) -> Result<Response<crate::proto::StripeUrlResponse>, Status> {
        let start = std::time::Instant::now();
        let metadata = request.metadata();
        let user_id = rusve_users::auth(metadata)?.user_id;

        let conn = self.pool.get().await.map_err(|e| {
            tracing::error!("Failed to get connection: {:?}", e);
            Status::internal("Failed to get connection")
        })?;
        let user = select_user_by_id(&conn, StringOrUuid::String(user_id))
            .await
            .map_err(|e| {
                tracing::error!("Failed to auth user: {:?}", e);
                Status::unauthenticated("Failed to auth user")
            })?;

        let url = crate::stripe_service::create_portal(&conn, user)
            .await
            .map_err(|e| {
                tracing::error!("Failed to create portal session: {:?}", e);
                Status::internal("Failed to create portal session")
            })?;

        tracing::info!("CreateStripePortal: {:?}", start.elapsed());
        Ok(Response::new(crate::proto::StripeUrlResponse { url }))
    }
}
