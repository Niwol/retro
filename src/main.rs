use bevy::prelude::*;
use retro::{
    application::Application, breakout::BreackoutPlugin, menu::MenuPlugin, tetris::TetrisPlugin,
};

fn main() {
    App::new()
        .add_plugins(Application)
        .add_plugins(MenuPlugin)
        .add_plugins(BreackoutPlugin)
        .add_plugins(TetrisPlugin)
        .run();
}
