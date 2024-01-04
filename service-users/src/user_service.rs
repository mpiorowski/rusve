use tonic::{Request, Response, Status};

pub async fn create_user(
    env: &rusve_users::Env,
    pool: &deadpool_postgres::Pool,
    request: Request<crate::proto::Empty>,
) -> Result<Response<crate::proto::Id>, tonic::Status> {
    let start = std::time::Instant::now();
    let metadata = request.metadata();
    let oauth_user = rusve_users::decode_oauth_token(metadata, &env)?;

    let conn = pool.get().await.map_err(|e| {
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
    Ok(Response::new(crate::proto::Id {
        id: token.id.to_string(),
    }))
}

pub async fn auth(
    env: &rusve_users::Env,
    pool: &deadpool_postgres::Pool,
    request: Request<crate::proto::Empty>,
) -> Result<Response<crate::proto::AuthResponse>, tonic::Status> {
    let start = std::time::Instant::now();
    let metadata = request.metadata();
    let token = rusve_users::decode_token(metadata)?.id;

    let conn = pool.get().await.map_err(|e| {
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
    let mut user =
        crate::user_db::select_user_by_id(&conn, crate::user_db::StringOrUuid::Uuid(token.user_id))
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
    let subscribed = crate::stripe_service::check_subscription(&conn, &env, &user)
        .await
        .map_err(|e| {
            tracing::error!("Failed to update subscription: {:?}", e);
            Status::internal("Failed to update subscription")
        })?;
    user.subscription_active = subscribed;

    tracing::info!("Auth: {:?}", start.elapsed());
    Ok(Response::new(crate::proto::AuthResponse {
        user: user.into(),
        token: token_id.to_string(),
    }))
}
