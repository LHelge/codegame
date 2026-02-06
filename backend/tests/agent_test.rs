//! Integration tests for agent endpoints.

mod common;

use axum_extra::extract::cookie::Cookie;
use axum_test::TestServer;
use backend::models::Agent;
use backend::prelude::AppState;
use backend::repositories::{GameRepository, UserRepository};
use backend::routes;
use serde_json::json;

/// Helper to create a test server with a pre-configured database.
async fn setup_server() -> (TestServer, AppState) {
    let config = common::test_config();
    let db = common::test_db().await;
    let state = AppState::new(config, db);
    let app = routes::routes().with_state(state.clone());
    let server = TestServer::new(app).unwrap();
    (server, state)
}

/// Helper to create a test user and return their token.
async fn create_user_with_token(state: &AppState, username: &str) -> (i64, String) {
    let repo = UserRepository::new(&state.db);
    let user = repo
        .create(username, "Password123!", false)
        .await
        .expect("Failed to create user");
    let token = common::create_test_token(user.id, false, username, &state.config.jwt_secret);
    (user.id, token)
}

/// Helper to get game ID for robotsumo (seeded game).
async fn get_robotsumo_game_id(state: &AppState) -> i64 {
    let repo = GameRepository::new(&state.db);
    let game = repo
        .find_by_name("robotsumo")
        .await
        .expect("Failed to query game")
        .expect("robotsumo game should exist");
    game.id
}

// ============================================================================
// Create Agent Tests
// ============================================================================

#[tokio::test]
async fn create_agent_succeeds() {
    let (server, state) = setup_server().await;
    let (user_id, token) = create_user_with_token(&state, "testuser").await;
    let game_id = get_robotsumo_game_id(&state).await;

    let response = server
        .post("/agents")
        .add_cookie(Cookie::new("token", token))
        .json(&json!({
            "game_id": game_id,
            "name": "My First Agent",
            "code": "-- Lua code here"
        }))
        .await;

    response.assert_status_ok();
    let agent: Agent = response.json();
    assert_eq!(agent.name, "My First Agent");
    assert_eq!(agent.code, "-- Lua code here");
    assert_eq!(agent.user_id, user_id);
    assert_eq!(agent.game_id, game_id);
}

#[tokio::test]
async fn create_agent_without_auth_fails() {
    let (server, state) = setup_server().await;
    let game_id = get_robotsumo_game_id(&state).await;

    let response = server
        .post("/agents")
        .json(&json!({
            "game_id": game_id,
            "name": "Agent",
            "code": "-- code"
        }))
        .await;

    response.assert_status_unauthorized();
}

#[tokio::test]
async fn create_agent_with_empty_code_fails() {
    let (server, state) = setup_server().await;
    let (_user_id, token) = create_user_with_token(&state, "testuser").await;
    let game_id = get_robotsumo_game_id(&state).await;

    let response = server
        .post("/agents")
        .add_cookie(Cookie::new("token", token))
        .json(&json!({
            "game_id": game_id,
            "name": "Empty Agent"
        }))
        .await;

    response.assert_status_bad_request();
}

#[tokio::test]
async fn create_agent_with_whitespace_code_fails() {
    let (server, state) = setup_server().await;
    let (_user_id, token) = create_user_with_token(&state, "testuser").await;
    let game_id = get_robotsumo_game_id(&state).await;

    let response = server
        .post("/agents")
        .add_cookie(Cookie::new("token", token))
        .json(&json!({
            "game_id": game_id,
            "name": "Empty Agent",
            "code": "   \n\t  "
        }))
        .await;

    response.assert_status_bad_request();
}

#[tokio::test]
async fn create_duplicate_agent_name_fails() {
    let (server, state) = setup_server().await;
    let (_user_id, token) = create_user_with_token(&state, "testuser").await;
    let game_id = get_robotsumo_game_id(&state).await;

    // Create first agent
    server
        .post("/agents")
        .add_cookie(Cookie::new("token", token.clone()))
        .json(&json!({
            "game_id": game_id,
            "name": "Duplicate Name",
            "code": "-- code"
        }))
        .await
        .assert_status_ok();

    // Try to create another with same name
    let response = server
        .post("/agents")
        .add_cookie(Cookie::new("token", token))
        .json(&json!({
            "game_id": game_id,
            "name": "Duplicate Name",
            "code": "-- code"
        }))
        .await;

    response.assert_status_not_ok();
}

