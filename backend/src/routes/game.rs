use crate::models::Game;
use crate::prelude::*;
use crate::repositories::GameRepository;
use axum::{
    Json, Router,
    extract::{Path, State},
    routing::get,
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(list_games))
        .route("/{id}", get(get_game))
}

/// List all available games.
async fn list_games(State(state): State<AppState>) -> Result<Json<Vec<Game>>> {
    let repo = GameRepository::new(&state.db);
    let games = repo.find_all().await?;
    Ok(Json(games))
}

/// Get a specific game by ID.
async fn get_game(State(state): State<AppState>, Path(id): Path<i64>) -> Result<Json<Game>> {
    let repo = GameRepository::new(&state.db);
    let game = repo.find_by_id(id).await?.ok_or(Error::NotFound)?;
    Ok(Json(game))
}
