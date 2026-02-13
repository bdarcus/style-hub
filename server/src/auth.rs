use oauth2::{
    basic::BasicClient, AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};
use chrono::{Utc, Duration};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,
    pub exp: i64,
    pub role: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub role: String,
}

pub fn create_oauth_client() -> BasicClient {
    let client_id = std::env::var("GITHUB_CLIENT_ID").expect("GITHUB_CLIENT_ID must be set");
    let client_secret = std::env::var("GITHUB_CLIENT_SECRET").expect("GITHUB_CLIENT_SECRET must be set");
    
    BasicClient::new(
        ClientId::new(client_id),
        Some(ClientSecret::new(client_secret)),
        AuthUrl::new("https://github.com/login/oauth/authorize".to_string()).unwrap(),
        Some(TokenUrl::new("https://github.com/login/oauth/access_token".to_string()).unwrap()),
    )
    .set_redirect_uri(RedirectUrl::new(std::env::var("REDIRECT_URL").expect("REDIRECT_URL must be set")).unwrap())
}

pub fn create_jwt(user_id: Uuid, role: &str) -> String {
    let expiration = Utc::now()
        .checked_add_signed(Duration::days(7))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: user_id,
        exp: expiration,
        role: role.to_string(),
    };

    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref())).unwrap()
}

pub fn decode_jwt(token: &str) -> Result<Claims, String> {
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::new(Algorithm::HS256),
    )
    .map(|data| data.claims)
    .map_err(|e| e.to_string())
}

#[derive(Debug, Deserialize)]
pub struct GithubUser {
    pub id: i64,
    pub email: Option<String>,
    pub login: String,
}

use axum::{
    extract::FromRequestParts,
    http::request::Parts,
    async_trait,
};

pub struct AuthenticatedUser {
    pub id: Uuid,
    pub role: String,
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthenticatedUser
where
    S: Send + Sync,
{
    type Rejection = (axum::http::StatusCode, String);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let auth_header = parts.headers
            .get(axum::http::header::AUTHORIZATION)
            .and_then(|h| h.to_str().ok())
            .ok_or((axum::http::StatusCode::UNAUTHORIZED, "Missing authorization header".to_string()))?;

        if !auth_header.starts_with("Bearer ") {
            return Err((axum::http::StatusCode::UNAUTHORIZED, "Invalid authorization header".to_string()));
        }

        let token = &auth_header[7..];
        let claims = decode_jwt(token)
            .map_err(|e| (axum::http::StatusCode::UNAUTHORIZED, format!("Invalid token: {}", e)))?;

        Ok(AuthenticatedUser {
            id: claims.sub,
            role: claims.role,
        })
    }
}
