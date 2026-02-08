use bevy::asset::RenderAssetUsages;
use bevy::camera::ScalingMode;
use bevy::image::{CompressedImageFormats, ImageSampler, ImageType};
use bevy::prelude::*;

use crate::direction::{Direction, direction_between};
use crate::game::{CELL_SIZE, GRID_SIZE, SPRITE_FOOD, SnakeGame};
use crate::game::{GameTimer, TICK_SECS, grid_to_world};

// ---------------------------------------------------------------------------
// Sprite indices in the 15-frame horizontal sprite sheet
// ---------------------------------------------------------------------------

const SPRITE_BODY_H: usize = 0;
const SPRITE_BODY_V: usize = 1;
const SPRITE_TURN_SE: usize = 2;
const SPRITE_TURN_SW: usize = 3;
const SPRITE_TURN_NW: usize = 4;
const SPRITE_TURN_NE: usize = 5;
const SPRITE_TAIL_W: usize = 6;
const SPRITE_TAIL_N: usize = 7;
const SPRITE_TAIL_E: usize = 8;
const SPRITE_TAIL_S: usize = 9;
const SPRITE_HEAD_W: usize = 10;
const SPRITE_HEAD_N: usize = 11;
const SPRITE_HEAD_E: usize = 12;
const SPRITE_HEAD_S: usize = 13;

// ---------------------------------------------------------------------------
// Resources & components
// ---------------------------------------------------------------------------

#[derive(Resource)]
pub struct SpriteAssets {
    pub texture: Handle<Image>,
    pub layout: Handle<TextureAtlasLayout>,
}

#[derive(Resource, Default)]
pub struct GameEntities {
    pub segments: Vec<Entity>,
    pub food: Option<Entity>,
    pub game_over_ui: Option<Entity>,
}

#[derive(Component)]
pub struct ScoreText;

// ---------------------------------------------------------------------------
// Setup system
// ---------------------------------------------------------------------------

pub fn setup(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // Camera — orthographic, sized to fit the grid
    let total = GRID_SIZE as f32 * CELL_SIZE;
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::AutoMin {
                min_width: total,
                min_height: total,
            },
            ..OrthographicProjection::default_2d()
        }),
    ));

    // Decode embedded sprite sheet
    let sprite_bytes = include_bytes!("../assets/sprites.png");
    let mut image = Image::from_buffer(
        sprite_bytes,
        ImageType::Extension("png"),
        CompressedImageFormats::NONE,
        true,
        ImageSampler::nearest(),
        RenderAssetUsages::RENDER_WORLD,
    )
    .expect("Failed to decode sprites.png");
    image.sampler = ImageSampler::nearest();
    let texture = images.add(image);

    let layout = TextureAtlasLayout::from_grid(UVec2::splat(64), 15, 1, None, None);
    let layout_handle = layouts.add(layout);

    commands.insert_resource(SpriteAssets {
        texture,
        layout: layout_handle,
    });

    // Game state
    commands.insert_resource(SnakeGame::default());
    commands.insert_resource(GameTimer {
        timer: Timer::from_seconds(TICK_SECS, TimerMode::Repeating),
    });

    // Score UI
    commands
        .spawn(Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        })
        .with_child((
            Text::new("Score: 0"),
            TextFont {
                font_size: 32.0,
                ..default()
            },
            TextColor(Color::WHITE),
            ScoreText,
        ));
}

// ---------------------------------------------------------------------------
// Rendering systems
// ---------------------------------------------------------------------------

