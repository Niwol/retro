use bevy::color::palettes;
use bevy::prelude::*;

// Constants
pub const WINDOW_RESOLUTION: [f32; 2] = [MENU_SIZE.x + GAME_SIZE.x, MENU_SIZE.y];
pub const GAME_SIZE: Vec2 = Vec2 { x: 800.0, y: 600.0 };
pub const MENU_SIZE: Vec2 = Vec2 { x: 300.0, y: 800.0 };

pub const MENU_AREA: Rect = Rect {
    min: Vec2 {
        x: -WINDOW_RESOLUTION[0] / 2.0,
        y: -WINDOW_RESOLUTION[1] / 2.0,
    },
    max: Vec2 {
        x: -WINDOW_RESOLUTION[0] / 2.0 + MENU_SIZE.x,
        y: -WINDOW_RESOLUTION[1] / 2.0 + MENU_SIZE.y,
    },
};

pub const GAME_AREA: Rect = Rect {
    min: Vec2 {
        x: MENU_AREA.max.x,
        y: -GAME_SIZE.y / 2.0,
    },
    max: Vec2 {
        x: MENU_AREA.max.x + GAME_SIZE.x,
        y: GAME_SIZE.y / 2.0,
    },
};

// Application states
#[derive(States, Hash, PartialEq, Eq, Clone, Debug)]
pub enum CurrentGame {
    InMainMenu,
    Breakout,
    Tetris,
}

pub struct Application;

impl Plugin for Application {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WINDOW_RESOLUTION.into(),
                title: String::from("Retro Games"),
                resizable: false,
                ..Default::default()
            }),
            ..Default::default()
        }))
        .insert_state(CurrentGame::InMainMenu)
        .insert_resource(ClearColor(palettes::basic::GRAY.into()))
        .add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
