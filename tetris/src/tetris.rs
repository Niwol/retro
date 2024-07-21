use bevy::color::palettes;
use bevy::prelude::*;

use crate::{
    constants::{
        BLOCK_SIZE, GAME_CENTER, GAME_RIGHT, INFO_NEXT_BLOCK_SIZE, NB_COLS, NB_ROWS, WINDOW_RIGHT,
        WINDOW_TOP,
    },
    TetrisState,
};

pub struct TetrisPlugin;

impl Plugin for TetrisPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<LoadGameEvent>()
            .add_event::<ClearGameEvent>()
            .add_systems(Startup, setup)
            .add_systems(First, load_game.run_if(on_event::<LoadGameEvent>()))
            .add_systems(
                Update,
                (
                    spawn_next_tetromino,
                    take_next_tetromino,
                    place_falling_tetromino,
                    fall,
                )
                    .run_if(in_state(TetrisState::InGame)),
            )
            .add_systems(Update, draw_gizmos);
    }
}

#[derive(Event, Default)]
pub struct LoadGameEvent;

#[derive(Event, Default)]
pub struct ClearGameEvent;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn load_game() {}

fn spawn_next_tetromino() {}

fn take_next_tetromino() {}

fn place_falling_tetromino() {}

fn fall() {}

fn draw_gizmos(mut gizmos: Gizmos) {
    let color = palettes::basic::RED;
    gizmos
        .grid_2d(
            GAME_CENTER,
            0.0,
            UVec2 {
                x: NB_COLS as u32,
                y: NB_ROWS as u32,
            },
            Vec2::ONE * BLOCK_SIZE,
            color,
        )
        .outer_edges();

    gizmos.line_2d(
        Vec2 {
            x: GAME_RIGHT,
            y: WINDOW_TOP - INFO_NEXT_BLOCK_SIZE.y,
        },
        Vec2 {
            x: WINDOW_RIGHT,
            y: WINDOW_TOP - INFO_NEXT_BLOCK_SIZE.y,
        },
        color,
    );
}
