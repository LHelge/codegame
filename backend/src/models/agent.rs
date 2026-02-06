use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// Represents a user's AI agent for a specific game.
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Agent {
    pub id: i64,
    pub user_id: i64,
    pub game_id: i64,
    pub name: String,
    pub code: String,
    pub created_at: String,
    pub updated_at: String,
}

/// Request payload for creating a new agent.
#[derive(Debug, Deserialize)]
pub struct CreateAgentRequest {
    pub game_id: i64,
    pub name: String,
    #[serde(default)]
    pub code: String,
}

/// Request payload for updating an existing agent.
#[derive(Debug, Deserialize)]
pub struct UpdateAgentRequest {
    pub name: Option<String>,
    pub code: Option<String>,
}
