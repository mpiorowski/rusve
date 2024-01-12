use crate::{
    auth_oauth::{OAuth, OAuthConfig},
    AppState,
};
use anyhow::Result;
use axum::{
    extract::{Path, Query, State},
    response::Redirect,
};
use oauth2::{CsrfToken, PkceCodeChallenge, Scope, TokenResponse};
use std::{collections::HashMap, sync::Arc};

pub async fn oauth_login(
    Path(provider): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Result<Redirect, Redirect> {
    let conn = state.pool.get().await.map_err(|err| {
        tracing::error!("Failed to get DB connection: {:?}", err);
        Redirect::to(&format!("{}/auth?error=1", state.env.client_url))
    })?;

    let oauth_config =
        OAuthConfig::get_config_by_provider(&provider, state.env.clone()).map_err(|err| {
            tracing::error!("Failed to get OAuth provider: {:?}", err);
            Redirect::to(&format!("{}/auth?error=1", state.env.client_url))
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

    // Save the CSRF token and PKCE verifier so we can verify them later.
    // Here we are using the PostgreSQL database, but you can use whatever you want, e.g. Redis.
    crate::auth_db::create_verifiers(&conn, csrf_token.secret(), pkce_verifier.secret())
        .await
        .map_err(|err| {
            tracing::error!("Failed to save verifier: {:?}", err);
            Redirect::to(&format!("{}/auth?error=1", state.env.client_url))
        })?;

    Ok(Redirect::to(auth_url.as_ref()))
}

pub async fn oauth_callback(
    Path(provider): Path<String>,
    State(state): State<Arc<AppState>>,
    Query(query): Query<HashMap<String, String>>,
) -> Result<Redirect, Redirect> {
    let conn = state.pool.get().await.map_err(|err| {
        tracing::error!("Failed to get DB connection: {:?}", err);
        Redirect::to(&format!("{}/auth?error=2", state.env.client_url))
    })?;

    let code = query.get("code").ok_or_else(|| {
        tracing::error!("Missing code");
        Redirect::to(&format!("{}/auth?error=2", state.env.client_url))
    })?;
    let csrf = query.get("state").ok_or_else(|| {
        tracing::error!("Missing CSRF token");
        Redirect::to(&format!("{}/auth?error=2", state.env.client_url))
    })?;

    let verifiers = match crate::auth_db::select_verifiers_by_csrf(&conn, csrf).await {
        Ok(Some(verifiers)) => verifiers,
        Ok(None) => {
            return Err(Redirect::to(&format!(
                "{}/auth?error=2",
                state.env.client_url
            )))
        }
        Err(err) => {
            tracing::error!("Failed to select verifiers: {:?}", err);
            return Err(Redirect::to(&format!(
                "{}/auth?error=2",
                state.env.client_url
            )));
        }
    };

    // Delete old verifiers asynchronously. If this fails, it's not a big deal.
    tokio::spawn(async move {
        if let Err(err) = crate::auth_db::delete_old_verifiers(&conn).await {
            tracing::error!("Failed to delete old verifiers: {:?}", err);
        }
    });

    // Check if the CSRF token is valid.
    if verifiers.created + time::Duration::minutes(10) < time::OffsetDateTime::now_utc() {
        return Err(Redirect::to(&format!(
            "{}/auth?error=2",
            state.env.client_url
        )));
    }

    // Exchange the code with a token.
    let oauth_config =
        OAuthConfig::get_config_by_provider(&provider, state.env.clone()).map_err(|err| {
            tracing::error!("Failed to get OAuth provider: {:?}", err);
            Redirect::to(&format!("{}/auth?error=2", state.env.client_url))
        })?;
    let client = oauth_config.build_oauth_client();
    let token = client
        .exchange_code(oauth2::AuthorizationCode::new(code.to_string()))
        // Set the PKCE code verifier.
        .set_pkce_verifier(oauth2::PkceCodeVerifier::new(verifiers.pkce_verifier))
        .request_async(oauth2::reqwest::async_http_client)
        .await
        .map_err(|err| {
            tracing::error!("Failed to exchange code with token: {:?}", err);
            Redirect::to(&format!("{}/auth?error=2", state.env.client_url))
        })?;

    // Get the user's profile.
    let user_profile = oauth_config
        .get_user_info(token.access_token().secret())
        .await
        .map_err(|err| {
            tracing::error!("Failed to get user profile: {:?}", err);
            Redirect::to(&format!("{}/auth?error=2", state.env.client_url))
        })?;

    /*
     * This is where you implement you own logic to create or update a user in your database.
     * Here we are creating the jwt token with the user data and sending it to the client.
     * The client will act as a gateway to the other services and use it to create a new user.
     */
    let jwt_token = oauth_config.generate_jwt(user_profile).map_err(|err| {
        tracing::error!("Failed to generate JWT: {:?}", err);
        Redirect::to(&format!("{}/auth?error=2", state.env.client_url))
    })?;

    tracing::info!("User authenticated");
    Ok(Redirect::to(&format!(
        "{}/?oauth_token={}",
        state.env.client_url, jwt_token
    )))
}
