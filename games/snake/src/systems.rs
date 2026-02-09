use bevy::prelude::*;

use crate::game::{
    GAME_OVER_DELAY, GRID_SIZE, GameTimer, SnakeGame, random_food_position, reset_game_state,
};
use crate::lua_engine::LuaEngine;
use crate::wasm_bridge::{PENDING_LUA_CODE, PENDING_RESET};

/// Poll the thread-local for new Lua code from JS (WASM) or reset requests.
pub fn receive_pending_code(mut lua_engine: NonSendMut<LuaEngine>, mut game: ResMut<SnakeGame>) {
    PENDING_LUA_CODE.with(|cell| {
        if let Some(code) = cell.borrow_mut().take() {
            lua_engine.load_script(&code);
        }
    });
    PENDING_RESET.with(|cell| {
        let mut flag = cell.borrow_mut();
        if *flag {
            *flag = false;
            reset_game_state(&mut game);
        }
    });
}

/// Advance the game by one tick when the timer fires.
pub fn game_tick(
    time: Res<Time>,
    mut timer: ResMut<GameTimer>,
    mut game: ResMut<SnakeGame>,
    lua_engine: NonSend<LuaEngine>,
) {
    timer.timer.tick(time.delta());
    if !timer.timer.just_finished() || game.game_over {
        return;
    }

    // Ask the Lua AI for a direction
    if let Some(new_dir) = lua_engine.call_think(&game)
        && new_dir != game.direction.opposite()
    {
        game.direction = new_dir;
    }

    // New head position
    let (dx, dy) = game.direction.delta();
    let head = *game.snake.front().unwrap();
    let new_head = (head.0 + dx, head.1 + dy);

    // Wall collision
    if new_head.0 < 0 || new_head.0 >= GRID_SIZE || new_head.1 < 0 || new_head.1 >= GRID_SIZE {
        game.game_over = true;
        game.game_over_timer = GAME_OVER_DELAY;
        return;
    }

    // Determine if the snake will grow (eating food)
    let will_grow = new_head == game.food;

    // Self collision: when not growing, exclude the tail since it will move out of the way
    let self_collision = if will_grow {
        game.snake.contains(&new_head)
    } else {
        game.snake
            .iter()
            .take(game.snake.len() - 1)
            .any(|&seg| seg == new_head)
    };

    if self_collision {
        game.game_over = true;
        game.game_over_timer = GAME_OVER_DELAY;
        return;
    }

    game.snake.push_front(new_head);

    if will_grow {
        game.score += 1;
        if let Some(new_food) = random_food_position(&game.snake) {
            game.food = new_food;
        } else {
            // Snake filled the entire grid — player wins!
            game.game_won = true;
            game.game_over = true;
            game.game_over_timer = GAME_OVER_DELAY;
        }
    } else {
        game.snake.pop_back();
    }
}

/// Count down the game-over delay, then reset.
pub fn game_over_countdown(time: Res<Time>, mut game: ResMut<SnakeGame>) {
    if !game.game_over {
        return;
    }
    game.game_over_timer -= time.delta_secs();
    if game.game_over_timer <= 0.0 {
        reset_game_state(&mut game);
    }
}
