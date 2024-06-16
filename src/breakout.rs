use std::{f32::consts::PI, usize};

use bevy::prelude::*;

use crate::application::{CurrentGame, GAME_AREA, GAME_SIZE, WINDOW_RESOLUTION};

const PLAYER_SIZE: Vec2 = Vec2 { x: 150.0, y: 15.0 };
const PLAYER_GROW_SIZE: Vec2 = Vec2 { x: 300.0, y: 15.0 };
const PLAYER_SHRINK_SIZE: Vec2 = Vec2 { x: 100.0, y: 15.0 };
const PLAYER_AXIS: f32 = -GAME_SIZE.y / 2.0 + 20.0;

const PLAYER_SPEED: f32 = 300.0;
const BALL_SPEED: f32 = 300.0;
const UPGRADE_SPEED: f32 = 100.0;

const BALL_COLOR: Color = Color::rgb(0.2, 1.0, 0.2);

const NB_BRICK_COLS: usize = 15;
const NB_BRICK_ROWS: usize = 8;

const BRICK_AREA: Vec2 = Vec2 {
    x: GAME_SIZE.x,
    y: GAME_SIZE.y / 2.5,
};

const BRICK_SIZE: Vec2 = Vec2 {
    x: BRICK_AREA.x / NB_BRICK_COLS as f32 - 6.0,
    y: BRICK_AREA.y / NB_BRICK_ROWS as f32 - 6.0,
};

const UPGRADE_SIZE: Vec2 = Vec2 { x: 40.0, y: 20.0 };

const TOTAL_LEVELS: usize = 5;

pub struct BreackoutPlugin;

impl Plugin for BreackoutPlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(GameState::Exited)
            .insert_state(InGameState::Paused)
            .add_event::<LoadLevelEvent>()
            .add_event::<SpawnBallEvent>()
            .add_event::<SpawnUpgradeEvent>()
            .add_event::<DespawnBallEvent>()
            .add_event::<DespawnBrickEvent>()
            .add_event::<DespawnUpgradeEvent>()
            .add_event::<GameOverEvent>()
            .add_event::<GameWonEvent>()
            .add_systems(OnEnter(CurrentGame::Breakout), (load_game, load_menu))
            .add_systems(
                OnExit(CurrentGame::Breakout),
                (cleanup_level, cleanup_game, cleanup_menu).chain(),
            )
            .add_systems(
                OnEnter(GameState::InMenu),
                show_menu.run_if(in_state(CurrentGame::Breakout)),
            )
            .add_systems(
                OnExit(GameState::InMenu),
                hide_menu.run_if(in_state(CurrentGame::Breakout)),
            )
            .add_systems(
                PreUpdate,
                (
                    (cleanup_level, (setup, load_player, load_ball, load_level))
                        .chain()
                        .run_if(on_event::<LoadLevelEvent>()),
                    spawn_ball.run_if(on_event::<SpawnBallEvent>()),
                    spawn_upgrade.run_if(on_event::<SpawnUpgradeEvent>()),
                )
                    .run_if(in_state(CurrentGame::Breakout)),
            )
            .add_systems(
                Update,
                (
                    (
                        (handle_menu_navigation_input, handle_menu_select_input),
                        update_menu,
                    )
                        .chain()
                        .run_if(in_state(GameState::InMenu)),
                    (
                        (
                            (
                                update_balls,
                                solve_ball_walls_colisions,
                                solve_ball_player_colisions,
                                solve_ball_brick_colisions,
                                update_ball_transforms,
                            )
                                .chain(),
                            handle_player_input,
                            (update_upgrades, catch_upgrade).chain(),
                            tick_upgrade_timer,
                        )
                            .run_if(in_state(InGameState::Playing)),
                        handle_pause_input.run_if(in_state(InGameState::Paused)),
                    )
                        .run_if(in_state(GameState::InGame)),
                )
                    .run_if(in_state(CurrentGame::Breakout)),
            )
            .add_systems(
                PostUpdate,
                (
                    despawn_ball.run_if(on_event::<DespawnBallEvent>()),
                    despawn_brick.run_if(on_event::<DespawnBrickEvent>()),
                    despawn_upgrade.run_if(on_event::<DespawnUpgradeEvent>()),
                    (game_over, cleanup_level).run_if(on_event::<GameOverEvent>()),
                    (game_won, cleanup_level).run_if(on_event::<GameWonEvent>()),
                )
                    .run_if(in_state(CurrentGame::Breakout)),
            );
    }
}

#[derive(Component)]
struct Ball {
    radius: f32,
    velocity: Vec2,
    current_position: Vec2,
    old_position: Vec2,
}

impl Ball {
    fn bounce(&mut self, rect: Rect) -> bool {
        let nb_steps = 50;

        let travel_line = self.current_position - self.old_position;

        for i in 0..nb_steps {
            let step = i as f32 / (nb_steps as f32 - 1.0);

            let mut new_ball = Ball {
                current_position: self.old_position + travel_line * step,
                ..*self
            };

            if new_ball.bounce_of_rect(rect) {
                *self = new_ball;
                return true;
            }
        }

        false
    }

