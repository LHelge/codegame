//! Integration tests for user endpoints.

mod common;

use axum_extra::extract::cookie::Cookie;
use axum_test::TestServer;
use backend::prelude::AppState;
use backend::repositories::UserRepository;
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

// ============================================================================
// Authentication Tests
// ============================================================================

#[tokio::test]
async fn auth_with_valid_credentials_succeeds() {
    let (server, state) = setup_server().await;

    // Create a user first
    let repo = UserRepository::new(&state.db);
    repo.create("testuser", "Password123!", false)
        .await
        .expect("Failed to create user");

    // Authenticate
    let response = server
        .post("/users/auth")
        .json(&json!({
            "username": "testuser",
            "password": "Password123!"
        }))
        .await;

    response.assert_status_ok();
    let body: serde_json::Value = response.json();
    assert_eq!(body["username"], "testuser");
    assert_eq!(body["admin"], false);

    // Check that a cookie was set
    let cookies = response.cookie("token");
    assert!(!cookies.value().is_empty(), "Token cookie should be set");
}

#[tokio::test]
async fn auth_with_invalid_password_fails() {
    let (server, state) = setup_server().await;

    // Create a user first
    let repo = UserRepository::new(&state.db);
    repo.create("testuser", "Password123!", false)
        .await
        .expect("Failed to create user");

    // Try to authenticate with wrong password
    let response = server
        .post("/users/auth")
        .json(&json!({
            "username": "testuser",
            "password": "wrongpassword"
        }))
        .await;

    response.assert_status_not_ok();
}

#[tokio::test]
async fn auth_with_nonexistent_user_fails() {
    let (server, _state) = setup_server().await;

    let response = server
        .post("/users/auth")
        .json(&json!({
            "username": "nonexistent",
            "password": "Password123!"
        }))
        .await;

    response.assert_status_not_ok();
}

// ============================================================================
// Me Endpoint Tests
// ============================================================================

#[tokio::test]
async fn me_with_valid_token_returns_user() {
    let (server, state) = setup_server().await;

    // Create a user first
    let repo = UserRepository::new(&state.db);
    let user = repo
        .create("testuser", "Password123!", false)
        .await
        .expect("Failed to create user");

    // Create auth token
    let token = common::create_test_token(user.id, false, "testuser", &state.config.jwt_secret);

    // Get current user
    let response = server
        .get("/users")
        .add_cookie(Cookie::new("token", token))
        .await;

    response.assert_status_ok();
    let body: serde_json::Value = response.json();
    assert_eq!(body["username"], "testuser");
    assert_eq!(body["id"], user.id);
}

#[tokio::test]
async fn me_without_token_returns_unauthorized() {
    let (server, _state) = setup_server().await;

    let response = server.get("/users").await;

    response.assert_status_unauthorized();
}

// ============================================================================
// List Users Tests (Admin Only)
// ============================================================================

#[tokio::test]
async fn list_users_as_admin_returns_all_users() {
    let (server, state) = setup_server().await;

    // Create some users
    let repo = UserRepository::new(&state.db);
    let admin = repo
        .create("admin", "Password123!", true)
        .await
        .expect("Failed to create admin");
    repo.create("user1", "Password123!", false)
        .await
        .expect("Failed to create user1");
    repo.create("user2", "Password123!", false)
        .await
        .expect("Failed to create user2");

    // Create admin token
    let token = common::create_test_token(admin.id, true, "admin", &state.config.jwt_secret);

    // List users
    let response = server
        .get("/users/users")
        .add_cookie(Cookie::new("token", token))
        .await;

    response.assert_status_ok();
    let body: Vec<serde_json::Value> = response.json();
    assert_eq!(body.len(), 3);
}

#[tokio::test]
async fn list_users_as_non_admin_returns_not_found() {
    let (server, state) = setup_server().await;

    // Create a regular user
    let repo = UserRepository::new(&state.db);
    let user = repo
        .create("testuser", "Password123!", false)
        .await
        .expect("Failed to create user");

    // Create non-admin token
    let token = common::create_test_token(user.id, false, "testuser", &state.config.jwt_secret);

    // Try to list users
    let response = server
        .get("/users/users")
        .add_cookie(Cookie::new("token", token))
        .await;

    response.assert_status_not_found();
}

// ============================================================================
// Create User Tests (Admin Only)
// ============================================================================

