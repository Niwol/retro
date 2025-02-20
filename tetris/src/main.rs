use bevy::prelude::*;
use tetris::tetris::{LoadGameEvent, TetrisPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (1000.0, 800.0).into(), // WINDOW_SIZE.into(),
                title: String::from("Tetris"),
                resizable: false,
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins(TetrisPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, exit)
        .run();
}

fn setup(mut load_game_event: EventWriter<LoadGameEvent>) {
    load_game_event.send_default();
}

fn exit(input: Res<ButtonInput<KeyCode>>, mut exit_event: EventWriter<AppExit>) {
    if input.just_pressed(KeyCode::Escape) {
        exit_event.send_default();
    }
}
