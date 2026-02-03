use crate::prelude::*;
use axum::{Router, routing::get};

pub fn routes() -> Router<AppState> {
    Router::new().route("/", get(hello))
}

async fn hello() -> &'static str {
    "hello world"
}