#[tokio::test]
async fn create_user_as_admin_succeeds() {
    let (server, state) = setup_server().await;

    // Create an admin user
    let repo = UserRepository::new(&state.db);
    let admin = repo
        .create("admin", "Password123!", true)
        .await
        .expect("Failed to create admin");

    // Create admin token
    let token = common::create_test_token(admin.id, true, "admin", &state.config.jwt_secret);

    // Create new user
    let response = server
        .post("/users/users")
        .add_cookie(Cookie::new("token", token))
        .json(&json!({
            "username": "newuser",
            "password": "NewPassword1!",
            "admin": false
        }))
        .await;

    response.assert_status_ok();
    let body: serde_json::Value = response.json();
    assert_eq!(body["username"], "newuser");
    assert_eq!(body["admin"], false);
}

#[tokio::test]
async fn create_user_as_non_admin_fails() {
    let (server, state) = setup_server().await;

    // Create a regular user
    let repo = UserRepository::new(&state.db);
    let user = repo
        .create("testuser", "Password123!", false)
        .await
        .expect("Failed to create user");

    // Create non-admin token
    let token = common::create_test_token(user.id, false, "testuser", &state.config.jwt_secret);

    // Try to create new user
    let response = server
        .post("/users/users")
        .add_cookie(Cookie::new("token", token))
        .json(&json!({
            "username": "newuser",
            "password": "NewPassword1!",
            "admin": false
        }))
        .await;

    response.assert_status_not_found();
}

// ============================================================================
// Get User Tests
// ============================================================================

#[tokio::test]
async fn get_own_user_succeeds() {
    let (server, state) = setup_server().await;

    // Create a user
    let repo = UserRepository::new(&state.db);
    let user = repo
        .create("testuser", "Password123!", false)
        .await
        .expect("Failed to create user");

    // Create token
    let token = common::create_test_token(user.id, false, "testuser", &state.config.jwt_secret);

    // Get user
    let response = server
        .get(&format!("/users/users/{}", user.id))
        .add_cookie(Cookie::new("token", token))
        .await;

    response.assert_status_ok();
    let body: serde_json::Value = response.json();
    assert_eq!(body["username"], "testuser");
}

#[tokio::test]
async fn get_other_user_as_non_admin_fails() {
    let (server, state) = setup_server().await;

    // Create two users
    let repo = UserRepository::new(&state.db);
    let user1 = repo
        .create("user1", "Password123!", false)
        .await
        .expect("Failed to create user1");
    let user2 = repo
        .create("user2", "Password123!", false)
        .await
        .expect("Failed to create user2");

    // Create token for user1
    let token = common::create_test_token(user1.id, false, "user1", &state.config.jwt_secret);

    // Try to get user2
    let response = server
        .get(&format!("/users/users/{}", user2.id))
        .add_cookie(Cookie::new("token", token))
        .await;

    response.assert_status_not_found();
}

#[tokio::test]
async fn get_other_user_as_admin_succeeds() {
    let (server, state) = setup_server().await;

    // Create admin and regular user
    let repo = UserRepository::new(&state.db);
    let admin = repo
        .create("admin", "Password123!", true)
        .await
        .expect("Failed to create admin");
    let user = repo
        .create("testuser", "Password123!", false)
        .await
        .expect("Failed to create user");

    // Create admin token
    let token = common::create_test_token(admin.id, true, "admin", &state.config.jwt_secret);

    // Get the regular user
    let response = server
        .get(&format!("/users/users/{}", user.id))
        .add_cookie(Cookie::new("token", token))
        .await;

    response.assert_status_ok();
    let body: serde_json::Value = response.json();
    assert_eq!(body["username"], "testuser");
}

// ============================================================================
// Update User Tests (Admin Only)
// ============================================================================

#[tokio::test]
async fn update_user_as_admin_succeeds() {
    let (server, state) = setup_server().await;

    // Create admin and regular user
    let repo = UserRepository::new(&state.db);
    let admin = repo
        .create("admin", "Password123!", true)
        .await
        .expect("Failed to create admin");
    let user = repo
        .create("testuser", "Password123!", false)
        .await
        .expect("Failed to create user");

    // Create admin token
    let token = common::create_test_token(admin.id, true, "admin", &state.config.jwt_secret);

    // Update the user
    let response = server
        .put(&format!("/users/users/{}", user.id))
        .add_cookie(Cookie::new("token", token))
        .json(&json!({
            "username": "updateduser",
            "admin": true
        }))
        .await;

    response.assert_status_ok();
    let body: serde_json::Value = response.json();
    assert_eq!(body["username"], "updateduser");
    assert_eq!(body["admin"], true);
}

