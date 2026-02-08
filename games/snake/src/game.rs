use bevy::prelude::*;
use std::collections::VecDeque;

use crate::direction::Direction;

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

pub const GRID_SIZE: i32 = 32;
pub const CELL_SIZE: f32 = 64.0;
pub const TICK_SECS: f32 = 0.15;
pub const INITIAL_LENGTH: usize = 3;
pub const GAME_OVER_DELAY: f32 = 3.0;
pub const SPRITE_FOOD: usize = 14;

// ---------------------------------------------------------------------------
// Game state resource
// ---------------------------------------------------------------------------

#[derive(Resource)]
pub struct SnakeGame {
    pub snake: VecDeque<(i32, i32)>,
    pub direction: Direction,
    pub food: (i32, i32),
    pub score: u32,
    pub game_over: bool,
    pub game_over_timer: f32,
}

impl Default for SnakeGame {
    fn default() -> Self {
        let mid = GRID_SIZE / 2;
        let mut snake = VecDeque::new();
        for i in 0..INITIAL_LENGTH as i32 {
            // Head at (mid, mid), body extends west
            snake.push_back((mid - i, mid));
        }
        let food = random_food_position(&snake);
        Self {
            snake,
            direction: Direction::East,
            food,
            score: 0,
            game_over: false,
            game_over_timer: 0.0,
        }
    }
}

#[derive(Resource)]
pub struct GameTimer {
    pub timer: Timer,
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

pub fn grid_to_world(gx: i32, gy: i32) -> Vec3 {
    let half = (GRID_SIZE as f32 - 1.0) / 2.0;
    Vec3::new(
        (gx as f32 - half) * CELL_SIZE,
        (gy as f32 - half) * CELL_SIZE,
        0.0,
    )
}

pub fn random_food_position(snake: &VecDeque<(i32, i32)>) -> (i32, i32) {
    loop {
        let mut buf = [0u8; 8];
        getrandom::fill(&mut buf).unwrap();
        let x = (u32::from_le_bytes([buf[0], buf[1], buf[2], buf[3]]) % GRID_SIZE as u32) as i32;
        let y = (u32::from_le_bytes([buf[4], buf[5], buf[6], buf[7]]) % GRID_SIZE as u32) as i32;
        if !snake.contains(&(x, y)) {
            return (x, y);
        }
    }
}

pub fn reset_game_state(game: &mut SnakeGame) {
    let fresh = SnakeGame::default();
    game.snake = fresh.snake;
    game.direction = fresh.direction;
    game.food = fresh.food;
    game.score = 0;
    game.game_over = false;
    game.game_over_timer = 0.0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_game_has_correct_snake_length() {
        let game = SnakeGame::default();
        assert_eq!(game.snake.len(), INITIAL_LENGTH);
    }

    #[test]
    fn default_game_snake_starts_at_center() {
        let game = SnakeGame::default();
        let mid = GRID_SIZE / 2;
        assert_eq!(*game.snake.front().unwrap(), (mid, mid));
    }

    #[test]
    fn default_game_starts_heading_east() {
        let game = SnakeGame::default();
        assert_eq!(game.direction, Direction::East);
    }

    #[test]
    fn default_game_is_not_over() {
        let game = SnakeGame::default();
        assert!(!game.game_over);
        assert_eq!(game.score, 0);
    }

    #[test]
    fn food_not_on_snake() {
        let game = SnakeGame::default();
        assert!(!game.snake.contains(&game.food));
    }

    #[test]
    fn random_food_avoids_snake() {
        let game = SnakeGame::default();
        for _ in 0..50 {
            let pos = random_food_position(&game.snake);
            assert!(!game.snake.contains(&pos));
            assert!(pos.0 >= 0 && pos.0 < GRID_SIZE);
            assert!(pos.1 >= 0 && pos.1 < GRID_SIZE);
        }
    }

    #[test]
    fn grid_to_world_center() {
        // The center cell of an even grid won't be exactly zero, but
        // the middle-ish cell should be near the origin.
        let mid = GRID_SIZE / 2;
        let pos = grid_to_world(mid, mid);
        // With GRID_SIZE=32, half=(31/2)=15.5, so cell 16 → (0.5*64)=32.0
        let half = (GRID_SIZE as f32 - 1.0) / 2.0;
        let expected_x = (mid as f32 - half) * CELL_SIZE;
        assert!((pos.x - expected_x).abs() < f32::EPSILON);
    }

    #[test]
    fn grid_to_world_origin_cell() {
        let pos = grid_to_world(0, 0);
        let half = (GRID_SIZE as f32 - 1.0) / 2.0;
        assert!((pos.x - (-half * CELL_SIZE)).abs() < f32::EPSILON);
        assert!((pos.y - (-half * CELL_SIZE)).abs() < f32::EPSILON);
        assert!((pos.z).abs() < f32::EPSILON);
    }

    #[test]
    fn reset_clears_game_over() {
        let mut game = SnakeGame {
            game_over: true,
            score: 42,
            game_over_timer: 2.0,
            ..SnakeGame::default()
        };
        reset_game_state(&mut game);
        assert!(!game.game_over);
        assert_eq!(game.score, 0);
        assert_eq!(game.game_over_timer, 0.0);
        assert_eq!(game.snake.len(), INITIAL_LENGTH);
    }
}