    fn bounce_of_rect(&mut self, rect: Rect) -> bool {
        let closet_point = Vec2 {
            x: f32::clamp(self.current_position.x, rect.min.x, rect.max.x),
            y: f32::clamp(self.current_position.y, rect.min.y, rect.max.y),
        };

        let to_rect = closet_point - self.current_position;
        let dist = to_rect.length();

        if dist < self.radius {
            let overlap = self.radius - dist;
            let n = to_rect.normalize();
            self.current_position -= n * (overlap + 0.001);

            self.velocity -= n * self.velocity.dot(n) * 2.0;

            return true;
        }

        false
    }

    fn bounce_of_player_rect(&mut self, player_rect: Rect) {
        let extended_player_rect =
            Rect::from_center_size(player_rect.center(), player_rect.size() + self.radius);

        let p11 = Vec2::new(extended_player_rect.min.x, extended_player_rect.max.y);
        let p12 = Vec2::new(extended_player_rect.max.x, extended_player_rect.max.y);

        let p21 = self.old_position;
        let p22 = self.current_position;

        if segment_intersets(p11, p12, p21, p22) {
            self.bounce_off_player(player_rect);
        }
    }

    fn bounce_off_player(&mut self, player_rect: Rect) {
        let hit_x = self.current_position.x - player_rect.center().x;
        let hit_x = hit_x / player_rect.width();
        let hit_x = hit_x.clamp(-1.0, 1.0);

        let out_dir_x = hit_x;
        let out_dir_y = hit_x.acos().sin();
        let new_dir = Vec2::new(out_dir_x, out_dir_y);
        self.velocity = new_dir;
        self.current_position.y = PLAYER_AXIS + player_rect.height() / 2.0 + self.radius;
    }
}

#[derive(PartialEq, Eq, Debug)]
enum PointOrientation {
    CW,
    CCW,
    Colinear,
}

fn segment_intersets(p11: Vec2, p12: Vec2, p21: Vec2, p22: Vec2) -> bool {
    let ori_1 = get_orientation(p11, p12, p21);
    let ori_2 = get_orientation(p11, p12, p22);

    if ori_1 == ori_2 {
        return false;
    }

    let ori_1 = get_orientation(p21, p22, p11);
    let ori_2 = get_orientation(p21, p22, p12);

    if ori_1 == ori_2 {
        return false;
    }

    true
}

fn get_orientation(p1: Vec2, p2: Vec2, p3: Vec2) -> PointOrientation {
    let diff = (p2.y - p1.y) * (p3.x - p2.x) - (p3.y - p2.y) * (p2.x - p1.x);

    if diff == 0.0 {
        PointOrientation::Colinear
    } else if diff < 0.0 {
        PointOrientation::CW
    } else {
        PointOrientation::CCW
    }
}

#[derive(Event, Default)]
struct LoadLevelEvent(usize);

#[derive(Event)]
struct SpawnBallEvent {
    location: Vec2,
    initial_velocity: Vec2,
}

#[derive(Event)]
struct SpawnUpgradeEvent {
    location: Vec2,
}

#[derive(Event)]
struct DespawnBallEvent(Entity);

#[derive(Event)]
struct DespawnBrickEvent(Entity);

#[derive(Event)]
struct DespawnUpgradeEvent(Entity);

#[derive(Event, Default)]
struct GameOverEvent;

#[derive(Event, Default)]
struct GameWonEvent;

#[derive(Resource)]
struct GameAssets {
    ball_sprite: Handle<Image>,
    player_sprit: Handle<Image>,
    normal_bricks: Vec<Handle<Image>>,
    spawner_brick: Handle<Image>,
    upgrade_brick: Handle<Image>,
    grow_upgrade: Handle<Image>,
    shrink_upgrade: Handle<Image>,
}

#[derive(Resource)]
struct LevelLoaded(bool);

#[derive(Resource)]
struct LastLevelPlayed(usize);

#[derive(Resource)]
struct UpgradeTimer {
    timer: Timer,
}

#[derive(Component)]
struct LastLevelCompleteText;

#[derive(Component)]
struct Background;

#[derive(States, Debug, PartialEq, Eq, Hash, Clone)]
enum GameState {
    Exited,
    InMenu,
    InGame,
}

#[derive(States, Debug, PartialEq, Eq, Hash, Clone)]
enum InGameState {
    Playing,
    Paused,
}

#[derive(Component)]
struct Player;

enum BrickType {
    Normal,
    BallSpawner,
    Upgrade,
}

#[derive(Component)]
struct Brick {
    lives: i32,
    brick_type: BrickType,
}

#[derive(Component)]
enum UpgradeComponent {
    Grow,
    Shrink,
}

