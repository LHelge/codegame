use crate::prelude::*;
use axum::{Json, Router, routing::get};
use serde::Serialize;

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
}

pub fn routes() -> Router<AppState> {
    Router::new().route("/", get(health))
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse { status: "ok" })
}
