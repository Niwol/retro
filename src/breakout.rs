use bevy::prelude::*;

use crate::application::{CurrentGame, GameEntity, GAME_SIZE};

const PLAYER_SIZE: Vec2 = Vec2 { x: 50.0, y: 10.0 };
const PLAYER_SPEED: f32 = 200.0;
const PLAYER_AXIS: f32 = -GAME_SIZE.y / 2.0 + 20.0;

#[derive(Component)]
struct Player;

pub struct BreackoutPlugin;

impl Plugin for BreackoutPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(CurrentGame::Breakout), load)
            .add_systems(OnExit(CurrentGame::Breakout), cleanup)
            .add_systems(
                Update,
                (handle_player_input, handle_game_exit).run_if(in_state(CurrentGame::Breakout)),
            );
    }
}

fn load(mut commands: Commands, game_entity: Res<GameEntity>) {
    commands.entity(game_entity.0).with_children(|builder| {
        builder.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::PURPLE,
                    custom_size: Some(PLAYER_SIZE),
                    ..Default::default()
                },
                transform: Transform::from_translation(Vec3::Y * PLAYER_AXIS),
                ..Default::default()
            },
            Player,
        ));
    });
}

fn cleanup(mut commands: Commands, game_entity: Res<GameEntity>) {
    commands.entity(game_entity.0).despawn_descendants();
}

fn handle_player_input(
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut player: Query<&mut Transform, With<Player>>,
) {
    let mut player_transform = player.single_mut();
    let dt = time.delta().as_secs_f32();
    if input.pressed(KeyCode::ArrowLeft) {
        player_transform.translation.x -= PLAYER_SPEED * dt;
    }

    if input.pressed(KeyCode::ArrowRight) {
        player_transform.translation.x += PLAYER_SPEED * dt;
    }

    if player_transform.translation.x - PLAYER_SIZE.x / 2.0 < -GAME_SIZE.x / 2.0 {
        player_transform.translation.x = -GAME_SIZE.x / 2.0 + PLAYER_SIZE.x / 2.0;
    }

    if player_transform.translation.x + PLAYER_SIZE.x / 2.0 > GAME_SIZE.x / 2.0 {
        player_transform.translation.x = GAME_SIZE.x / 2.0 - PLAYER_SIZE.x / 2.0;
    }
}

fn handle_game_exit(
    input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<CurrentGame>>,
) {
    if input.just_pressed(KeyCode::Escape) {
        println!("Closing Breakout");
        next_state.set(CurrentGame::InMainMenu);
    }
}