fn load_game(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut next_in_game_state: ResMut<NextState<InGameState>>,
) {
    // Spawn Background
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::BLACK,
                custom_size: Some(GAME_SIZE),
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::from((GAME_AREA.center(), -1.0))),
            ..Default::default()
        },
        Background,
    ));

    // Spawn last level complete text
    commands.spawn((
        Text2dBundle {
            text: Text::from_sections([
                TextSection::new("Congratulations !!!\n", TextStyle {
                    font_size: 50.0,
                    color: Color::GREEN,
                    ..Default::default()
                }),
                TextSection::new(
                    "you completed the last level\n\n",
                    TextStyle {
                        font_size: 30.0,
                        color: Color::GREEN,
                        ..Default::default()
                    },
                ),
                TextSection::new(
                    "I was a little too lazy to implement a special menu for this situation\nso both menu options will bring you back to the main menu ^^\"",
                    TextStyle {
                        font_size: 15.0,
                        color: Color::GREEN,
                        ..Default::default()
                    },
                ),
            ]).with_justify(JustifyText::Center),
            transform: Transform::from_translation(Vec3::from((GAME_AREA.center() + Vec2::Y * 150.0, 1.0))),
            
            // gets visible once the player completes the last level
            visibility: Visibility::Hidden, 
            ..Default::default()
        },
        LastLevelCompleteText,
    ));

    // Load game assets
    let ball_sprite = asset_server.load("breakout/sprites/balls/ball.png");
    let player_sprit = asset_server.load("breakout/sprites/player/player.png");
    let brick_sprites = (1..=5)
        .into_iter()
        .map(|i| asset_server.load(format!("breakout/sprites/bricks/normal_{i}.png")))
        .collect();
    let spawner_brick = asset_server.load("breakout/sprites/bricks/spawner.png");
    let upgrade_brick = asset_server.load("breakout/sprites/bricks/upgrade.png");
    let grow_upgrade = asset_server.load("breakout/sprites/upgrades/grow_upgrade.png");
    let shrink_upgrade = asset_server.load("breakout/sprites/upgrades/shrink_upgrade.png");

    let game_assets = GameAssets {
        ball_sprite,
        player_sprit,
        normal_bricks: brick_sprites,
        spawner_brick,
        upgrade_brick,
        grow_upgrade,
        shrink_upgrade,
    };

    commands.insert_resource(game_assets);
    commands.insert_resource(LevelLoaded(false));
    commands.insert_resource(LastLevelPlayed(0));
    let mut timer = Timer::from_seconds(15.0, TimerMode::Once);
    timer.pause();
    commands.insert_resource(UpgradeTimer { timer });

    next_game_state.set(GameState::InMenu);
    next_in_game_state.set(InGameState::Paused);
}

fn cleanup_level(
    mut commands: Commands,
    player_entity: Query<Entity, With<Player>>,
    ball_entities: Query<Entity, With<Ball>>,
    brick_entities: Query<Entity, With<Brick>>,
    upgrade_entities: Query<Entity, With<UpgradeComponent>>,
    mut level_loaded: ResMut<LevelLoaded>,
) {
    if level_loaded.0 {
        commands.entity(player_entity.single()).despawn();

        for entity in &ball_entities {
            commands.entity(entity).despawn();
        }

        for entity in &brick_entities {
            commands.entity(entity).despawn();
        }

        for entity in &upgrade_entities {
            commands.entity(entity).despawn();
        }

        level_loaded.0 = false;
    }
}

fn cleanup_game(
    mut commands: Commands,
    entities: Query<Entity, Or<(With<LastLevelCompleteText>, With<Background>)>>,
    mut sprites: ResMut<Assets<Image>>,
    game_assets: Res<GameAssets>,
) {
    for entity in &entities {
        commands.entity(entity).despawn();
    }

    sprites.remove(game_assets.player_sprit.clone());
    sprites.remove(game_assets.ball_sprite.clone());
    for brick_sprite in &game_assets.normal_bricks {
        sprites.remove(brick_sprite.clone());
    }
    sprites.remove(game_assets.spawner_brick.clone());
    sprites.remove(game_assets.upgrade_brick.clone());
    sprites.remove(game_assets.grow_upgrade.clone());
    sprites.remove(game_assets.shrink_upgrade.clone());

    commands.remove_resource::<GameAssets>();
    commands.remove_resource::<LevelLoaded>();
    commands.remove_resource::<LastLevelPlayed>();
    commands.remove_resource::<UpgradeTimer>();
}

fn setup(mut next_in_game_state: ResMut<NextState<InGameState>>) {
    next_in_game_state.set(InGameState::Paused);
}

fn load_level(
    mut commands: Commands,
    bricks: Query<Entity, With<Brick>>,
    game_assets: Res<GameAssets>,
    mut level_loaded: ResMut<LevelLoaded>,
    mut last_level_played: ResMut<LastLevelPlayed>,
    mut load_events: EventReader<LoadLevelEvent>,
) {
    // Clear bricks first
    for brick_entity in &bricks {
        commands.entity(brick_entity).despawn();
    }

    let mut event_reader = load_events.read();

    if let Some(load_event) = event_reader.next() {
        let level = load_event.0;

        let level_file = format!("assets/breakout/levels/level_{level}.txt");
        let file_content = std::fs::read_to_string(level_file).unwrap();
        let mut char_iter = file_content.split_whitespace();

        let nb_cols = char_iter.next().unwrap().parse::<usize>().unwrap();
        let nb_rows = char_iter.next().unwrap().parse::<usize>().unwrap();

        for y in 0..nb_rows {
            for x in 0..nb_cols {
                let extended_brick_size = BRICK_SIZE + 6.0;
                let center_offset = extended_brick_size / 2.0;
                let position = Vec3 {
                    x: GAME_AREA.min.x + center_offset.x + x as f32 * extended_brick_size.x,
                    y: GAME_AREA.max.y - center_offset.y - y as f32 * extended_brick_size.y,
                    z: 1.0,
                };

                let next_char = char_iter.next().unwrap();
                let brick = if let Ok(lives) = next_char.parse::<i32>() {
                    Some(Brick {
                        lives,
                        brick_type: BrickType::Normal,
                    })
                } else if next_char == "S" {
                    Some(Brick {
                        lives: 1,
                        brick_type: BrickType::BallSpawner,
                    })
                } else if next_char == "U" {
                    Some(Brick {
                        lives: 1,
                        brick_type: BrickType::Upgrade,
                    })
                } else {
                    None
                };

                if let Some(brick) = brick {
                    commands.spawn((
                        SpriteBundle {
                            sprite: Sprite {
                                custom_size: Some(BRICK_SIZE),
                                ..Default::default()
                            },
                            transform: Transform::from_translation(position),
                            texture: match brick.brick_type {
                                BrickType::Normal => {
                                    game_assets.normal_bricks[brick.lives as usize - 1].clone()
                                }
                                BrickType::BallSpawner => game_assets.spawner_brick.clone(),
                                BrickType::Upgrade => game_assets.upgrade_brick.clone(),
                            },
                            ..Default::default()
                        },
                        brick,
                    ));
                }
            }
        }

        level_loaded.0 = true;
        last_level_played.0 = level;
        load_events.clear();
    }
}

