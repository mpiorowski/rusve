use crate::{
    oauth_service::{OAuth, OAuthConfig},
    AppState,
};
use anyhow::Result;
use axum::{
    extract::{Path, Query, State},
    response::{Redirect, Response},
};
use http::{header, StatusCode};
use oauth2::{CsrfToken, PkceCodeChallenge, Scope, TokenResponse};
use std::{collections::HashMap, sync::Arc};

pub async fn oauth_login(
    Path(provider): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Result<Redirect, StatusCode> {
    let conn = state.db_pool.get().await.map_err(|err| {
        tracing::error!("Failed to get DB connection: {:?}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let oauth_config =
        OAuthConfig::get_config_by_provider(&provider, state.env.clone()).map_err(|err| {
            tracing::error!("Failed to get OAuth provider: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    let client = oauth_config.build_oauth_client();

    // Generate a PKCE challenge.
    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    // Generate the full authorization URL.
    let mut client = client
        .authorize_url(CsrfToken::new_random)
        .set_pkce_challenge(pkce_challenge);
    for scope in oauth_config.scopes {
        client = client.add_scope(Scope::new(scope));
    }
    let (auth_url, csrf_token) = client.add_extra_param("access_type", "offline").url();

    // Save the CSRF token to the database.
    match crate::oauth_db::create_pkce(&conn, csrf_token.secret(), pkce_verifier.secret()).await {
        Ok(_) => {}
        Err(err) => {
            tracing::error!("Failed to save PKCE verifier: {:?}", err);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    }

    Ok(Redirect::to(auth_url.as_ref()))
}

pub async fn oauth_callback(
    Path(provider): Path<String>,
    State(state): State<Arc<AppState>>,
    Query(query): Query<HashMap<String, String>>,
) -> Result<Response, StatusCode> {
    let conn = state.db_pool.get().await.map_err(|err| {
        tracing::error!("Failed to get DB connection: {:?}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let code = match query.get("code") {
        Some(code) => code,
        None => return Err(StatusCode::BAD_REQUEST),
    };
    let csrf = match query.get("state") {
        Some(state) => state,
        None => return Err(StatusCode::BAD_REQUEST),
    };

    let pkce = match crate::oauth_db::select_pkce_by_csrf(&conn, csrf).await {
        Ok(Some(pkce)) => pkce,
        Ok(None) => return Err(StatusCode::BAD_REQUEST),
        Err(err) => {
            tracing::error!("Failed to select PKCE verifier: {:?}", err);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    // check if the `created` field is older than 10 minutes
    let diff = time::OffsetDateTime::now_utc() - pkce.created;
    if diff.whole_minutes() > 10 {
        return Err(StatusCode::BAD_REQUEST);
    }

    // Exchange the code with a token.
    let oauth_config =
        OAuthConfig::get_config_by_provider(&provider, state.env.clone()).map_err(|err| {
            tracing::error!("Failed to get OAuth provider: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    let client = oauth_config.build_oauth_client();
    let token = client
        .exchange_code(oauth2::AuthorizationCode::new(code.to_string()))
        // Set the PKCE code verifier.
        .set_pkce_verifier(oauth2::PkceCodeVerifier::new(pkce.pkce_verifier))
        .request_async(oauth2::reqwest::async_http_client)
        .await
        .map_err(|err| {
            tracing::error!("Failed to exchange code with token: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let user_profile = oauth_config
        .get_user_info(token.access_token().secret())
        .await?;

    // let mut client = match UsersServiceClient::connect("http://service-users:443").await {
    //     Ok(client) => client,
    //     Err(err) => {
    //         tracing::error!("Failed to connect to users service: {:?}", err);
    //         return Err(StatusCode::INTERNAL_SERVER_ERROR);
    //     }
    // };
    // let request = tonic::Request::new(crate::proto::AuthRequest {
    //     email: user_profile.email,
    //     sub: user_profile.sub,
    // });
    // let user = match client.auth(request).await {
    //     Ok(user) => user.into_inner(),
    //     Err(err) => {
    //         tracing::error!("Failed to authenticate user: {:?}", err);
    //         return Err(StatusCode::INTERNAL_SERVER_ERROR);
    //     }
    // };

    // Create a new user if one doesn't exist, otherwise update the existing user.
    let user = match crate::oauth_db::auth_user(&conn, &user_profile.sub, &user_profile.email).await
    {
        Ok(user) => user,
        Err(err) => {
            tracing::error!("Failed to authenticate user: {:?}", err);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    // Create a new token.
    let token = match crate::oauth_db::create_token(
        &conn,
        &user.id,
        token.access_token().secret(),
        match token.refresh_token() {
            Some(token) => token.secret(),
            None => "",
        },
    )
    .await
    {
        Ok(token) => token,
        Err(err) => {
            tracing::error!("Failed to create token: {:?}", err);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    tracing::info!("User authenticated: {:?}", user);

    // Delete old PKCE verifiers and tokens asynchronously. If this fails, it's not a big deal.
    tokio::spawn(async move {
        if let Err(err) = crate::oauth_db::delete_old_pkces(&conn).await {
            tracing::error!("Failed to delete old PKCE verifiers: {:?}", err);
        }
        if let Err(err) = crate::oauth_db::delete_old_tokens(&conn).await {
            tracing::error!("Failed to delete old tokens: {:?}", err);
        }
    });

    Ok(Response::builder()
        .status(StatusCode::PERMANENT_REDIRECT)
        // .header(
        // header::AUTHORIZATION,
        // format!("Bearer {}", token.id),
        // format!(
        //     "token={}; HttpOnly; Max-Age={}; Path=/; SameSite=Lax; Domain={}; Secure",
        //     token.id,
        //     // 7 days
        //     3600 * 24 * 7,
        //     client_domain
        // ),
        // )
        .header(
            header::LOCATION,
            format!("{}/?token={}", state.env.client_url, token.id),
        )
        .body("".into())
        .unwrap())
}
