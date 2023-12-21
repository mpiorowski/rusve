use axum::{
    extract::{Query, State},
    response::{Redirect, Response},
    Extension,
};
use http::{header, StatusCode};
use oauth2::{
    basic::BasicClient, AuthUrl, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge, RedirectUrl,
    Scope, TokenResponse, TokenUrl,
};
use std::{collections::HashMap, sync::Arc, time::Duration};

use crate::{
    oauth_db::{
        auth_user, create_pkce, create_token, delete_pkce_by_id, delete_token_by_user_id,
        select_pkce_by_csrf,
    },
    AppState,
};

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct GoogleUser {
    sub: String,
    email: String,
    email_verified: bool,
    picture: String,
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

pub async fn oauth_login(
    State(state): State<Arc<AppState>>,
    Extension(client): Extension<BasicClient>,
) -> Result<Redirect, StatusCode> {
    let conn = state.db_pool.get().await.map_err(|err| {
        tracing::error!("Failed to get DB connection: {:?}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // Generate a PKCE challenge.
    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    // Generate the full authorization URL.
    let (auth_url, csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        // Set the desired scopes.
        .add_scope(Scope::new("openid".to_string()))
        .add_scope(Scope::new("email".to_string()))
        // Set the PKCE code challenge.
        .set_pkce_challenge(pkce_challenge)
        .url();

    // Save the CSRF token to the database.
    match create_pkce(&conn, csrf_token.secret(), pkce_verifier.secret()).await {
        Ok(_) => {}
        Err(err) => {
            tracing::error!("Failed to save PKCE verifier: {:?}", err);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    }

    Ok(Redirect::to(auth_url.as_ref()))
}

pub async fn oauth_callback(
    State(state): State<Arc<AppState>>,
    Query(query): Query<HashMap<String, String>>,
    Extension(client): Extension<BasicClient>,
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

    let pkce = match select_pkce_by_csrf(&conn, csrf).await {
        Ok(Some(pkce)) => pkce,
        Ok(None) => return Err(StatusCode::BAD_REQUEST),
        Err(err) => {
            tracing::error!("Failed to select PKCE verifier: {:?}", err);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    // Delete the PKCE verifier from the database asynchronously.
    // If this fails, it's not a big deal.
    tokio::spawn(async move {
        let conn = state.db_pool.get().await.map_err(|err| {
            tracing::error!("Failed to get DB connection: {:?}", err);
        });
        if let Err(err) = delete_pkce_by_id(&conn.unwrap(), pkce.id).await {
            tracing::error!("Failed to delete PKCE verifier: {:?}", err);
        }
    });

    // check if the `created` field is older than 10 minutes
    let diff = time::OffsetDateTime::now_utc() - pkce.created;
    if diff.whole_minutes() > 10 {
        return Err(StatusCode::BAD_REQUEST);
    }

    // Exchange the code with a token.
    let token_result = client
        .exchange_code(oauth2::AuthorizationCode::new(code.to_string()))
        // Set the PKCE code verifier.
        .set_pkce_verifier(oauth2::PkceCodeVerifier::new(pkce.pkce_verifier))
        .request_async(oauth2::reqwest::async_http_client)
        .await;

    let auth_token = match token_result {
        Ok(token) => token,
        Err(err) => {
            tracing::error!("Failed to exchange code with token: {:?}", err);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    // Get the user's profile.
    let user_profile = reqwest::Client::new()
        .get("https://www.googleapis.com/oauth2/v3/userinfo")
        .header(
            reqwest::header::AUTHORIZATION,
            format!("Bearer {}", auth_token.access_token().secret()),
        )
        .send()
        .await;

    let user_profile = match user_profile {
        Ok(response) => response,
        Err(err) => {
            tracing::error!("Failed to get user profile: {:?}", err);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let user_profile = match user_profile.json::<GoogleUser>().await {
        Ok(profile) => profile,
        Err(err) => {
            tracing::error!("Failed to parse user profile: {:?}", err);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

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
    let user = match auth_user(&conn, &user_profile.sub, &user_profile.email).await {
        Ok(user) => user,
        Err(err) => {
            tracing::error!("Failed to authenticate user: {:?}", err);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    // Delete the user's existing token.
    if let Err(err) = delete_token_by_user_id(&conn, &user.id).await {
        tracing::error!("Failed to delete token: {:?}", err);
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }
    // Create a new token.
    let expires_in = auth_token.expires_in().unwrap_or(Duration::from_secs(3600));
    let token = match create_token(
        &conn,
        &user.id,
        auth_token.access_token().secret(),
        "",
        expires_in.as_secs_f32() as i32,
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

    Ok(Response::builder()
        .status(StatusCode::PERMANENT_REDIRECT)
        .header(
            header::SET_COOKIE,
            format!(
                "token={}; HttpOnly; Max-Age={}; Path=/; SameSite=Lax",
                token.id,
                // 7 days
                3600 * 24 * 7
            ),
        )
        .header(header::LOCATION, "http://127.0.0.1:3000")
        .body("".into())
        .unwrap())
}