fn load_player(mut commands: Commands, game_assets: Res<GameAssets>) {
    // Spawn player
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(PLAYER_SIZE),
                ..Default::default()
            },
            texture: game_assets.player_sprit.clone(),
            transform: Transform::from_translation(Vec3::from((
                GAME_AREA.center() + Vec2::Y * PLAYER_AXIS,
                0.0,
            ))),
            ..Default::default()
        },
        Player,
    ));
}

fn load_ball(
    mut commands: Commands,
    balls: Query<Entity, With<Ball>>,
    mut ball_spawn_event: EventWriter<SpawnBallEvent>,
) {
    // Clear balls first
    for ball_entity in &balls {
        commands.entity(ball_entity).despawn();
    }

    ball_spawn_event.send(SpawnBallEvent {
        location: GAME_AREA.center() + Vec2::Y * PLAYER_AXIS + Vec2::Y * 30.0,
        initial_velocity: Vec2::Y,
    });
}

fn spawn_ball(
    mut ball_spawn_event: EventReader<SpawnBallEvent>,
    mut commands: Commands,
    game_assets: Res<GameAssets>,
) {
    let radius = 10.0;
    for spawn_event in ball_spawn_event.read() {
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: BALL_COLOR,
                    custom_size: Some(Vec2::ONE * radius * 2.0),
                    ..Default::default()
                },

                texture: game_assets.ball_sprite.clone(),

                transform: Transform::from_translation(Vec3::from((spawn_event.location, 1.0))),
                ..Default::default()
            },
            Ball {
                velocity: spawn_event.initial_velocity,
                radius,
                current_position: spawn_event.location,
                old_position: spawn_event.location,
            },
        ));
    }
}

fn spawn_upgrade(
    mut upgrade_spawn_event: EventReader<SpawnUpgradeEvent>,
    mut commands: Commands,
    game_assets: Res<GameAssets>,
) {
    for spawn_event in upgrade_spawn_event.read() {
        let upgrade_component = if rand::random::<f32>() < 0.3 {
            UpgradeComponent::Shrink
        } else {
            UpgradeComponent::Grow
        };

        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(UPGRADE_SIZE),
                    ..Default::default()
                },

                texture: match upgrade_component {
                    UpgradeComponent::Grow => game_assets.grow_upgrade.clone(),
                    UpgradeComponent::Shrink => game_assets.shrink_upgrade.clone(),
                },

                transform: Transform::from_translation(Vec3::from((spawn_event.location, 1.0))),
                ..Default::default()
            },
            upgrade_component,
        ));
    }
}

fn despawn_ball(
    mut commands: Commands,
    mut despawn_ball_event: EventReader<DespawnBallEvent>,
    mut game_over_event: EventWriter<GameOverEvent>,
    balls: Query<&Ball>,
) {
    let nb_balls_despawned = despawn_ball_event.len();
    for event in despawn_ball_event.read() {
        if let Some(mut entity_commands) = commands.get_entity(event.0) {
            entity_commands.despawn();
        }
    }

    if nb_balls_despawned >= balls.iter().count() {
        game_over_event.send_default();
    }
}

fn despawn_brick(
    mut commands: Commands,
    mut despawn_brick_event: EventReader<DespawnBrickEvent>,
    mut ball_spawn_event: EventWriter<SpawnBallEvent>,
    mut upgrade_spawn_event: EventWriter<SpawnUpgradeEvent>,
    mut game_won_event: EventWriter<GameWonEvent>,
    bricks: Query<(&Transform, &Brick)>,
) {
    let nb_bricks_despawned = despawn_brick_event.len();

    for event in despawn_brick_event.read() {
        let (brick_transform, brick) = bricks.get(event.0).unwrap();
        match brick.brick_type {
            BrickType::BallSpawner => {
                let theta = rand::random::<f32>() * PI * 2.0;
                let x = f32::cos(theta);
                let y = f32::sin(theta);

                ball_spawn_event.send(SpawnBallEvent {
                    location: brick_transform.translation.xy(),
                    initial_velocity: Vec2::new(x, y),
                });
            }
            BrickType::Upgrade => {
                upgrade_spawn_event.send(SpawnUpgradeEvent {
                    location: brick_transform.translation.xy(),
                });
            }
            _ => (),
        }

        if let Some(mut entity_commands) = commands.get_entity(event.0) {
            entity_commands.despawn();
        }
    }

    if nb_bricks_despawned >= bricks.iter().count() {
        game_won_event.send_default();
    }
}

