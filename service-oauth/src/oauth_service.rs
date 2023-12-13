use std::{collections::HashMap, sync::Arc};

use axum::{
    extract::{Query, State},
    response::Redirect,
    Extension, Json,
};
use http::StatusCode;
use oauth2::{
    basic::BasicClient, AuthUrl, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge, RedirectUrl,
    Scope, TokenResponse, TokenUrl,
};

use crate::{
    oauth_db::{create_pkce, select_pkce_by_csrf},
    AppState,
};

struct OAuthCallbackQuery {
    code: String,
    pkce_verifier: String,
    state: String,
}

pub fn build_oauth_client(client_id: String, client_secret: String) -> BasicClient {
    // In prod, http://localhost:8000 would get replaced by whatever your production URL is
    let redirect_url = "http://127.0.0.1:8090/oauth-callback/google".to_string();

    // If you're not using Google OAuth, you can use whatever the relevant auth/token URL is for your given OAuth service
    let auth_url = AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string())
        .expect("Invalid authorization endpoint URL");
    let token_url = TokenUrl::new("https://www.googleapis.com/oauth2/v3/token".to_string())
        .expect("Invalid token endpoint URL");

    BasicClient::new(
        ClientId::new(client_id),
        Some(ClientSecret::new(client_secret)),
        auth_url,
        Some(token_url),
    )
    .set_redirect_uri(RedirectUrl::new(redirect_url).unwrap())
}

pub async fn oauth_auth(
    State(state): State<Arc<AppState>>,
    Extension(client): Extension<BasicClient>,
) -> Result<Redirect, StatusCode> {
    // Generate a PKCE challenge.
    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    // Generate the full authorization URL.
    let (auth_url, csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        // Set the desired scopes.
        .add_scope(Scope::new("openid".to_string()))
        // .add_scope(Scope::new("write".to_string()))
        // Set the PKCE code challenge.
        .set_pkce_challenge(pkce_challenge)
        .url();

    // Save the CSRF token to the database.
    match create_pkce(
        &state.db_pool,
        &csrf_token.secret(),
        &pkce_verifier.secret(),
    )
    .await
    {
        Ok(_) => {}
        Err(err) => {
            tracing::error!("Failed to save PKCE verifier: {:?}", err);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    }

    Ok(Redirect::temporary(&auth_url.to_string()))
}

pub async fn oauth_callback(
    State(state): State<Arc<AppState>>,
    Query(query): Query<HashMap<String, String>>,
    Extension(client): Extension<BasicClient>,
) -> Result<(StatusCode, Json<String>), StatusCode> {
    let code = match query.get("code") {
        Some(code) => code,
        None => return Err(StatusCode::BAD_REQUEST),
    };
    let csrf = match query.get("state") {
        Some(state) => state,
        None => return Err(StatusCode::BAD_REQUEST),
    };

    let pkce = match select_pkce_by_csrf(&state.db_pool, csrf).await {
        Ok(Some(pkce)) => pkce,
        Ok(None) => return Err(StatusCode::BAD_REQUEST),
        Err(err) => {
            tracing::error!("Failed to select PKCE verifier: {:?}", err);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };
    // Exchange the code with a token.
    let token_result = client
        .exchange_code(oauth2::AuthorizationCode::new(code.to_string()))
        // Set the PKCE code verifier.
        .set_pkce_verifier(oauth2::PkceCodeVerifier::new(pkce.pkce_verifier))
        .request_async(oauth2::reqwest::async_http_client)
        .await;

    match token_result {
        Ok(token) => Ok((StatusCode::OK, Json(format!("{:?}", token)))),
        Err(err) => {
            tracing::error!("Failed to exchange code for token: {:?}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
