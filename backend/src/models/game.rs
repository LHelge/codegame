use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// Represents a game in the system.
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Game {
    pub id: i64,
    pub name: String,
    pub wasm_filename: String,
}