fn update_balls(time: ResMut<Time>, mut balls: Query<&mut Ball>) {
    let dt = time.delta().as_secs_f32();
    for mut ball in &mut balls {
        let velocity = ball.velocity;

        ball.old_position = ball.current_position;
        ball.current_position += velocity * BALL_SPEED * dt;
    }
}

fn solve_ball_walls_colisions(
    mut balls: Query<(Entity, &mut Ball)>,
    mut despawn_ball_event: EventWriter<DespawnBallEvent>,
) {
    for (entity, mut ball) in &mut balls {
        // Top
        if ball.current_position.y + ball.radius > GAME_AREA.max.y {
            ball.current_position.y = GAME_AREA.max.y - ball.radius;
            ball.velocity.y *= -1.0;
        }

        // Left
        if ball.current_position.x - ball.radius < GAME_AREA.min.x {
            ball.current_position.x = GAME_AREA.min.x + ball.radius;
            ball.velocity.x *= -1.0;
        }

        // Right
        if ball.current_position.x + ball.radius > GAME_AREA.max.x {
            ball.current_position.x = GAME_AREA.max.x - ball.radius;
            ball.velocity.x *= -1.0;
        }

        // Bottom
        if ball.current_position.y - ball.radius < GAME_AREA.min.y {
            despawn_ball_event.send(DespawnBallEvent(entity));
        }
    }
}

fn solve_ball_player_colisions(
    mut balls: Query<&mut Ball, Without<Player>>,
    player: Query<(&Transform, &Sprite), With<Player>>,
) {
    let (player_transform, player_sprite) = player.single();
    let player_size = player_sprite.custom_size.unwrap();
    for mut ball in &mut balls {
        let player_rect = Rect::from_center_size(player_transform.translation.xy(), player_size);
        ball.bounce_of_player_rect(player_rect);
    }
}

fn solve_ball_brick_colisions(
    mut balls: Query<&mut Ball>,
    mut bricks: Query<(Entity, &Transform, &mut Brick, &mut Handle<Image>)>,
    game_assets: Res<GameAssets>,
    mut despawn_brick_event: EventWriter<DespawnBrickEvent>,
) {
    for mut ball in &mut balls {
        for (brick_entity, brick_transform, mut brick, mut image_handle) in &mut bricks {
            let brick_rect = Rect::from_center_size(brick_transform.translation.xy(), BRICK_SIZE);

            if ball.bounce(brick_rect) {
                brick.lives -= 1;
                if brick.lives == 0 {
                    despawn_brick_event.send(DespawnBrickEvent(brick_entity));
                } else {
                    let sprite_index = i32::max(brick.lives - 1, 0);

                    *image_handle = game_assets.normal_bricks[sprite_index as usize].clone();
                }
            }
        }
    }
}

fn update_ball_transforms(mut balls: Query<(&Ball, &mut Transform)>) {
    for (ball, mut transform) in &mut balls {
        transform.translation = Vec3::from((ball.current_position, 1.0));
    }
}

fn update_upgrades(
    time: Res<Time>,
    mut uprades: Query<(Entity, &mut Transform), With<UpgradeComponent>>,
    mut despawn_upgrade_event: EventWriter<DespawnUpgradeEvent>,
) {
    let dt = time.delta().as_secs_f32();

    for (entity, mut upgrade_transform) in &mut uprades {
        upgrade_transform.translation.y -= UPGRADE_SPEED * dt;

        if upgrade_transform.translation.y < GAME_AREA.min.y {
            despawn_upgrade_event.send(DespawnUpgradeEvent(entity));
        }
    }
}

fn despawn_upgrade(
    mut commands: Commands,
    mut despawn_upgrade_event: EventReader<DespawnUpgradeEvent>,
) {
    for despawn_event in despawn_upgrade_event.read() {
        if let Some(mut entity_commands) = commands.get_entity(despawn_event.0) {
            entity_commands.despawn();
        }
    }
}

fn catch_upgrade(
    mut player_sprite: Query<(&Transform, &mut Sprite), With<Player>>,
    upgrades: Query<(Entity, &Transform, &UpgradeComponent)>,
    mut despawn_upgrade_event: EventWriter<DespawnUpgradeEvent>,
    mut upgrade_timer: ResMut<UpgradeTimer>,
) {
    let (player_transform, mut player_sprite) = player_sprite.single_mut();
    let player_size = player_sprite.custom_size.unwrap();
    for (entity, upgrade_transform, upgrade) in &upgrades {
        let player_rect = Rect::from_center_size(player_transform.translation.xy(), player_size);
        let upgrade_rect = Rect::from_center_size(upgrade_transform.translation.xy(), UPGRADE_SIZE);

        if !player_rect.intersect(upgrade_rect).is_empty() {
            despawn_upgrade_event.send(DespawnUpgradeEvent(entity));
            upgrade_timer.timer.reset();
            upgrade_timer.timer.unpause();

            match upgrade {
                UpgradeComponent::Grow if player_size != PLAYER_GROW_SIZE => {
                    player_sprite.custom_size = Some(PLAYER_GROW_SIZE)
                }
                UpgradeComponent::Shrink if player_size != PLAYER_SHRINK_SIZE => {
                    player_sprite.custom_size = Some(PLAYER_SHRINK_SIZE)
                }
                _ => (),
            };
        }
    }
}

