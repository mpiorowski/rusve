use tonic::{Request, Response, Status};

pub async fn get_profile_by_user_id(
    env: &rusve_users::Env,
    pool: &deadpool_postgres::Pool,
    request: Request<crate::proto::Empty>,
) -> Result<Response<crate::proto::Profile>, Status> {
    let start = std::time::Instant::now();
    let metadata = request.metadata();
    let user_id = rusve_users::decode_token(metadata, &env.jwt_secret)?.id;

    let conn = pool.get().await.map_err(|e| {
        tracing::error!("Failed to get connection: {:?}", e);
        Status::internal("Failed to get connection")
    })?;

    let profile = crate::profile_db::select_profile_by_user_id(&conn, &user_id)
        .await
        .map_err(|e| {
            tracing::error!("Failed to get profile: {:?}", e);
            Status::internal("Failed to get profile")
        })?;

    tracing::info!("get_profile_by_user_id: {:?}", start.elapsed());
    Ok(Response::new(profile.unwrap_or_default()))
}

pub async fn create_profile(
    env: &rusve_users::Env,
    pool: &deadpool_postgres::Pool,
    request: Request<crate::proto::Profile>,
) -> Result<Response<crate::proto::Profile>, Status> {
    let start = std::time::Instant::now();
    let metadata = request.metadata();
    let user_id = rusve_users::decode_token(metadata, &env.jwt_secret)?.id;

    let conn = pool.get().await.map_err(|e| {
        tracing::error!("Failed to get connection: {:?}", e);
        Status::internal("Failed to get connection")
    })?;

    let mut profile = request.into_inner();
    crate::profile_validation::Validation::validate(&profile)?;

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

    tracing::info!("update_user: {:?}", start.elapsed());
    Ok(Response::new(profile))
}
