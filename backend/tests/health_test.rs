//! Integration tests for health endpoints.

mod common;

use axum_test::TestServer;
use backend::prelude::AppState;
use backend::routes;

#[tokio::test]
async fn health_returns_ok() {
    let config = common::test_config();
    let db = common::test_db().await;
    let state = AppState::new(config, db);
    let app = routes::routes().with_state(state);
    let server = TestServer::new(app).unwrap();

    let response = server.get("/health").await;

    response.assert_status_ok();
    response.assert_json(&serde_json::json!({
        "status": "ok"
    }));
}
