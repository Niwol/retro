use bevy::prelude::*;

// Constants
pub const WINDOW_RESOLUTION: [f32; 2] = [MENU_SIZE.x + GAME_SIZE.x, MENU_SIZE.y];
pub const GAME_SIZE: Vec2 = Vec2 { x: 800.0, y: 600.0 };
pub const MENU_SIZE: Vec2 = Vec2 { x: 300.0, y: 800.0 };

const MARGINE_TOP_BOTTOM: f32 = (MENU_SIZE.y - GAME_SIZE.y) / 2.0;
pub const MENU_AREA: Rect = Rect {
    min: Vec2 { x: 0.0, y: 0.0 },
    max: MENU_SIZE,
};

pub const GAME_AREA: Rect = Rect {
    min: Vec2 {
        x: MENU_AREA.max.x,
        y: MARGINE_TOP_BOTTOM,
    },
    max: Vec2 {
        x: MENU_AREA.max.x + GAME_SIZE.x,
        y: GAME_SIZE.y + MARGINE_TOP_BOTTOM,
    },
};

// Application states
#[derive(States, Hash, PartialEq, Eq, Clone, Debug)]
pub enum CurrentGame {
    InMainMenu,
    Breakout,
}

// Resources
#[derive(Resource)]
pub struct GameEntity(pub Entity);

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
        .add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    let game_entity = commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::BLACK,
                rect: Some(Rect::from_center_size(Vec2::ZERO, GAME_AREA.size())),
                ..Default::default()
            },
            transform: Transform::from_translation(
                Vec3::X * (GAME_AREA.center().x - WINDOW_RESOLUTION[0] / 2.0),
            ),
            ..Default::default()
        })
        .id();

    commands.insert_resource(GameEntity(game_entity));
}