/// Synchronise Bevy entities with the current `SnakeGame` state.
pub fn render_game(
    mut commands: Commands,
    game: Res<SnakeGame>,
    mut entities: ResMut<GameEntities>,
    sprites: Res<SpriteAssets>,
    mut query: Query<(&mut Transform, &mut Sprite)>,
) {
    let snake_len = game.snake.len();

    // --- Snake segment entities ---
    // Grow
    while entities.segments.len() < snake_len {
        let entity = commands
            .spawn((
                Sprite {
                    image: sprites.texture.clone(),
                    texture_atlas: Some(TextureAtlas {
                        layout: sprites.layout.clone(),
                        index: 0,
                    }),
                    ..default()
                },
                Transform::from_xyz(0.0, 0.0, 0.0),
            ))
            .id();
        entities.segments.push(entity);
    }
    // Shrink
    while entities.segments.len() > snake_len {
        if let Some(entity) = entities.segments.pop() {
            commands.entity(entity).despawn();
        }
    }

    // Update positions and sprite indices
    for (i, &(gx, gy)) in game.snake.iter().enumerate() {
        let entity = entities.segments[i];
        let idx = sprite_index_for_segment(&game, i);
        if let Ok((mut transform, mut sprite)) = query.get_mut(entity) {
            transform.translation = grid_to_world(gx, gy);
            if let Some(ref mut atlas) = sprite.texture_atlas {
                atlas.index = idx;
            }
        }
    }

    // --- Food entity ---
    match entities.food {
        Some(food_entity) => {
            if let Ok((mut transform, _)) = query.get_mut(food_entity) {
                transform.translation = grid_to_world(game.food.0, game.food.1);
                transform.translation.z = 1.0;
            }
        }
        None => {
            let entity = commands
                .spawn((
                    Sprite {
                        image: sprites.texture.clone(),
                        texture_atlas: Some(TextureAtlas {
                            layout: sprites.layout.clone(),
                            index: SPRITE_FOOD,
                        }),
                        ..default()
                    },
                    Transform::from_translation(grid_to_world(game.food.0, game.food.1))
                        .with_scale(Vec3::splat(1.0)),
                ))
                .id();
            entities.food = Some(entity);
        }
    }

    // --- Game over overlay ---
    if game.game_over && entities.game_over_ui.is_none() {
        let entity = commands
            .spawn(Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            })
            .with_child((
                Text::new(format!("Game Over!  Score: {}", game.score)),
                TextFont {
                    font_size: 64.0,
                    ..default()
                },
                TextColor(Color::srgba(1.0, 0.3, 0.3, 1.0)),
            ))
            .id();
        entities.game_over_ui = Some(entity);
    } else if !game.game_over
        && let Some(entity) = entities.game_over_ui.take()
    {
        commands.entity(entity).despawn();
    }
}

/// Keep the score text in sync.
pub fn update_score_text(game: Res<SnakeGame>, mut query: Query<&mut Text, With<ScoreText>>) {
    for mut text in &mut query {
        **text = format!("Score: {}", game.score);
    }
}

// ---------------------------------------------------------------------------
// Sprite helpers
// ---------------------------------------------------------------------------

