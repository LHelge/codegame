//! Integration tests for hello endpoints.

mod common;

use axum_test::TestServer;
use backend::prelude::AppState;
use backend::routes;

#[tokio::test]
async fn hello_returns_hello_world() {
    let config = common::test_config();
    let db = common::test_db().await;
    let state = AppState::new(config, db);
    let app = routes::routes().with_state(state);
    let server = TestServer::new(app).unwrap();

    let response = server.get("/hello").await;

    response.assert_status_ok();
    response.assert_text("hello world");
}
