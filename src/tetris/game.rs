use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::on_timer};

use super::{grid::GameGrid, TetrisState};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<LoadGameEvent>()
            .add_event::<CleanupGameEvent>()
            .add_systems(
                Update,
                (
                    load_game.run_if(on_event::<LoadGameEvent>()),
                    cleanup_game.run_if(on_event::<CleanupGameEvent>()),
                    (update.run_if(on_timer(Duration::from_secs_f32(0.5))))
                        .run_if(in_state(TetrisState::InGame)),
                ),
            );
    }
}

#[derive(Event, Default)]
pub struct LoadGameEvent;

#[derive(Event, Default)]
pub struct CleanupGameEvent;

fn load_game(mut commands: Commands) {
    commands.insert_resource(GameGrid::new());
}

fn cleanup_game(mut commands: Commands) {
    commands.remove_resource::<GameGrid>();
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum MoveDirection {
    Up,
    Down,
    Left,
    Right,
}

fn update() {
    println!("Update");
}

fn _draw_wireframe(mut _gizmos: Gizmos) {
    //    for x in 0..NB_COLS {
    //        for y in 0..NB_ROWS {
    //            let position = Vec2 {
    //                x: BLOCK_AREA.min.x + x as f32 * 32.0 + 16.0,
    //                y: BLOCK_AREA.min.y + y as f32 * 32.0 + 16.0,
    //            };
    //            gizmos.rect_2d(position, 0.0, vec2(32.0, 32.0), Color::RED);
    //        }
    //    }
}
