use crate::models::{Agent, CreateAgentRequest, UpdateAgentRequest};
use crate::prelude::*;
use crate::repositories::AgentRepository;
use axum::{
    Json, Router,
    extract::{Path, Query, State},
    routing::get,
};
use serde::Deserialize;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(list_agents).post(create_agent))
        .route(
            "/{id}",
            get(get_agent).put(update_agent).delete(delete_agent),
        )
}

#[derive(Deserialize)]
struct ListAgentsQuery {
    game_id: i64,
}

/// List all agents for the current user in a specific game.
async fn list_agents(
    State(state): State<AppState>,
    claims: Claims,
    Query(query): Query<ListAgentsQuery>,
) -> Result<Json<Vec<Agent>>> {
    let repo = AgentRepository::new(&state.db);
    let agents = repo
        .find_by_user_and_game(claims.user_id, query.game_id)
        .await?;
    Ok(Json(agents))
}

/// Create a new agent for the current user.
async fn create_agent(
    State(state): State<AppState>,
    claims: Claims,
    Json(payload): Json<CreateAgentRequest>,
) -> Result<Json<Agent>> {
    let repo = AgentRepository::new(&state.db);
    let agent = repo
        .create(
            claims.user_id,
            payload.game_id,
            &payload.name,
            &payload.code,
        )
        .await?;
    Ok(Json(agent))
}

/// Get a specific agent by ID (must belong to current user).
async fn get_agent(
    State(state): State<AppState>,
    claims: Claims,
    Path(id): Path<i64>,
) -> Result<Json<Agent>> {
    let repo = AgentRepository::new(&state.db);
    let agent = repo
        .find_by_id(id, claims.user_id)
        .await?
        .ok_or(Error::NotFound)?;
    Ok(Json(agent))
}

/// Update an agent (must belong to current user).
async fn update_agent(
    State(state): State<AppState>,
    claims: Claims,
    Path(id): Path<i64>,
    Json(payload): Json<UpdateAgentRequest>,
) -> Result<Json<Agent>> {
    let repo = AgentRepository::new(&state.db);
    let agent = repo
        .update(
            id,
            claims.user_id,
            payload.name.as_deref(),
            payload.code.as_deref(),
        )
        .await?
        .ok_or(Error::NotFound)?;
    Ok(Json(agent))
}

/// Delete an agent (must belong to current user).
async fn delete_agent(
    State(state): State<AppState>,
    claims: Claims,
    Path(id): Path<i64>,
) -> Result<()> {
    let repo = AgentRepository::new(&state.db);
    let deleted = repo.delete(id, claims.user_id).await?;
    if deleted {
        Ok(())
    } else {
        Err(Error::NotFound)
    }
}
