use crate::proto::users_service_server::UsersService;
use crate::proto::{AuthResponse, Empty, Id, Profile};
use crate::MyService;
use anyhow::Result;
use tonic::{Request, Response, Status};

#[tonic::async_trait]
impl UsersService for MyService {
    async fn create_user(
        &self,
        request: Request<crate::proto::Empty>,
    ) -> Result<Response<Id>, Status> {
        crate::user_service::create_user(&self.env, &self.pool, request).await
    }

    async fn auth(&self, request: Request<Empty>) -> Result<Response<AuthResponse>, Status> {
        crate::user_service::auth(&self.env, &self.pool, request).await
    }

    async fn get_profile_by_user_id(
        &self,
        request: Request<crate::proto::Empty>,
    ) -> Result<Response<crate::proto::Profile>, Status> {
        crate::profile_service::get_profile_by_user_id(&self.env, &self.pool, request).await
    }

    async fn create_profile(&self, request: Request<Profile>) -> Result<Response<Profile>, Status> {
        crate::profile_service::create_profile(&self.env, &self.pool, request).await
    }

    async fn create_stripe_checkout(
        &self,
        request: Request<crate::proto::Empty>,
    ) -> Result<Response<crate::proto::StripeUrlResponse>, Status> {
        crate::stripe_service::create_stripe_checkout(&self.env, &self.pool, request).await
    }

    async fn create_stripe_portal(
        &self,
        request: Request<crate::proto::Empty>,
    ) -> Result<Response<crate::proto::StripeUrlResponse>, Status> {
        crate::stripe_service::create_stripe_portal(&self.env, &self.pool, request).await
    }
}