fn tick_upgrade_timer(
    time: Res<Time>,
    mut upgrade_timer: ResMut<UpgradeTimer>,
    mut player: Query<&mut Sprite, With<Player>>,
) {
    let dt = time.delta();
    let timer = &mut upgrade_timer.timer;
    timer.tick(dt);

    if timer.just_finished() {
        timer.reset();
        timer.pause();

        let mut player_sprit = player.single_mut();
        player_sprit.custom_size = Some(PLAYER_SIZE);
    }
}

fn handle_player_input(
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut player: Query<(&mut Transform, &Sprite), With<Player>>,
    mut next_in_game_state: ResMut<NextState<InGameState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    let (mut player_transform, player_sprite) = player.single_mut();
    let player_size = player_sprite.custom_size.unwrap();
    let dt = time.delta().as_secs_f32();
    if input.pressed(KeyCode::ArrowLeft) {
        player_transform.translation.x -= PLAYER_SPEED * dt;
    }

    if input.pressed(KeyCode::ArrowRight) {
        player_transform.translation.x += PLAYER_SPEED * dt;
    }

    if player_transform.translation.x - player_size.x / 2.0 < GAME_AREA.min.x {
        player_transform.translation.x = GAME_AREA.min.x + player_size.x / 2.0;
    }

    if player_transform.translation.x + player_size.x / 2.0 > GAME_AREA.max.x {
        player_transform.translation.x = GAME_AREA.max.x - player_size.x / 2.0;
    }

    if input.just_pressed(KeyCode::Escape) {
        next_in_game_state.set(InGameState::Paused);
        next_game_state.set(GameState::InMenu);
    }
}

fn handle_pause_input(
    input: Res<ButtonInput<KeyCode>>,
    mut next_in_game_state: ResMut<NextState<InGameState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    if input.just_pressed(KeyCode::Space) {
        next_in_game_state.set(InGameState::Playing);
    }

    if input.just_pressed(KeyCode::Escape) {
        next_game_state.set(GameState::InMenu);
    }
}

fn game_over(
    mut next_in_game_state: ResMut<NextState<InGameState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut menu_state: ResMut<MenuState>,
) {
    next_in_game_state.set(InGameState::Paused);
    next_game_state.set(GameState::InMenu);

    menu_state.update(0, GAME_OVER_ITEMS[0], Some(MenuNode::GameOver));
}

fn game_won(
    mut next_in_game_state: ResMut<NextState<InGameState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut menu_state: ResMut<MenuState>,
    last_level_played: Res<LastLevelPlayed>,
    mut last_level_complete_text: Query<&mut Visibility, With<LastLevelCompleteText>>,
) {
    next_in_game_state.set(InGameState::Paused);
    next_game_state.set(GameState::InMenu);

    menu_state.update(0, GAME_WON_ITEMS[0], Some(MenuNode::GameWon));

    if last_level_played.0 == TOTAL_LEVELS {
        let mut text_visibility = last_level_complete_text.single_mut();
        *text_visibility = Visibility::Inherited;
    }
}

// Menu

const MENU_ITEMS: [&'static str; 3] = ["Play", "Load level", "Exit"];
const LEVEL_SELECT_ITEMS: [&'static str; 6] = [
    "Level 1", "Level 2", "Level 3", "Level 4", "Level 5", "Back",
];
const GAME_OVER_ITEMS: [&'static str; 2] = ["Retry", "Main Menu"];
const GAME_WON_ITEMS: [&'static str; 2] = ["Next Level", "Main Menu"];

const ITEM_BACKGROUND_SELECTED: Color = Color::rgb(0.3, 0.3, 0.3);
const ITEM_BACKGROUND_NORMAL: Color = Color::BLACK;

#[derive(Resource)]
struct MenuState {
    item_index: usize,

    current_value: String,
    old_value: String,
    menu_node: MenuNode,

    changed: bool,
}

impl MenuState {
    fn new(first_value: &str) -> Self {
        Self {
            item_index: 0,
            current_value: first_value.to_string(),
            old_value: first_value.to_string(),
            menu_node: MenuNode::MainMenu,
            changed: true,
        }
    }

    fn update(&mut self, new_index: usize, new_value: &str, new_menu_node: Option<MenuNode>) {
        self.item_index = new_index;
        self.old_value = self.current_value.clone();
        self.current_value = new_value.to_string();
        if let Some(new_menu_node) = new_menu_node {
            self.menu_node = new_menu_node;
        }
        self.changed = true;
    }

    fn get_menu_items(&self) -> &'static [&'static str] {
        match self.menu_node {
            MenuNode::MainMenu => &MENU_ITEMS,
            MenuNode::LevelSelection => &LEVEL_SELECT_ITEMS,
            MenuNode::GameOver => &GAME_OVER_ITEMS,
            MenuNode::GameWon => &GAME_WON_ITEMS,
        }
    }
}

#[derive(Component, Clone, Copy, PartialEq, Eq)]
enum MenuNode {
    MainMenu,
    LevelSelection,
    GameOver,
    GameWon,
}

#[derive(Component)]
struct MainMenuNode;

#[derive(Component)]
struct MainMenuItems;

#[derive(Component)]
struct LevelSelectItems;

#[derive(Component)]
struct GameOverItems;

#[derive(Component)]
struct MenuItemComponent;

fn load_menu(mut commands: Commands) {
    let main_node = commands
        .spawn((
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,

                    left: Val::Px(GAME_AREA.min.x + WINDOW_RESOLUTION[0] / 2.0),
                    bottom: Val::Px(GAME_AREA.max.y + WINDOW_RESOLUTION[1] / 2.0),
                    width: Val::Px(GAME_AREA.width()),
                    height: Val::Px(GAME_AREA.height()),

                    ..Default::default()
                },
                ..Default::default()
            },
            MainMenuNode,
        ))
        .id();

    let main_menu_node = spawn_menu_node(&mut commands, MenuNode::MainMenu);
    load_menu_items(&mut commands, main_menu_node, &MENU_ITEMS);

    let level_select_node = spawn_menu_node(&mut commands, MenuNode::LevelSelection);
    load_menu_items(&mut commands, level_select_node, &LEVEL_SELECT_ITEMS);

    let game_over_node = spawn_menu_node(&mut commands, MenuNode::GameOver);
    load_menu_items(&mut commands, game_over_node, &GAME_OVER_ITEMS);

    let game_won_node = spawn_menu_node(&mut commands, MenuNode::GameWon);
    load_menu_items(&mut commands, game_won_node, &GAME_WON_ITEMS);

    commands
        .entity(main_node)
        .add_child(main_menu_node)
        .add_child(level_select_node)
        .add_child(game_over_node)
        .add_child(game_won_node);

    commands.insert_resource(MenuState::new(MENU_ITEMS[0]));
}