/// Pick the correct sprite atlas index for the snake segment at `index`.
fn sprite_index_for_segment(game: &SnakeGame, index: usize) -> usize {
    let len = game.snake.len();
    if len < 2 {
        return SPRITE_HEAD_E;
    }

    let cur = game.snake[index];

    if index == 0 {
        let body = game.snake[1];
        let to_body = direction_between(cur, body);
        return match to_body {
            Direction::West => SPRITE_HEAD_W,
            Direction::North => SPRITE_HEAD_N,
            Direction::East => SPRITE_HEAD_E,
            Direction::South => SPRITE_HEAD_S,
        };
    }

    if index == len - 1 {
        let body = game.snake[index - 1];
        let to_body = direction_between(cur, body);
        return match to_body {
            Direction::West => SPRITE_TAIL_W,
            Direction::North => SPRITE_TAIL_N,
            Direction::East => SPRITE_TAIL_E,
            Direction::South => SPRITE_TAIL_S,
        };
    }

    // Body — determine connection directions
    let prev = game.snake[index - 1]; // toward head
    let next = game.snake[index + 1]; // toward tail
    let to_prev = direction_between(cur, prev);
    let to_next = direction_between(cur, next);

    match (to_prev, to_next) {
        (Direction::East, Direction::West) | (Direction::West, Direction::East) => SPRITE_BODY_H,
        (Direction::North, Direction::South) | (Direction::South, Direction::North) => {
            SPRITE_BODY_V
        }
        (Direction::South, Direction::East) | (Direction::East, Direction::South) => SPRITE_TURN_SE,
        (Direction::South, Direction::West) | (Direction::West, Direction::South) => SPRITE_TURN_SW,
        (Direction::North, Direction::West) | (Direction::West, Direction::North) => SPRITE_TURN_NW,
        (Direction::North, Direction::East) | (Direction::East, Direction::North) => SPRITE_TURN_NE,
        _ => SPRITE_BODY_H, // fallback
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::VecDeque;

    /// Build a minimal game with the given snake segments.
    fn game_with_snake(segments: &[(i32, i32)]) -> SnakeGame {
        SnakeGame {
            snake: VecDeque::from(segments.to_vec()),
            direction: Direction::East,
            food: (0, 0),
            score: 0,
            game_over: false,
            game_over_timer: 0.0,
        }
    }

    #[test]
    fn head_facing_east() {
        // Head at (5,5), body at (4,5) → head looks east (body is to the west)
        let game = game_with_snake(&[(5, 5), (4, 5), (3, 5)]);
        // The head sprite is chosen by where the *body* is relative to the head.
        // Body is west of head → head sprite faces east (away from body).
        // Actually the code maps to_body direction directly to sprite:
        // to_body=West → SPRITE_HEAD_W?  Let's check the mapping.
        // In the code: to_body = direction_between(cur, body) = direction from (5,5) to (4,5) = West
        // match to_body West => SPRITE_HEAD_W
        // Wait, that means the head sprite index indicates WHERE THE BODY IS,
        // which is the direction the head is facing INTO (i.e., the tail side).
        // The naming convention on the sprite sheet determines this.
        assert_eq!(sprite_index_for_segment(&game, 0), SPRITE_HEAD_W);
    }

    #[test]
    fn head_facing_north() {
        // Head at (5,6), body at (5,5) → body is south of head
        let game = game_with_snake(&[(5, 6), (5, 5), (5, 4)]);
        assert_eq!(sprite_index_for_segment(&game, 0), SPRITE_HEAD_S);
    }

    #[test]
    fn tail_sprite() {
        // Snake: (5,5) (4,5) (3,5) — tail at (3,5), body at (4,5)
        let game = game_with_snake(&[(5, 5), (4, 5), (3, 5)]);
        let tail_idx = game.snake.len() - 1;
        // direction_between(tail=(3,5), body=(4,5)) = East
        assert_eq!(sprite_index_for_segment(&game, tail_idx), SPRITE_TAIL_E);
    }

    #[test]
    fn horizontal_body_segment() {
        // Straight horizontal snake: (6,5) (5,5) (4,5) (3,5)
        let game = game_with_snake(&[(6, 5), (5, 5), (4, 5), (3, 5)]);
        // Middle segment at index 1: (5,5), prev=(6,5) next=(4,5)
        // to_prev = East, to_next = West → SPRITE_BODY_H
        assert_eq!(sprite_index_for_segment(&game, 1), SPRITE_BODY_H);
    }

    #[test]
    fn vertical_body_segment() {
        // Straight vertical snake: (5,7) (5,6) (5,5) (5,4)
        let game = game_with_snake(&[(5, 7), (5, 6), (5, 5), (5, 4)]);
        // Middle segment at index 1: (5,6), prev=(5,7) next=(5,5)
        // to_prev = North, to_next = South → SPRITE_BODY_V
        assert_eq!(sprite_index_for_segment(&game, 1), SPRITE_BODY_V);
    }

    #[test]
    fn turn_segment() {
        // Snake turns: head going east then turns north
        // (5,6) (5,5) (4,5) (3,5)
        let game = game_with_snake(&[(5, 6), (5, 5), (4, 5), (3, 5)]);
        // Turn segment at index 1: (5,5), prev=(5,6) next=(4,5)
        // to_prev = North, to_next = West → SPRITE_TURN_NW
        assert_eq!(sprite_index_for_segment(&game, 1), SPRITE_TURN_NW);
    }

    #[test]
    fn single_segment_snake() {
        let game = game_with_snake(&[(5, 5)]);
        assert_eq!(sprite_index_for_segment(&game, 0), SPRITE_HEAD_E);
    }
}
