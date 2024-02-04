use anyhow::Result;
use oauth2::{basic::BasicClient, AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl};
use service_auth::Env;
use serde::{Deserialize, Serialize};
use tonic::metadata::{Ascii, MetadataValue};

pub enum OAuthProvider {
    Google,
    Github,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OAuthUser {
    pub sub: String,
    pub email: String,
    pub avatar: String,
    pub exp: i64,
}

pub trait OAuth {
    fn get_config_by_provider(provider: &str, env: Env) -> Result<Self>
    where
        Self: Sized;
    fn build_oauth_client(&self) -> BasicClient;
    async fn get_user_info(&self, token: &str) -> Result<OAuthUser>;
    fn generate_jwt(&self, user: OAuthUser) -> Result<MetadataValue<Ascii>>;
}

pub struct OAuthConfig {
    provider: OAuthProvider,
    client_id: String,
    client_secret: String,
    auth_url: String,
    token_url: String,
    redirect_url: String,
    pub scopes: Vec<String>,
    user_info_url: String,
    jwt_secret: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct GoogleUser {
    sub: String,
    email: String,
    email_verified: bool,
    picture: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct GithubUser {
    id: i64,
    email: String,
    avatar_url: String,
}

impl OAuth for OAuthConfig {
    fn get_config_by_provider(provider: &str, env: Env) -> Result<Self> {
        match provider {
            "google" => Ok(Self {
                provider: OAuthProvider::Google,
                client_id: env.google_client_id,
                client_secret: env.google_client_secret,
                auth_url: "https://accounts.google.com/o/oauth2/v2/auth".to_string(),
                token_url: "https://www.googleapis.com/oauth2/v3/token".to_string(),
                redirect_url: format!("{}/oauth-callback/google", env.auth_url),
                user_info_url: "https://www.googleapis.com/oauth2/v3/userinfo".to_string(),
                scopes: vec!["email".to_string(), "openid".to_string()],
                jwt_secret: env.jwt_secret,
            }),
            "github" => Ok(Self {
                provider: OAuthProvider::Github,
                client_id: env.github_client_id,
                client_secret: env.github_client_secret,
                auth_url: "https://github.com/login/oauth/authorize".to_string(),
                token_url: "https://github.com/login/oauth/access_token".to_string(),
                redirect_url: format!("{}/oauth-callback/github", env.auth_url),
                user_info_url: "https://api.github.com/user".to_string(),
                scopes: vec!["user:email".to_string()],
                jwt_secret: env.jwt_secret,
            }),
            _ => Err(anyhow::anyhow!(format!(
                "Invalid OAuth provider: {}",
                provider
            ))),
        }
    }
    fn build_oauth_client(&self) -> BasicClient {
        let auth_url =
            AuthUrl::new(self.auth_url.to_owned()).expect("Invalid authorization endpoint URL");
        let token_url =
            TokenUrl::new(self.token_url.to_owned()).expect("Invalid token endpoint URL");
        let redirect_url =
            RedirectUrl::new(self.redirect_url.to_owned()).expect("Invalid redirect URL");
        BasicClient::new(
            ClientId::new(self.client_id.to_owned()),
            Some(ClientSecret::new(self.client_secret.to_owned())),
            auth_url,
            Some(token_url),
        )
        .set_redirect_uri(redirect_url)
    }

    async fn get_user_info(&self, token: &str) -> Result<OAuthUser> {
        match self.provider {
            OAuthProvider::Google => {
                let user_profile = reqwest::Client::new()
                    .get(&self.user_info_url)
                    .header(reqwest::header::AUTHORIZATION, format!("Bearer {}", token))
                    .send()
                    .await?;

                let user_profile = user_profile.json::<GoogleUser>().await?;
                if !user_profile.email_verified {
                    return Err(anyhow::anyhow!("User email is not verified"));
                }
                Ok(OAuthUser {
                    sub: user_profile.sub,
                    email: user_profile.email,
                    avatar: user_profile.picture,
                    // 5 min
                    exp: time::OffsetDateTime::now_utc().unix_timestamp() + 60 * 5,
                })
            }
            OAuthProvider::Github => {
                let user_profile = reqwest::Client::new()
                    .get(&self.user_info_url)
                    .header(reqwest::header::AUTHORIZATION, format!("Bearer {}", token))
                    .header(reqwest::header::ACCEPT, "application/vnd.github.v3+json")
                    .header(reqwest::header::USER_AGENT, "Rusve")
                    .send()
                    .await?;
                let user_profile = user_profile.json::<GithubUser>().await?;
                Ok(OAuthUser {
                    sub: user_profile.id.to_string(),
                    email: user_profile.email,
                    avatar: user_profile.avatar_url,
                    // 5 min
                    exp: time::OffsetDateTime::now_utc().unix_timestamp() + 60 * 5,
                })
            }
        }
    }

    fn generate_jwt(&self, user: OAuthUser) -> Result<MetadataValue<Ascii>> {
        let token = jsonwebtoken::encode(
            &jsonwebtoken::Header::new(jsonwebtoken::Algorithm::HS256),
            &user,
            &jsonwebtoken::EncodingKey::from_secret(self.jwt_secret.as_bytes()),
        )?;
        Ok(format!("bearer {}", token).parse()?)
    }
}
