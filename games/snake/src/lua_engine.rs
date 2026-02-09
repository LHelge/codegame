use bevy::prelude::*;
use mlua::prelude::*;
use std::cell::RefCell;

use crate::direction::Direction;
use crate::game::{GRID_SIZE, SnakeGame};

// ---------------------------------------------------------------------------
// Thread-local for exposing script errors to JS (WASM)
// ---------------------------------------------------------------------------

std::thread_local! {
    /// The most recent Lua script error, if any. Cleared when a new script loads successfully.
    pub static LAST_SCRIPT_ERROR: RefCell<Option<String>> = const { RefCell::new(None) };
}

/// Set the last script error (called internally).
fn set_last_error(msg: String) {
    LAST_SCRIPT_ERROR.with(|cell| {
        *cell.borrow_mut() = Some(msg);
    });
}

/// Clear the last script error.
fn clear_last_error() {
    LAST_SCRIPT_ERROR.with(|cell| {
        *cell.borrow_mut() = None;
    });
}

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
            let msg = format!("Syntax error: {e}");
            warn!("Lua load error: {e}");
            set_last_error(msg);
            self.script_loaded = false;
            return false;
        }
        // Verify a `think` function exists.
        match self.lua.globals().get::<mlua::Function>("think") {
            Ok(_) => {
                clear_last_error();
                self.script_loaded = true;
                true
            }
            Err(e) => {
                let msg = "Script must define a `think(state)` function".to_string();
                warn!("Lua script has no `think` function: {e}");
                set_last_error(msg);
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

        let result: Result<String, _> = think.call(state);
        match result {
            Ok(dir_str) => {
                if let Some(dir) = Direction::from_str(&dir_str) {
                    Some(dir)
                } else {
                    let msg = format!(
                        "think() returned '{}', expected 'north', 'south', 'east', or 'west'",
                        dir_str
                    );
                    warn!("{}", msg);
                    set_last_error(msg);
                    None
                }
            }
            Err(e) => {
                let msg = format!("Runtime error in think(): {e}");
                warn!("{}", msg);
                set_last_error(msg);
                None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::VecDeque;

    fn make_game() -> SnakeGame {
        SnakeGame {
            snake: VecDeque::from([(5, 5), (4, 5), (3, 5)]),
            direction: Direction::East,
            food: (10, 10),
            score: 0,
            game_over: false,
            game_won: false,
            game_over_timer: 0.0,
        }
    }

    #[test]
    fn new_engine_has_no_script() {
        let engine = LuaEngine::new();
        assert!(!engine.script_loaded);
    }

    #[test]
    fn load_valid_script() {
        let mut engine = LuaEngine::new();
        let ok = engine.load_script("function think(state) return 'north' end");
        assert!(ok);
        assert!(engine.script_loaded);
    }

    #[test]
    fn load_script_without_think_fails() {
        let mut engine = LuaEngine::new();
        let ok = engine.load_script("function helper() return 1 end");
        assert!(!ok);
        assert!(!engine.script_loaded);
    }

    #[test]
    fn load_syntax_error_fails() {
        let mut engine = LuaEngine::new();
        let ok = engine.load_script("this is not valid lua %%!!");
        assert!(!ok);
    }

    #[test]
    fn call_think_returns_direction() {
        let mut engine = LuaEngine::new();
        engine.load_script("function think(state) return 'north' end");
        let game = make_game();
        assert_eq!(engine.call_think(&game), Some(Direction::North));
    }

    #[test]
    fn call_think_returns_none_when_no_script() {
        let engine = LuaEngine::new();
        let game = make_game();
        assert_eq!(engine.call_think(&game), None);
    }

    #[test]
    fn call_think_returns_none_for_invalid_direction() {
        let mut engine = LuaEngine::new();
        engine.load_script("function think(state) return 'sideways' end");
        let game = make_game();
        assert_eq!(engine.call_think(&game), None);
    }

    #[test]
    fn think_receives_game_state() {
        let mut engine = LuaEngine::new();
        // Script that reads state and returns direction based on food position.
        let script = r#"
            function think(state)
                if state.food.x > state.snake[1].x then
                    return "east"
                else
                    return "west"
                end
            end
        "#;
        engine.load_script(script);
        let game = make_game(); // food at (10,10), head at (5,5)
        assert_eq!(engine.call_think(&game), Some(Direction::East));
    }

    #[test]
    fn think_receives_grid_size() {
        let mut engine = LuaEngine::new();
        let script = r#"
            function think(state)
                if state.grid_size == 32 then
                    return "north"
                end
                return "south"
            end
        "#;
        engine.load_script(script);
        let game = make_game();
        assert_eq!(engine.call_think(&game), Some(Direction::North));
    }

    #[test]
    fn replacing_script_works() {
        let mut engine = LuaEngine::new();
        engine.load_script("function think(state) return 'north' end");
        let game = make_game();
        assert_eq!(engine.call_think(&game), Some(Direction::North));

        engine.load_script("function think(state) return 'south' end");
        assert_eq!(engine.call_think(&game), Some(Direction::South));
    }

    #[test]
    fn syntax_error_sets_last_error() {
        clear_last_error();
        let mut engine = LuaEngine::new();
        engine.load_script("this is not valid lua %%!!");
        let err = LAST_SCRIPT_ERROR.with(|cell| cell.borrow().clone());
        assert!(err.is_some());
        assert!(err.unwrap().contains("Syntax error"));
    }

    #[test]
    fn missing_think_sets_last_error() {
        clear_last_error();
        let mut engine = LuaEngine::new();
        engine.load_script("function helper() return 1 end");
        let err = LAST_SCRIPT_ERROR.with(|cell| cell.borrow().clone());
        assert!(err.is_some());
        assert!(err.unwrap().contains("think(state)"));
    }

    #[test]
    fn valid_script_clears_last_error() {
        // First cause an error
        let mut engine = LuaEngine::new();
        engine.load_script("invalid syntax !!");
        assert!(LAST_SCRIPT_ERROR.with(|cell| cell.borrow().is_some()));

        // Now load a valid script
        engine.load_script("function think(state) return 'north' end");
        assert!(LAST_SCRIPT_ERROR.with(|cell| cell.borrow().is_none()));
    }

    #[test]
    fn invalid_direction_sets_last_error() {
        clear_last_error();
        let mut engine = LuaEngine::new();
        engine.load_script("function think(state) return 'sideways' end");
        let game = make_game();
        engine.call_think(&game);
        let err = LAST_SCRIPT_ERROR.with(|cell| cell.borrow().clone());
        assert!(err.is_some());
        assert!(err.unwrap().contains("sideways"));
    }

    #[test]
    fn runtime_error_sets_last_error() {
        clear_last_error();
        let mut engine = LuaEngine::new();
        // Script that causes a runtime error by accessing nil
        engine.load_script("function think(state) return state.nonexistent.value end");
        let game = make_game();
        engine.call_think(&game);
        let err = LAST_SCRIPT_ERROR.with(|cell| cell.borrow().clone());
        assert!(err.is_some());
        assert!(err.unwrap().contains("Runtime error"));
    }
}
