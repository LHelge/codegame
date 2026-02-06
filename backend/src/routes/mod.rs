use crate::prelude::*;
use axum::Router;

mod game;
mod health;
mod user;

pub fn routes() -> Router<AppState> {
    Router::new()
        .nest("/games", game::routes())
        .nest("/health", health::routes())
        .nest("/users", user::routes())
}
