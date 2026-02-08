mod direction;
mod game;
mod lua_engine;
mod rendering;
mod systems;
mod wasm_bridge;

use bevy::prelude::*;
use wasm_bindgen::prelude::*;

use lua_engine::LuaEngine;
use rendering::GameEntities;

// Re-export the WASM bridge functions so wasm-bindgen can find them.
pub use wasm_bridge::{request_reset, set_agent_code};

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

#[wasm_bindgen(start)]
pub fn run() {
    build_app(None).run();
}

/// Run with an initial Lua script (used by native main.rs).
pub fn run_with_script(code: &str) {
    build_app(Some(code.to_string())).run();
}

fn build_app(initial_script: Option<String>) -> App {
    let mut app = App::new();

    // Lua engine must be inserted as non-send (mlua::Lua is !Send).
    let mut lua_engine = LuaEngine::new();
    if let Some(code) = &initial_script {
        lua_engine.load_script(code);
    }
    app.insert_non_send_resource(lua_engine);

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Snake".into(),
            canvas: Some("#snake-canvas".into()),
            fit_canvas_to_parent: true,
            prevent_default_event_handling: true,
            ..default()
        }),
        ..default()
    }))
    .insert_resource(ClearColor(Color::srgb(0.1, 0.1, 0.12)))
    .insert_resource(GameEntities::default())
    .add_systems(Startup, rendering::setup)
    .add_systems(
        Update,
        (
            systems::receive_pending_code,
            systems::game_tick,
            systems::game_over_countdown,
            rendering::render_game,
            rendering::update_score_text,
        )
            .chain(),
    );

    app
}
