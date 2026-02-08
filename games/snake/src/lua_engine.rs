use bevy::prelude::*;
use mlua::prelude::*;

use crate::direction::Direction;
use crate::game::{GRID_SIZE, SnakeGame};

/// Lua scripting engine (non-Send because `mlua::Lua` is `!Send`).
pub struct LuaEngine {
    lua: Lua,
    script_loaded: bool,
}

impl LuaEngine {
    pub fn new() -> Self {
        Self {
            lua: Lua::new(),
            script_loaded: false,
        }
    }

    pub fn load_script(&mut self, code: &str) -> bool {
        if let Err(e) = self.lua.load(code).exec() {
            warn!("Lua load error: {e}");
            self.script_loaded = false;
            return false;
        }
        // Verify a `think` function exists.
        match self.lua.globals().get::<mlua::Function>("think") {
            Ok(_) => {
                self.script_loaded = true;
                true
            }
            Err(e) => {
                warn!("Lua script has no `think` function: {e}");
                self.script_loaded = false;
                false
            }
        }
    }

    pub fn call_think(&self, game: &SnakeGame) -> Option<Direction> {
        if !self.script_loaded {
            return None;
        }
        let lua = &self.lua;
        let think: mlua::Function = lua.globals().get("think").ok()?;

        let state = lua.create_table().ok()?;

        // snake array (1-indexed)
        let snake_table = lua.create_table().ok()?;
        for (i, &(x, y)) in game.snake.iter().enumerate() {
            let pos = lua.create_table().ok()?;
            pos.set("x", x).ok()?;
            pos.set("y", y).ok()?;
            snake_table.set(i + 1, pos).ok()?;
        }
        state.set("snake", snake_table).ok()?;

        let food = lua.create_table().ok()?;
        food.set("x", game.food.0).ok()?;
        food.set("y", game.food.1).ok()?;
        state.set("food", food).ok()?;

        state.set("direction", game.direction.as_str()).ok()?;
        state.set("grid_size", GRID_SIZE).ok()?;
        state.set("score", game.score).ok()?;

        let result: String = think.call(state).ok()?;
        Direction::from_str(&result)
    }
}
