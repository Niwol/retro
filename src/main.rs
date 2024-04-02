use bevy::prelude::*;
use retro::{application::Application, breakout::BreackoutPlugin, menu::MenuPlugin};

fn main() {
    App::new()
        .add_plugins(Application)
        .add_plugins(MenuPlugin)
        .add_plugins(BreackoutPlugin)
        .run();
}