fn spawn_menu_node(commands: &mut Commands, menu_node: MenuNode) -> Entity {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    display: Display::None,
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..Default::default()
                },
                ..Default::default()
            },
            menu_node,
        ))
        .id()
}

fn load_menu_items(commands: &mut Commands, parent_node: Entity, items_to_load: &[&str]) {
    commands.entity(parent_node).with_children(|builder| {
        for menu_item in items_to_load {
            builder
                .spawn(
                    TextBundle::from_section(
                        *menu_item,
                        TextStyle {
                            font_size: 30.0,
                            color: Color::GREEN,
                            ..Default::default()
                        },
                    )
                    .with_background_color(ITEM_BACKGROUND_NORMAL),
                )
                .insert(MenuItemComponent);
        }
    });
}

fn cleanup_menu(
    mut commands: Commands,
    menu_items: Query<Entity, With<MenuItemComponent>>,
    menu_nodes: Query<
        Entity,
        Or<(
            With<MainMenuNode>,
            With<MainMenuItems>,
            With<LevelSelectItems>,
        )>,
    >,
) {
    commands.remove_resource::<MenuState>();

    for menu_item in &menu_items {
        commands.entity(menu_item).despawn();
    }

    for note_entity in &menu_nodes {
        commands.entity(note_entity).despawn();
    }
}

fn show_menu(
    mut menu: Query<&mut Visibility, With<MainMenuNode>>,
    mut menu_items: Query<(&mut BackgroundColor, &Text), With<MenuItemComponent>>,
    mut selected_menu_item: ResMut<MenuState>,
) {
    let mut menu_visibility = menu.single_mut();
    *menu_visibility = Visibility::Inherited;

    selected_menu_item.item_index = 0;

    for (mut menu_item, text) in &mut menu_items {
        if text.sections[0].value == selected_menu_item.current_value {
            *menu_item = BackgroundColor(ITEM_BACKGROUND_SELECTED);
        } else {
            *menu_item = BackgroundColor(ITEM_BACKGROUND_NORMAL);
        }
    }
}

fn hide_menu(mut menu: Query<&mut Visibility, With<MainMenuNode>>) {
    let mut menu_visibility = menu.single_mut();
    *menu_visibility = Visibility::Hidden;
}

fn handle_menu_navigation_input(
    input: Res<ButtonInput<KeyCode>>,
    mut menu_state: ResMut<MenuState>,
) {
    let items = menu_state.get_menu_items();
    if input.just_pressed(KeyCode::ArrowUp) {
        let new_index = if menu_state.item_index == 0 {
            items.len() - 1
        } else {
            menu_state.item_index - 1
        };
        menu_state.update(new_index, items[new_index], None);
    }

    if input.just_pressed(KeyCode::ArrowDown) {
        let new_index = if menu_state.item_index == items.len() - 1 {
            0
        } else {
            menu_state.item_index + 1
        };
        menu_state.update(new_index, items[new_index], None);
    }
}

fn update_menu(
    mut menu_state: ResMut<MenuState>,
    mut menu_nodes: Query<(&mut Style, &MenuNode)>,
    mut menu_items: Query<(&mut BackgroundColor, &Text), With<MenuItemComponent>>,
) {
    if menu_state.changed {
        for (mut background, text) in &mut menu_items {
            if text.sections[0].value == menu_state.old_value {
                *background = BackgroundColor(ITEM_BACKGROUND_NORMAL);
            }

            if text.sections[0].value == menu_state.current_value {
                *background = BackgroundColor(ITEM_BACKGROUND_SELECTED);
            }
        }

        menu_state.old_value = menu_state.current_value.clone();

        for (mut style, menu_node) in &mut menu_nodes {
            if menu_state.menu_node == *menu_node {
                style.display = Display::default();
            } else {
                style.display = Display::None;
            }
        }

        menu_state.changed = false;
    }
}

