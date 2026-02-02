use super::config::ConfigError;
use axum::{http::StatusCode, response::IntoResponse};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Configuration error: {0}")]
    ConfigError(#[from] ConfigError),

    #[error("Claims error: {0}")]
    ClaimsError(#[from] crate::prelude::ClaimsError),

    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),

    #[error("User error: {0}")]
    UserError(#[from] crate::models::UserError),

    #[error("Not found")]
    NotFound,
}

pub type Result<T> = std::result::Result<T, Error>;

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        match self {
            Error::ClaimsError(e) => e.into_response(),
            Error::NotFound => StatusCode::NOT_FOUND.into_response(),
            _ => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }
}
