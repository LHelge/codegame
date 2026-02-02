use crate::prelude::*;
use axum::{Router, routing::get};

pub fn routes() -> Router<AppState> {
    Router::new().route("/hello", get(hello))
}

async fn hello() -> &'static str {
    "hello world"
}
