use crate::prelude::*;
use axum::Router;

mod health;
mod hello;
mod user;

pub fn routes() -> Router<AppState> {
    Router::new()
        .nest("/health", health::routes())
        .nest("/hello", hello::routes())
        .nest("/users", user::routes())
}