#[tokio::test]
async fn create_agent_with_empty_name_fails() {
    let (server, state) = setup_server().await;
    let (_user_id, token) = create_user_with_token(&state, "testuser").await;
    let game_id = get_robotsumo_game_id(&state).await;

    let response = server
        .post("/agents")
        .add_cookie(Cookie::new("token", token))
        .json(&json!({
            "game_id": game_id,
            "name": "",
            "code": "-- code"
        }))
        .await;

    response.assert_status_bad_request();
}

#[tokio::test]
async fn create_agent_with_whitespace_name_fails() {
    let (server, state) = setup_server().await;
    let (_user_id, token) = create_user_with_token(&state, "testuser").await;
    let game_id = get_robotsumo_game_id(&state).await;

    let response = server
        .post("/agents")
        .add_cookie(Cookie::new("token", token))
        .json(&json!({
            "game_id": game_id,
            "name": "   ",
            "code": "-- code"
        }))
        .await;

    response.assert_status_bad_request();
}

#[tokio::test]
async fn create_agent_with_long_name_fails() {
    let (server, state) = setup_server().await;
    let (_user_id, token) = create_user_with_token(&state, "testuser").await;
    let game_id = get_robotsumo_game_id(&state).await;

    let long_name = "a".repeat(51);
    let response = server
        .post("/agents")
        .add_cookie(Cookie::new("token", token))
        .json(&json!({
            "game_id": game_id,
            "name": long_name,
            "code": "-- code"
        }))
        .await;

    response.assert_status_bad_request();
}

#[tokio::test]
async fn create_agent_with_invalid_characters_fails() {
    let (server, state) = setup_server().await;
    let (_user_id, token) = create_user_with_token(&state, "testuser").await;
    let game_id = get_robotsumo_game_id(&state).await;

    let response = server
        .post("/agents")
        .add_cookie(Cookie::new("token", token))
        .json(&json!({
            "game_id": game_id,
            "name": "Agent!@#$",
            "code": "-- code"
        }))
        .await;

    response.assert_status_bad_request();
}

#[tokio::test]
async fn create_agent_with_invalid_lua_syntax_fails() {
    let (server, state) = setup_server().await;
    let (_user_id, token) = create_user_with_token(&state, "testuser").await;
    let game_id = get_robotsumo_game_id(&state).await;

    let response = server
        .post("/agents")
        .add_cookie(Cookie::new("token", token))
        .json(&json!({
            "game_id": game_id,
            "name": "Broken Agent",
            "code": "function broken()"  // missing 'end'
        }))
        .await;

    response.assert_status_bad_request();
}

// ============================================================================
// List Agents Tests
// ============================================================================

#[tokio::test]
async fn list_agents_returns_user_agents() {
    let (server, state) = setup_server().await;
    let (_user_id, token) = create_user_with_token(&state, "testuser").await;
    let game_id = get_robotsumo_game_id(&state).await;

    // Create some agents
    for name in ["Agent A", "Agent B", "Agent C"] {
        server
            .post("/agents")
            .add_cookie(Cookie::new("token", token.clone()))
            .json(&json!({
                "game_id": game_id,
                "name": name,
                "code": "-- code"
            }))
            .await
            .assert_status_ok();
    }

    // List agents
    let response = server
        .get(&format!("/agents?game_id={}", game_id))
        .add_cookie(Cookie::new("token", token))
        .await;

    response.assert_status_ok();
    let agents: Vec<Agent> = response.json();
    assert_eq!(agents.len(), 3);
}

#[tokio::test]
async fn list_agents_without_auth_fails() {
    let (server, state) = setup_server().await;
    let game_id = get_robotsumo_game_id(&state).await;

    let response = server.get(&format!("/agents?game_id={}", game_id)).await;
    response.assert_status_unauthorized();
}

#[tokio::test]
async fn list_agents_only_returns_own_agents() {
    let (server, state) = setup_server().await;
    let (_user1_id, token1) = create_user_with_token(&state, "user1").await;
    let (_user2_id, token2) = create_user_with_token(&state, "user2").await;
    let game_id = get_robotsumo_game_id(&state).await;

    // User 1 creates agents
    for name in ["User1 Agent A", "User1 Agent B"] {
        server
            .post("/agents")
            .add_cookie(Cookie::new("token", token1.clone()))
            .json(&json!({
                "game_id": game_id,
                "name": name,
                "code": "-- code"
            }))
            .await
            .assert_status_ok();
    }

    // User 2 creates an agent
    server
        .post("/agents")
        .add_cookie(Cookie::new("token", token2.clone()))
        .json(&json!({
            "game_id": game_id,
            "name": "User2 Agent",
            "code": "-- code"
        }))
        .await
        .assert_status_ok();

    // User 1 should only see their agents
    let response = server
        .get(&format!("/agents?game_id={}", game_id))
        .add_cookie(Cookie::new("token", token1))
        .await;

    let agents: Vec<Agent> = response.json();
    assert_eq!(agents.len(), 2);
    assert!(agents.iter().all(|a| a.name.starts_with("User1")));

    // User 2 should only see their agent
    let response = server
        .get(&format!("/agents?game_id={}", game_id))
        .add_cookie(Cookie::new("token", token2))
        .await;

    let agents: Vec<Agent> = response.json();
    assert_eq!(agents.len(), 1);
    assert_eq!(agents[0].name, "User2 Agent");
}

