use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// Represents a game in the system.
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Game {
    pub id: i64,
    /// URL-friendly unique identifier (matches directory name in games/)
    pub name: String,
    /// Human-friendly display name
    pub display_name: String,
}
