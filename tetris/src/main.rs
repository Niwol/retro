use bevy::prelude::*;
use tetris::{constants::WINDOW_SIZE, menu::MenuPlugin, tetris::TetrisPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WINDOW_SIZE.into(),
                title: String::from("Tetris"),
                resizable: false,
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins((TetrisPlugin, MenuPlugin))
        .add_systems(Update, exit)
        .run();
}

fn exit(input: Res<ButtonInput<KeyCode>>, mut exit_event: EventWriter<AppExit>) {
    if input.just_pressed(KeyCode::Escape) {
        exit_event.send_default();
    }
}