// ============================================================================
// Get Agent Tests
// ============================================================================

#[tokio::test]
async fn get_agent_returns_agent() {
    let (server, state) = setup_server().await;
    let (_user_id, token) = create_user_with_token(&state, "testuser").await;
    let game_id = get_robotsumo_game_id(&state).await;

    // Create agent
    let create_response = server
        .post("/agents")
        .add_cookie(Cookie::new("token", token.clone()))
        .json(&json!({
            "game_id": game_id,
            "name": "My Agent",
            "code": "function think() end"
        }))
        .await;

    let created: Agent = create_response.json();

    // Get agent
    let response = server
        .get(&format!("/agents/{}", created.id))
        .add_cookie(Cookie::new("token", token))
        .await;

    response.assert_status_ok();
    let agent: Agent = response.json();
    assert_eq!(agent.id, created.id);
    assert_eq!(agent.name, "My Agent");
}

#[tokio::test]
async fn get_other_users_agent_fails() {
    let (server, state) = setup_server().await;
    let (_user1_id, token1) = create_user_with_token(&state, "user1").await;
    let (_user2_id, token2) = create_user_with_token(&state, "user2").await;
    let game_id = get_robotsumo_game_id(&state).await;

    // User 1 creates agent
    let create_response = server
        .post("/agents")
        .add_cookie(Cookie::new("token", token1))
        .json(&json!({
            "game_id": game_id,
            "name": "Private Agent",
            "code": "-- code"
        }))
        .await;

    let created: Agent = create_response.json();

    // User 2 tries to get it
    let response = server
        .get(&format!("/agents/{}", created.id))
        .add_cookie(Cookie::new("token", token2))
        .await;

    response.assert_status_not_found();
}

// ============================================================================
// Update Agent Tests
// ============================================================================

#[tokio::test]
async fn update_agent_name_succeeds() {
    let (server, state) = setup_server().await;
    let (_user_id, token) = create_user_with_token(&state, "testuser").await;
    let game_id = get_robotsumo_game_id(&state).await;

    // Create agent
    let create_response = server
        .post("/agents")
        .add_cookie(Cookie::new("token", token.clone()))
        .json(&json!({
            "game_id": game_id,
            "name": "Original Name",
            "code": "-- original code"
        }))
        .await;

    let created: Agent = create_response.json();

    // Update name only
    let response = server
        .put(&format!("/agents/{}", created.id))
        .add_cookie(Cookie::new("token", token))
        .json(&json!({
            "name": "New Name"
        }))
        .await;

    response.assert_status_ok();
    let updated: Agent = response.json();
    assert_eq!(updated.name, "New Name");
    assert_eq!(updated.code, "-- original code");
}

#[tokio::test]
async fn update_agent_code_succeeds() {
    let (server, state) = setup_server().await;
    let (_user_id, token) = create_user_with_token(&state, "testuser").await;
    let game_id = get_robotsumo_game_id(&state).await;

    // Create agent
    let create_response = server
        .post("/agents")
        .add_cookie(Cookie::new("token", token.clone()))
        .json(&json!({
            "game_id": game_id,
            "name": "My Agent",
            "code": "-- original code"
        }))
        .await;

    let created: Agent = create_response.json();

    // Update code
    let response = server
        .put(&format!("/agents/{}", created.id))
        .add_cookie(Cookie::new("token", token))
        .json(&json!({
            "code": "-- new code"
        }))
        .await;

    response.assert_status_ok();
    let updated: Agent = response.json();
    assert_eq!(updated.code, "-- new code");
}

