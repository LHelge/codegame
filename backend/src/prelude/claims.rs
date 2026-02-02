use crate::prelude::*;
use axum::{
    extract::FromRequestParts,
    http::{StatusCode, request::Parts},
    response::IntoResponse,
};
use axum_extra::extract::CookieJar;
use chrono::Duration;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ClaimsError {
    #[error("Token is missing")]
    TokenMissing,

    #[error("Token is invalid: {0}")]
    TokenInvalid(#[from] jsonwebtoken::errors::Error),
}

impl IntoResponse for ClaimsError {
    fn into_response(self) -> axum::response::Response {
        StatusCode::UNAUTHORIZED.into_response()
    }
}

pub type ClaimsResult<T> = std::result::Result<T, ClaimsError>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    exp: usize,
    iat: usize,
    pub user_id: i64,
    pub admin: bool,
    pub username: String,
}

impl Claims {
    pub fn new(user_id: i64, admin: bool, username: impl Into<String>, lifetime: Duration) -> Self {
        let iat = chrono::Utc::now();
        let exp = iat + lifetime;

        Self {
            exp: exp.timestamp() as usize,
            iat: iat.timestamp() as usize,
            user_id,
            admin,
            username: username.into(),
        }
    }

    pub fn encode(&self, secret: &str) -> ClaimsResult<String> {
        Ok(jsonwebtoken::encode(
            &Header::default(),
            self,
            &EncodingKey::from_secret(secret.as_ref()),
        )?)
    }

    pub fn decode(token: &str, secret: &str) -> ClaimsResult<Self> {
        let token = jsonwebtoken::decode::<Claims>(
            token,
            &DecodingKey::from_secret(secret.as_ref()),
            &Validation::default(),
        )?;

        Ok(token.claims)
    }
}

//#[async_trait]
impl FromRequestParts<AppState> for Claims {
    type Rejection = ClaimsError;

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> ClaimsResult<Self> {
        let cookies = CookieJar::from_headers(&parts.headers);

        if let Some(token) = cookies.get("token") {
            Ok(Claims::decode(token.value(), &state.config.jwt_secret)?)
        } else {
            Err(ClaimsError::TokenMissing)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::{AppState, Config};
    use axum::http::Request;

    fn test_state(secret: &str) -> AppState {
        let database_url = ":memory:".to_string();
        let db = sqlx::SqlitePool::connect_lazy(&database_url).unwrap();
        AppState::new(
            Config {
                database_url,
                server_port: 3000,
                jwt_secret: secret.to_string(),
            },
            db,
        )
    }

    #[test]
    fn claims_encode_decode_roundtrip() {
        let claims = Claims::new(42, true, "alice", Duration::minutes(5));
        let token = claims.encode("secret").expect("encode token");
        let decoded = Claims::decode(&token, "secret").expect("decode token");

        assert_eq!(decoded.user_id, 42);
        assert!(decoded.admin);
        assert_eq!(decoded.username, "alice");
    }

    #[test]
    fn claims_decode_fails_with_wrong_secret() {
        let claims = Claims::new(7, false, "bob", Duration::minutes(5));
        let token = claims.encode("secret-a").expect("encode token");

        let result = Claims::decode(&token, "secret-b");
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn from_request_parts_reads_cookie_token() {
        let state = test_state("secret");
        let claims = Claims::new(9, false, "carol", Duration::minutes(5));
        let token = claims.encode("secret").expect("encode token");

        let request = Request::builder()
            .header("cookie", format!("token={}", token))
            .body(())
            .expect("build request");
        let (mut parts, _) = request.into_parts();

        let extracted = Claims::from_request_parts(&mut parts, &state)
            .await
            .expect("extract claims");

        assert_eq!(extracted.user_id, 9);
        assert!(!extracted.admin);
        assert_eq!(extracted.username, "carol");
    }

    #[tokio::test]
    async fn from_request_parts_missing_cookie_returns_error() {
        let state = test_state("secret");
        let request = Request::builder().body(()).expect("build request");
        let (mut parts, _) = request.into_parts();

        let result = Claims::from_request_parts(&mut parts, &state).await;
        assert!(matches!(result, Err(ClaimsError::TokenMissing)));
    }

    #[test]
    fn claims_error_into_response_is_unauthorized() {
        let response = ClaimsError::TokenMissing.into_response();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }
}