// ============================================================================
// Update Password Tests
// ============================================================================

#[tokio::test]
async fn update_own_password_succeeds() {
    let (server, state) = setup_server().await;

    // Create a user
    let repo = UserRepository::new(&state.db);
    let user = repo
        .create("testuser", "OldPassword1!", false)
        .await
        .expect("Failed to create user");

    // Create token
    let token = common::create_test_token(user.id, false, "testuser", &state.config.jwt_secret);

    // Update password
    let response = server
        .post(&format!("/users/users/{}/password", user.id))
        .add_cookie(Cookie::new("token", token))
        .json(&json!({
            "password": "NewPassword1!"
        }))
        .await;

    response.assert_status_ok();

    // Verify new password works by authenticating
    let auth_response = server
        .post("/users/auth")
        .json(&json!({
            "username": "testuser",
            "password": "NewPassword1!"
        }))
        .await;

    auth_response.assert_status_ok();
}

// ============================================================================
// Delete User Tests (Admin Only)
// ============================================================================

#[tokio::test]
async fn delete_user_as_admin_succeeds() {
    let (server, state) = setup_server().await;

    // Create admin and regular user
    let repo = UserRepository::new(&state.db);
    let admin = repo
        .create("admin", "Password123!", true)
        .await
        .expect("Failed to create admin");
    let user = repo
        .create("testuser", "Password123!", false)
        .await
        .expect("Failed to create user");

    // Create admin token
    let token = common::create_test_token(admin.id, true, "admin", &state.config.jwt_secret);

    // Delete the user
    let response = server
        .delete(&format!("/users/users/{}", user.id))
        .add_cookie(Cookie::new("token", token.clone()))
        .await;

    response.assert_status_ok();

    // Verify user is deleted
    let get_response = server
        .get(&format!("/users/users/{}", user.id))
        .add_cookie(Cookie::new("token", token))
        .await;

    get_response.assert_status_not_found();
}

#[tokio::test]
async fn delete_user_as_non_admin_fails() {
    let (server, state) = setup_server().await;

    // Create two regular users
    let repo = UserRepository::new(&state.db);
    let user1 = repo
        .create("user1", "Password123!", false)
        .await
        .expect("Failed to create user1");
    let user2 = repo
        .create("user2", "Password123!", false)
        .await
        .expect("Failed to create user2");

    // Create token for user1
    let token = common::create_test_token(user1.id, false, "user1", &state.config.jwt_secret);

    // Try to delete user2
    let response = server
        .delete(&format!("/users/users/{}", user2.id))
        .add_cookie(Cookie::new("token", token))
        .await;

    response.assert_status_not_found();
}

// ============================================================================
// Password Validation Integration Tests
// ============================================================================

#[tokio::test]
async fn create_user_with_weak_password_fails() {
    let (server, state) = setup_server().await;

    // Create an admin user with a valid password
    let repo = UserRepository::new(&state.db);
    let admin = repo
        .create("admin", "AdminPass1!", true)
        .await
        .expect("Failed to create admin");

    let token = common::create_test_token(admin.id, true, "admin", &state.config.jwt_secret);

    // Try to create user with password too short
    let response = server
        .post("/users/users")
        .add_cookie(Cookie::new("token", token.clone()))
        .json(&json!({
            "username": "newuser",
            "password": "short",
            "admin": false
        }))
        .await;

    response.assert_status_not_ok();
}

#[tokio::test]
async fn create_user_with_strong_password_succeeds() {
    let (server, state) = setup_server().await;

    // Create an admin user
    let repo = UserRepository::new(&state.db);
    let admin = repo
        .create("admin", "AdminPass1!", true)
        .await
        .expect("Failed to create admin");

    let token = common::create_test_token(admin.id, true, "admin", &state.config.jwt_secret);

    // Create user with strong password
    let response = server
        .post("/users/users")
        .add_cookie(Cookie::new("token", token))
        .json(&json!({
            "username": "newuser",
            "password": "StrongPass1!",
            "admin": false
        }))
        .await;

    response.assert_status_ok();
    let body: serde_json::Value = response.json();
    assert_eq!(body["username"], "newuser");
}

