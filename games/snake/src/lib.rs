use bevy::asset::RenderAssetUsages;
use bevy::camera::ScalingMode;
use bevy::image::{CompressedImageFormats, ImageSampler, ImageType};
use bevy::prelude::*;
use mlua::prelude::*;
use std::collections::VecDeque;
use wasm_bindgen::prelude::*;

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

const GRID_SIZE: i32 = 32;
const CELL_SIZE: f32 = 64.0;
const TICK_SECS: f32 = 0.15;
const INITIAL_LENGTH: usize = 3;
const GAME_OVER_DELAY: f32 = 3.0;

// Sprite indices in the 15-frame horizontal sprite sheet
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
const SPRITE_FOOD: usize = 14;

// ---------------------------------------------------------------------------
// WASM bridge — thread-local for receiving Lua code from JavaScript
// ---------------------------------------------------------------------------

std::thread_local! {
    static PENDING_LUA_CODE: std::cell::RefCell<Option<String>> =
        const { std::cell::RefCell::new(None) };
    static PENDING_RESET: std::cell::RefCell<bool> =
        const { std::cell::RefCell::new(false) };
}

#[wasm_bindgen]
pub fn set_agent_code(code: &str) {
    PENDING_LUA_CODE.with(|cell| {
        *cell.borrow_mut() = Some(code.to_string());
    });
}

#[wasm_bindgen]
pub fn request_reset() {
    PENDING_RESET.with(|cell| {
        *cell.borrow_mut() = true;
    });
}

// ---------------------------------------------------------------------------
// Direction
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn opposite(self) -> Self {
        match self {
            Self::North => Self::South,
            Self::South => Self::North,
            Self::East => Self::West,
            Self::West => Self::East,
        }
    }

    fn delta(self) -> (i32, i32) {
        match self {
            Self::North => (0, 1),
            Self::South => (0, -1),
            Self::East => (1, 0),
            Self::West => (-1, 0),
        }
    }

    fn as_str(self) -> &'static str {
        match self {
            Self::North => "north",
            Self::South => "south",
            Self::East => "east",
            Self::West => "west",
        }
    }

    fn from_str(s: &str) -> Option<Self> {
        match s {
            "north" => Some(Self::North),
            "south" => Some(Self::South),
            "east" => Some(Self::East),
            "west" => Some(Self::West),
            _ => None,
        }
    }
}

/// Compute direction *from* `a` *to* `b` (adjacent cells).
fn direction_between(a: (i32, i32), b: (i32, i32)) -> Direction {
    let dx = b.0 - a.0;
    let dy = b.1 - a.1;
    if dx > 0 {
        Direction::East
    } else if dx < 0 {
        Direction::West
    } else if dy > 0 {
        Direction::North
    } else {
        Direction::South
    }
}

// ---------------------------------------------------------------------------
// Resources
// ---------------------------------------------------------------------------

#[derive(Resource)]
struct SnakeGame {
    snake: VecDeque<(i32, i32)>,
    direction: Direction,
    food: (i32, i32),
    score: u32,
    game_over: bool,
    game_over_timer: f32,
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
struct GameTimer {
    timer: Timer,
}

#[derive(Resource)]
struct SpriteAssets {
    texture: Handle<Image>,
    layout: Handle<TextureAtlasLayout>,
}

#[derive(Resource, Default)]
struct GameEntities {
    segments: Vec<Entity>,
    food: Option<Entity>,
    game_over_ui: Option<Entity>,
}

// ---------------------------------------------------------------------------
// Lua engine — non-Send because mlua::Lua is !Send
// ---------------------------------------------------------------------------

struct LuaEngine {
    lua: Lua,
    script_loaded: bool,
}

impl LuaEngine {
    fn new() -> Self {
        Self {
            lua: Lua::new(),
            script_loaded: false,
        }
    }

    fn load_script(&mut self, code: &str) -> bool {
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

    fn call_think(&self, game: &SnakeGame) -> Option<Direction> {
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

// ---------------------------------------------------------------------------
// Component markers
// ---------------------------------------------------------------------------

#[derive(Component)]
struct ScoreText;

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
    .add_systems(Startup, setup)
    .add_systems(
        Update,
        (
            receive_pending_code,
            game_tick,
            game_over_countdown,
            render_game,
            update_score_text,
        )
            .chain(),
    );

    app
}

// ---------------------------------------------------------------------------
// Setup system
// ---------------------------------------------------------------------------

fn setup(
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
// Systems
// ---------------------------------------------------------------------------

/// Poll the thread-local for new Lua code from JS (WASM) or reset requests.
fn receive_pending_code(mut lua_engine: NonSendMut<LuaEngine>, mut game: ResMut<SnakeGame>) {
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
fn game_tick(
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

    // Self collision
    if game.snake.contains(&new_head) {
        game.game_over = true;
        game.game_over_timer = GAME_OVER_DELAY;
        return;
    }

    game.snake.push_front(new_head);

    if new_head == game.food {
        game.score += 1;
        game.food = random_food_position(&game.snake);
    } else {
        game.snake.pop_back();
    }
}

/// Count down the game-over delay, then reset.
fn game_over_countdown(time: Res<Time>, mut game: ResMut<SnakeGame>) {
    if !game.game_over {
        return;
    }
    game.game_over_timer -= time.delta_secs();
    if game.game_over_timer <= 0.0 {
        reset_game_state(&mut game);
    }
}

/// Synchronise Bevy entities with the current `SnakeGame` state.
fn render_game(
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
fn update_score_text(game: Res<SnakeGame>, mut query: Query<&mut Text, With<ScoreText>>) {
    for mut text in &mut query {
        **text = format!("Score: {}", game.score);
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn grid_to_world(gx: i32, gy: i32) -> Vec3 {
    let half = (GRID_SIZE as f32 - 1.0) / 2.0;
    Vec3::new(
        (gx as f32 - half) * CELL_SIZE,
        (gy as f32 - half) * CELL_SIZE,
        0.0,
    )
}

fn random_food_position(snake: &VecDeque<(i32, i32)>) -> (i32, i32) {
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

fn reset_game_state(game: &mut SnakeGame) {
    let fresh = SnakeGame::default();
    game.snake = fresh.snake;
    game.direction = fresh.direction;
    game.food = fresh.food;
    game.score = 0;
    game.game_over = false;
    game.game_over_timer = 0.0;
}

/// Pick the correct sprite atlas index for the snake segment at `index`.
fn sprite_index_for_segment(game: &SnakeGame, index: usize) -> usize {
    let len = game.snake.len();
    if len < 2 {
        return SPRITE_HEAD_E;
    }

    let cur = game.snake[index];

    if index == 0 {
        // Head — direction name = where the body is relative to head
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
        // Tail — direction name = where the body is relative to tail
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
