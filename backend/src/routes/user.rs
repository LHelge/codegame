use crate::models::User;
use crate::prelude::*;
use axum::{
    Json, Router,
    extract::{Path, State},
    routing::get,
};
use axum_extra::extract::{
    CookieJar,
    cookie::{Cookie, SameSite},
};
use chrono::Duration;
use serde::Deserialize;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(me))
        .route("/user/:id", get(get_user))
        .route("/auth", get(authenticate))
}

async fn me(State(_state): State<AppState>, _claims: Claims) -> Result<Json<User>> {
    // TODO: Fetch user from database
    let user = User::new(1, "Test", "test", true)?;
    Ok(Json(user))
}

async fn get_user(State(_state): State<AppState>, Path(id): Path<i64>) -> Result<Json<User>> {
    // TODO: Fetch user from database
    let user = User::new(id, "Test", "test", false)?;
    Ok(Json(user))
}

#[derive(Deserialize)]
struct AuthRequest {
    username: String,
    password: String,
}

async fn authenticate(
    cookies: CookieJar,
    State(state): State<AppState>,
    Json(payload): Json<AuthRequest>,
) -> Result<(CookieJar, Json<User>)> {
    // TODO: Get user from database
    let user = User::new(1, payload.username, "test", true)?;

    user.verify_password(&payload.password)?;

    let token = Claims::new(user.id, user.admin, &user.username, Duration::hours(1))
        .encode(&state.config.jwt_secret)?;

    let cookie = Cookie::build(("token", token))
        .same_site(SameSite::Strict)
        .secure(true)
        .http_only(true)
        .build();

    Ok((cookies.add(cookie), Json(user)))
}
