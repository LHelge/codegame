use argon2::{
    Argon2, PasswordHash,
    password_hash::{PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum UserError {
    #[error("Password hashing error: {0}")]
    PasswordHashError(#[from] argon2::password_hash::Error),

    #[error("Invalid password")]
    InvalidPassword,
}

pub type Result<T> = std::result::Result<T, UserError>;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i64,

    pub username: String,

    #[serde(skip_serializing)]
    pub password_hash: String,

    pub admin: bool,
}

impl User {
    pub fn new(id: i64, username: impl Into<String>, password: &str, admin: bool) -> Result<Self> {
        let salt = SaltString::generate(&mut OsRng);
        let password_hash = Argon2::default()
            .hash_password(password.as_bytes(), &salt)?
            .to_string();

        Ok(Self {
            id,
            username: username.into(),
            password_hash,
            admin,
        })
    }

    pub fn verify_password(&self, password: &str) -> Result<()> {
        let parsed_hash = PasswordHash::new(&self.password_hash)?;
        Argon2::default().verify_password(password.as_bytes(), &parsed_hash)?;
        Ok(())
    }
}
