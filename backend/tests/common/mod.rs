//! Common test utilities for integration tests.

use backend::prelude::{Claims, Config};
use chrono::Duration;
use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};

/// Create a test configuration with in-memory database.
pub fn test_config() -> Config {
    Config {
        database_url: ":memory:".to_string(),
        server_port: 0,
        jwt_secret: "test-secret-key-for-testing-only".to_string(),
    }
}

/// Create a test database pool with migrations applied.
pub async fn test_db() -> SqlitePool {
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect(":memory:")
        .await
        .expect("Failed to create test database");

    // Run migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    pool
}

/// Helper to create JWT token for authenticated requests.
#[allow(dead_code)]
pub fn create_test_token(user_id: i64, admin: bool, username: &str, secret: &str) -> String {
    Claims::new(user_id, admin, username, Duration::hours(1))
        .encode(secret)
        .expect("Failed to create test token")
}
