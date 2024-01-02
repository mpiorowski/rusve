use crate::profile_validation::Validation;
use crate::proto::users_service_server::UsersService;
use crate::proto::{AuthResponse, Empty, Id, Profile};
use crate::user_db::StringOrUuid;
use crate::MyService;
use anyhow::Result;
use tonic::{Request, Response, Status};

#[tonic::async_trait]
impl UsersService for MyService {
    async fn create_user(&self, request: Request<Empty>) -> Result<Response<Id>, Status> {
        let start = std::time::Instant::now();
        let metadata = request.metadata();
        let oauth_user = rusve_users::decode_oauth_token(metadata, &self.env)?;

        let conn = self.pool.get().await.map_err(|e| {
            tracing::error!("Failed to get connection: {:?}", e);
            Status::internal("Failed to get connection")
        })?;

        // Create a new user if one doesn't exist, otherwise update the existing user.
        let user = crate::user_db::create_user(
            &conn,
            &oauth_user.sub,
            &oauth_user.email,
            &oauth_user.avatar,
        )
        .await
        .map_err(|e| {
            tracing::error!("Failed to create user: {:?}", e);
            Status::internal("Failed to create user")
        })?;

        // Create a new token.
        let token = crate::token_db::create_token(&conn, &user.id)
            .await
            .map_err(|e| {
                tracing::error!("Failed to create token: {:?}", e);
                Status::internal("Failed to create token")
            })?;

        // Delete old tokens. If this fails, it's not a big deal.
        tokio::spawn(async move {
            if let Err(err) = crate::token_db::delete_old_tokens(&conn).await {
                tracing::error!("Failed to delete old tokens: {:?}", err);
            }
        });

        tracing::info!("CreateUser: {:?}", start.elapsed());
        Ok(Response::new(Id {
            id: token.id.to_string(),
        }))
    }

    async fn auth(&self, request: Request<Empty>) -> Result<Response<AuthResponse>, Status> {
        let start = std::time::Instant::now();
        let metadata = request.metadata();
        let token = rusve_users::decode_token(metadata)?.token;

        let conn = self.pool.get().await.map_err(|e| {
            tracing::error!("Failed to get connection: {:?}", e);
            Status::internal("Failed to get connection")
        })?;

        let token = crate::token_db::select_token_by_id(&conn, &token)
            .await
            .map_err(|e| {
                tracing::error!("Failed to auth token: {:?}", e);
                Status::unauthenticated("Failed to auth token")
            })?;

        // check if token has expired, 7 days
        if token.created + time::Duration::days(7) < time::OffsetDateTime::now_utc() {
            tracing::error!("Token has expired");
            return Err(Status::unauthenticated("Unauthenticated"));
        }

        // create new token
        let token_id = crate::token_db::update_token_id(&conn, &token.id)
            .await
            .map_err(|e| {
                tracing::error!("Failed to update token: {:?}", e);
                Status::internal("Failed to update token")
            })?;

        // get user
        let mut user = crate::user_db::select_user_by_id(&conn, StringOrUuid::Uuid(token.user_id))
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
        let subscribed = crate::stripe_service::check_subscription(&conn, &self.env, &user)
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
        let user_id = rusve_users::decode_token(metadata)?.user_id;

        let conn = self.pool.get().await.map_err(|e| {
            tracing::error!("Failed to get connection: {:?}", e);
            Status::internal("Failed to get connection")
        })?;

        let profile = crate::profile_db::select_profile_by_user_id(&conn, &user_id)
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
        let user_id = rusve_users::decode_token(metadata)?.user_id;

        let conn = self.pool.get().await.map_err(|e| {
            tracing::error!("Failed to get connection: {:?}", e);
            Status::internal("Failed to get connection")
        })?;

        let mut profile = request.into_inner();
        Validation::validate(&profile)?;

        if profile.id.is_empty() {
            profile = crate::profile_db::insert_profile(&conn, &user_id, &profile)
                .await
                .map_err(|e| {
                    tracing::error!("Failed to insert profile: {:?}", e);
                    Status::internal("Failed to insert profile")
                })?;
        } else {
            profile = crate::profile_db::update_profile(&conn, &user_id, &profile)
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
        let user_id = rusve_users::decode_token(metadata)?.user_id;

        let conn = self.pool.get().await.map_err(|e| {
            tracing::error!("Failed to get connection: {:?}", e);
            Status::internal("Failed to get connection")
        })?;
        let user = crate::user_db::select_user_by_id(&conn, StringOrUuid::String(user_id))
            .await
            .map_err(|e| {
                tracing::error!("Failed to auth user: {:?}", e);
                Status::unauthenticated("Failed to auth user")
            })?;

        let url = crate::stripe_service::create_checkout(&conn, &self.env, user)
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
        let user_id = rusve_users::decode_token(metadata)?.user_id;

        let conn = self.pool.get().await.map_err(|e| {
            tracing::error!("Failed to get connection: {:?}", e);
            Status::internal("Failed to get connection")
        })?;
        let user = crate::user_db::select_user_by_id(&conn, StringOrUuid::String(user_id))
            .await
            .map_err(|e| {
                tracing::error!("Failed to auth user: {:?}", e);
                Status::unauthenticated("Failed to auth user")
            })?;

        let url = crate::stripe_service::create_portal(&conn, &self.env, user)
            .await
            .map_err(|e| {
                tracing::error!("Failed to create portal session: {:?}", e);
                Status::internal("Failed to create portal session")
            })?;

        tracing::info!("CreateStripePortal: {:?}", start.elapsed());
        Ok(Response::new(crate::proto::StripeUrlResponse { url }))
    }
}