#[tokio::test]
async fn update_password_with_weak_password_fails() {
    let (server, state) = setup_server().await;

    // Create a user
    let repo = UserRepository::new(&state.db);
    let user = repo
        .create("testuser", "OldPassword1!", false)
        .await
        .expect("Failed to create user");

    let token = common::create_test_token(user.id, false, "testuser", &state.config.jwt_secret);

    // Try to update with weak password
    let response = server
        .post(&format!("/users/users/{}/password", user.id))
        .add_cookie(Cookie::new("token", token))
        .json(&json!({
            "password": "weak"
        }))
        .await;

    response.assert_status_not_ok();
}

#[tokio::test]
async fn update_password_with_strong_password_succeeds() {
    let (server, state) = setup_server().await;

    // Create a user
    let repo = UserRepository::new(&state.db);
    let user = repo
        .create("testuser", "OldPassword1!", false)
        .await
        .expect("Failed to create user");

    let token = common::create_test_token(user.id, false, "testuser", &state.config.jwt_secret);

    // Update with strong password
    let response = server
        .post(&format!("/users/users/{}/password", user.id))
        .add_cookie(Cookie::new("token", token))
        .json(&json!({
            "password": "NewPassword1!"
        }))
        .await;

    response.assert_status_ok();

    // Verify new password works
    let auth_response = server
        .post("/users/auth")
        .json(&json!({
            "username": "testuser",
            "password": "NewPassword1!"
        }))
        .await;

    auth_response.assert_status_ok();
}

#[tokio::test]
async fn auth_after_password_change_with_old_password_fails() {
    let (server, state) = setup_server().await;

    // Create a user
    let repo = UserRepository::new(&state.db);
    let user = repo
        .create("testuser", "OldPassword1!", false)
        .await
        .expect("Failed to create user");

    let token = common::create_test_token(user.id, false, "testuser", &state.config.jwt_secret);

    // Update password
    server
        .post(&format!("/users/users/{}/password", user.id))
        .add_cookie(Cookie::new("token", token))
        .json(&json!({
            "password": "NewPassword1!"
        }))
        .await
        .assert_status_ok();

    // Try to authenticate with old password
    let response = server
        .post("/users/auth")
        .json(&json!({
            "username": "testuser",
            "password": "OldPassword1!"
        }))
        .await;

    response.assert_status_not_ok();
}

// ============================================================================
// Username Validation Integration Tests
// ============================================================================

#[tokio::test]
async fn create_user_with_short_username_fails() {
    let (server, state) = setup_server().await;

    // Create an admin user
    let repo = UserRepository::new(&state.db);
    let admin = repo
        .create("admin", "AdminPass1!", true)
        .await
        .expect("Failed to create admin");

    let token = common::create_test_token(admin.id, true, "admin", &state.config.jwt_secret);

    // Try to create user with username too short
    let response = server
        .post("/users/users")
        .add_cookie(Cookie::new("token", token))
        .json(&json!({
            "username": "ab",
            "password": "StrongPass1!",
            "admin": false
        }))
        .await;

    response.assert_status_not_ok();
}

#[tokio::test]
async fn create_user_with_duplicate_username_fails() {
    let (server, state) = setup_server().await;

    // Create an admin user
    let repo = UserRepository::new(&state.db);
    let admin = repo
        .create("admin", "AdminPass1!", true)
        .await
        .expect("Failed to create admin");

    let token = common::create_test_token(admin.id, true, "admin", &state.config.jwt_secret);

    // Create first user
    server
        .post("/users/users")
        .add_cookie(Cookie::new("token", token.clone()))
        .json(&json!({
            "username": "testuser",
            "password": "StrongPass1!",
            "admin": false
        }))
        .await
        .assert_status_ok();

    // Try to create second user with same username
    let response = server
        .post("/users/users")
        .add_cookie(Cookie::new("token", token))
        .json(&json!({
            "username": "testuser",
            "password": "AnotherPass1!",
            "admin": false
        }))
        .await;

    response.assert_status_not_ok();
}

#[tokio::test]
async fn create_user_with_valid_username_succeeds() {
    let (server, state) = setup_server().await;

    // Create an admin user
    let repo = UserRepository::new(&state.db);
    let admin = repo
        .create("admin", "AdminPass1!", true)
        .await
        .expect("Failed to create admin");

    let token = common::create_test_token(admin.id, true, "admin", &state.config.jwt_secret);

    // Create user with valid 3-char username
    let response = server
        .post("/users/users")
        .add_cookie(Cookie::new("token", token))
        .json(&json!({
            "username": "abc",
            "password": "StrongPass1!",
            "admin": false
        }))
        .await;

    response.assert_status_ok();
    let body: serde_json::Value = response.json();
    assert_eq!(body["username"], "abc");
}
