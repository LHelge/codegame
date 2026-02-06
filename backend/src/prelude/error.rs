use super::config::ConfigError;
use axum::{http::StatusCode, response::IntoResponse};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Configuration error: {0}")]
    Config(#[from] ConfigError),

    #[error("Claims error: {0}")]
    Claims(#[from] crate::prelude::ClaimsError),

    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Migration error: {0}")]
    Migration(#[from] sqlx::migrate::MigrateError),

    #[error("User error: {0}")]
    User(#[from] crate::models::UserError),

    #[error("Not found")]
    NotFound,
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(serde::Serialize)]
struct ErrorResponse {
    status: u16,
    error: String,
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        let status = match self {
            Error::Claims(_) => StatusCode::UNAUTHORIZED,
            Error::NotFound => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let error_response = ErrorResponse {
            status: status.as_u16(),
            error: self.to_string(),
        };

        (status, axum::Json(error_response)).into_response()
    }
}
