use bevy::math::Vec2;

pub const NB_ROWS: usize = 16;
pub const NB_COLS: usize = 10;
pub const BLOCK_SIZE: f32 = 32.0;

pub const POWER_WIDTH: f32 = 150.0;
pub const GAME_WIDTH: f32 = BLOCK_SIZE * NB_COLS as f32;
pub const INFO_WIDTH: f32 = 200.0;

pub const TOP_MARGINE: f32 = BLOCK_SIZE * 2.0;
pub const GAME_HEIGHT: f32 = BLOCK_SIZE * NB_ROWS as f32;

pub const WINDOW_SIZE: Vec2 = Vec2 {
    x: POWER_WIDTH + GAME_WIDTH + INFO_WIDTH,
    y: GAME_HEIGHT + TOP_MARGINE,
};

pub const WINDOW_TOP: f32 = WINDOW_SIZE.y / 2.0;
pub const WINDOW_BOTTOM: f32 = -WINDOW_SIZE.y / 2.0;
pub const WINDOW_LEFT: f32 = -WINDOW_SIZE.x / 2.0;
pub const WINDOW_RIGHT: f32 = WINDOW_SIZE.x / 2.0;

pub const GAME_TOP: f32 = GAME_BOTTOM + GAME_HEIGHT;
pub const GAME_BOTTOM: f32 = WINDOW_BOTTOM;
pub const GAME_LEFT: f32 = WINDOW_LEFT + POWER_WIDTH;
pub const GAME_RIGHT: f32 = GAME_LEFT + GAME_WIDTH;

pub const GAME_CENTER: Vec2 = Vec2 {
    x: (GAME_LEFT + GAME_RIGHT) / 2.0,
    y: (GAME_BOTTOM + GAME_TOP) / 2.0,
};

pub const INFO_NEXT_BLOCK_SIZE: Vec2 = Vec2 {
    x: INFO_WIDTH,
    y: BLOCK_SIZE * 5.0 + TOP_MARGINE,
};
