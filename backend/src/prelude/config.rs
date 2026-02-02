use std::num::ParseIntError;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Missing environment variable: {0}")]
    MissingEnvVar(String),
    #[error("Invalid port: {0}")]
    InvalidPort(#[from] ParseIntError),
}

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub server_port: u16,
    pub jwt_secret: String,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        let database_url = std::env::var("DATABASE_URL")
            .map_err(|_| ConfigError::MissingEnvVar("DATABASE_URL".to_string()))?;

        let server_port = std::env::var("SERVER_PORT")
            .map_err(|_| ConfigError::MissingEnvVar("SERVER_PORT".to_string()))?
            .parse()?;

        let jwt_secret = std::env::var("JWT_SECRET")
            .map_err(|_| ConfigError::MissingEnvVar("JWT_SECRET".to_string()))?;

        Ok(Config {
            database_url,
            server_port,
            jwt_secret,
        })
    }
}
