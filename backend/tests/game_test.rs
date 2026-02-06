mod common;

use axum_test::TestServer;
use backend::models::Game;
use backend::prelude::AppState;
use backend::routes;

#[tokio::test]
async fn list_games_returns_seeded_games() {
    let config = common::test_config();
    let db = common::test_db().await;
    let state = AppState::new(config, db);
    let app = routes::routes().with_state(state);
    let server = TestServer::new(app).unwrap();

    let response = server.get("/games").await;
    response.assert_status_ok();

    let games: Vec<Game> = response.json();
    assert_eq!(games.len(), 2);
    assert!(games.iter().any(|g| g.name == "robotsumo"));
    assert!(games.iter().any(|g| g.name == "snake"));
}

#[tokio::test]
async fn get_game_by_id_returns_game() {
    let config = common::test_config();
    let db = common::test_db().await;
    let state = AppState::new(config, db);
    let app = routes::routes().with_state(state);
    let server = TestServer::new(app).unwrap();

    let response = server.get("/games/1").await;
    response.assert_status_ok();

    let game: Game = response.json();
    assert_eq!(game.id, 1);
}

#[tokio::test]
async fn get_game_by_invalid_id_returns_not_found() {
    let config = common::test_config();
    let db = common::test_db().await;
    let state = AppState::new(config, db);
    let app = routes::routes().with_state(state);
    let server = TestServer::new(app).unwrap();

    let response = server.get("/games/999").await;
    response.assert_status_not_found();
}

#[tokio::test]
async fn get_game_by_wasm_filename_returns_game() {
    let config = common::test_config();
    let db = common::test_db().await;
    let state = AppState::new(config, db);
    let app = routes::routes().with_state(state);
    let server = TestServer::new(app).unwrap();

    let response = server.get("/games/by-wasm/robotsumo").await;
    response.assert_status_ok();

    let game: Game = response.json();
    assert_eq!(game.wasm_filename, "robotsumo");
}

#[tokio::test]
async fn get_game_by_invalid_wasm_filename_returns_not_found() {
    let config = common::test_config();
    let db = common::test_db().await;
    let state = AppState::new(config, db);
    let app = routes::routes().with_state(state);
    let server = TestServer::new(app).unwrap();

    let response = server.get("/games/by-wasm/nonexistent").await;
    response.assert_status_not_found();
}