fn handle_menu_select_input(
    input: Res<ButtonInput<KeyCode>>,

    mut menu_state: ResMut<MenuState>,

    level_loaded: Res<LevelLoaded>,
    mut load_level_event: EventWriter<LoadLevelEvent>,
    last_level_played: Res<LastLevelPlayed>,

    mut next_state: ResMut<NextState<CurrentGame>>,
    mut next_game_state: ResMut<NextState<GameState>>,

    mut last_level_complete_text: Query<&mut Visibility, With<LastLevelCompleteText>>,
) {
    if menu_state.menu_node == MenuNode::LevelSelection && input.just_pressed(KeyCode::Escape) {
        menu_state.update(0, MENU_ITEMS[0], Some(MenuNode::MainMenu));
        return;
    }

    if !input.just_pressed(KeyCode::Space) {
        return;
    }

    match menu_state.menu_node {
        MenuNode::MainMenu => handle_main_menu_select_input(
            &mut menu_state,
            level_loaded.0,
            &mut load_level_event,
            &mut next_state,
            &mut next_game_state,
        ),
        MenuNode::LevelSelection => handle_level_selection_menu_select_input(
            &mut menu_state,
            &mut load_level_event,
            &mut next_game_state,
        ),
        MenuNode::GameOver => handle_game_over_menu_select_input(
            &mut menu_state,
            &mut load_level_event,
            last_level_played.0,
            &mut next_game_state,
        ),
        MenuNode::GameWon => {
            handle_game_won_menu_select_input(
                &mut menu_state,
                &mut load_level_event,
                last_level_played.0,
                &mut next_game_state,
            );
            let mut text_visibility = last_level_complete_text.single_mut();
            *text_visibility = Visibility::Hidden;
        }
    }
}

fn handle_main_menu_select_input(
    menu_state: &mut MenuState,
    level_loaded: bool,
    load_level_event: &mut EventWriter<LoadLevelEvent>,
    next_state: &mut NextState<CurrentGame>,
    next_game_state: &mut NextState<GameState>,
) {
    match menu_state.current_value.as_str() {
        "Play" => {
            if !level_loaded {
                load_level_event.send(LoadLevelEvent(1));
            }

            next_game_state.set(GameState::InGame);
        }

        "Load level" => {
            menu_state.update(0, LEVEL_SELECT_ITEMS[0], Some(MenuNode::LevelSelection));
        }

        "Exit" => {
            next_game_state.set(GameState::Exited);
            next_state.set(CurrentGame::InMainMenu);
        }
        _ => (),
    }
}

fn handle_level_selection_menu_select_input(
    menu_state: &mut MenuState,
    load_level_event: &mut EventWriter<LoadLevelEvent>,
    next_game_state: &mut NextState<GameState>,
) {
    let mut should_swap_children = false;

    let selection = menu_state.current_value.clone();

    if selection.starts_with("Level") {
        for i in 1..(LEVEL_SELECT_ITEMS.len()) {
            if selection.ends_with(&format!("{i}")) {
                load_level_event.send(LoadLevelEvent(i));
                next_game_state.set(GameState::InGame);
                should_swap_children = true;
                break;
            }
        }
    }

    if selection == "Back" {
        should_swap_children = true;
    }

    if should_swap_children {
        menu_state.update(0, MENU_ITEMS[0], Some(MenuNode::MainMenu));
    }
}

fn handle_game_over_menu_select_input(
    menu_state: &mut MenuState,
    load_level_event: &mut EventWriter<LoadLevelEvent>,
    last_level_played: usize,
    next_game_state: &mut NextState<GameState>,
) {
    match menu_state.current_value.as_str() {
        "Retry" => {
            let last_level_played = last_level_played;
            load_level_event.send(LoadLevelEvent(last_level_played));
            next_game_state.set(GameState::InGame);
            menu_state.update(0, MENU_ITEMS[0], Some(MenuNode::MainMenu));
        }
        "Main Menu" => {
            menu_state.update(0, MENU_ITEMS[0], Some(MenuNode::MainMenu));
        }
        _ => (),
    }
}

fn handle_game_won_menu_select_input(
    menu_state: &mut MenuState,
    load_level_event: &mut EventWriter<LoadLevelEvent>,
    last_level_played: usize,
    next_game_state: &mut NextState<GameState>,
) {
    match menu_state.current_value.as_str() {
        "Next Level" => {
            if last_level_played == TOTAL_LEVELS {
                menu_state.update(0, MENU_ITEMS[0], Some(MenuNode::MainMenu));
                return;
            }

            load_level_event.send(LoadLevelEvent(last_level_played + 1));
            next_game_state.set(GameState::InGame);
            menu_state.update(0, MENU_ITEMS[0], Some(MenuNode::MainMenu));
        }
        "Main Menu" => {
            menu_state.update(0, MENU_ITEMS[0], Some(MenuNode::MainMenu));
        }
        _ => (),
    }
}