#[tokio::test]
async fn update_other_users_agent_fails() {
    let (server, state) = setup_server().await;
    let (_user1_id, token1) = create_user_with_token(&state, "user1").await;
    let (_user2_id, token2) = create_user_with_token(&state, "user2").await;
    let game_id = get_robotsumo_game_id(&state).await;

    // User 1 creates agent
    let create_response = server
        .post("/agents")
        .add_cookie(Cookie::new("token", token1))
        .json(&json!({
            "game_id": game_id,
            "name": "Private Agent",
            "code": "-- code"
        }))
        .await;

    let created: Agent = create_response.json();

    // User 2 tries to update it
    let response = server
        .put(&format!("/agents/{}", created.id))
        .add_cookie(Cookie::new("token", token2))
        .json(&json!({
            "name": "Hacked"
        }))
        .await;

    response.assert_status_not_found();
}

#[tokio::test]
async fn update_agent_with_invalid_name_fails() {
    let (server, state) = setup_server().await;
    let (_user_id, token) = create_user_with_token(&state, "testuser").await;
    let game_id = get_robotsumo_game_id(&state).await;

    // Create agent
    let create_response = server
        .post("/agents")
        .add_cookie(Cookie::new("token", token.clone()))
        .json(&json!({
            "game_id": game_id,
            "name": "Valid Name",
            "code": "-- code"
        }))
        .await;

    let created: Agent = create_response.json();

    // Try to update with invalid name
    let response = server
        .put(&format!("/agents/{}", created.id))
        .add_cookie(Cookie::new("token", token))
        .json(&json!({
            "name": "Invalid!@#$"
        }))
        .await;

    response.assert_status_bad_request();
}

#[tokio::test]
async fn update_agent_with_empty_code_fails() {
    let (server, state) = setup_server().await;
    let (_user_id, token) = create_user_with_token(&state, "testuser").await;
    let game_id = get_robotsumo_game_id(&state).await;

    // Create agent
    let create_response = server
        .post("/agents")
        .add_cookie(Cookie::new("token", token.clone()))
        .json(&json!({
            "game_id": game_id,
            "name": "Valid Name",
            "code": "-- valid code"
        }))
        .await;

    let created: Agent = create_response.json();

    // Try to update with empty code
    let response = server
        .put(&format!("/agents/{}", created.id))
        .add_cookie(Cookie::new("token", token))
        .json(&json!({
            "code": ""
        }))
        .await;

    response.assert_status_bad_request();
}

// ============================================================================
// Delete Agent Tests
// ============================================================================

#[tokio::test]
async fn delete_agent_succeeds() {
    let (server, state) = setup_server().await;
    let (_user_id, token) = create_user_with_token(&state, "testuser").await;
    let game_id = get_robotsumo_game_id(&state).await;

    // Create agent
    let create_response = server
        .post("/agents")
        .add_cookie(Cookie::new("token", token.clone()))
        .json(&json!({
            "game_id": game_id,
            "name": "To Delete",
            "code": "-- code"
        }))
        .await;

    let created: Agent = create_response.json();

    // Delete agent
    let response = server
        .delete(&format!("/agents/{}", created.id))
        .add_cookie(Cookie::new("token", token.clone()))
        .await;

    response.assert_status_ok();

    // Verify it's gone
    let get_response = server
        .get(&format!("/agents/{}", created.id))
        .add_cookie(Cookie::new("token", token))
        .await;

    get_response.assert_status_not_found();
}

#[tokio::test]
async fn delete_other_users_agent_fails() {
    let (server, state) = setup_server().await;
    let (_user1_id, token1) = create_user_with_token(&state, "user1").await;
    let (_user2_id, token2) = create_user_with_token(&state, "user2").await;
    let game_id = get_robotsumo_game_id(&state).await;

    // User 1 creates agent
    let create_response = server
        .post("/agents")
        .add_cookie(Cookie::new("token", token1.clone()))
        .json(&json!({
            "game_id": game_id,
            "name": "Private Agent",
            "code": "-- code"
        }))
        .await;

    let created: Agent = create_response.json();

    // User 2 tries to delete it
    let response = server
        .delete(&format!("/agents/{}", created.id))
        .add_cookie(Cookie::new("token", token2))
        .await;

    response.assert_status_not_found();

    // Verify it still exists
    let get_response = server
        .get(&format!("/agents/{}", created.id))
        .add_cookie(Cookie::new("token", token1))
        .await;

    get_response.assert_status_ok();
}

#[tokio::test]
async fn delete_nonexistent_agent_returns_not_found() {
    let (server, state) = setup_server().await;
    let (_user_id, token) = create_user_with_token(&state, "testuser").await;

    let response = server
        .delete("/agents/99999")
        .add_cookie(Cookie::new("token", token))
        .await;

    response.assert_status_not_found();
}
