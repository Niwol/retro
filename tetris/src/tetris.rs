use std::collections::HashMap;

use bevy::{color::palettes, prelude::*, sprite::Anchor};

use crate::constants::{
    BLOCK_SIZE, GAME_BOTTOM, GAME_LEFT, GAME_RIGHT, GAME_TOP, NB_COLS, NB_ROWS,
};

pub struct TetrisPlugin;

impl Plugin for TetrisPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<LoadGameEvent>()
            .insert_state(AppState::Menu)
            .add_systems(Startup, setup)
            .add_systems(Update, load_game.run_if(on_event::<LoadGameEvent>))
            .add_systems(Update, update_game.run_if(in_state(AppState::Game)))
            .add_systems(Update, draw_grid);
    }
}

#[derive(Event, Default)]
pub struct LoadGameEvent;

#[derive(States, Hash, PartialEq, Eq, Debug, Clone)]
enum AppState {
    Menu,
    Game,
}

#[derive(Resource)]
struct GameGrid {
    grid: [[Option<Entity>; NB_COLS]; NB_ROWS],
}

#[derive(Resource)]
struct GameState {
    score: u32,
    falling: Tetromino,
    next: Handle<Image>,
    tick_timer: Timer,
}

impl GameState {
    fn fall(&mut self, game_grid: &GameGrid) {
        if self.falling.can_fall(game_grid) {
            self.falling.fall();
        }
    }

    fn place(&mut self, game_grid: &mut GameGrid) {
        if self.falling.can_fall(game_grid) {
            return;
        }
    }
}

impl GameGrid {
    fn new() -> Self {
        Self {
            grid: [[None; NB_COLS]; NB_ROWS],
        }
    }

    fn is_occupied(&self, coord: IVec2) -> bool {
        if coord.y < 0 || coord.y >= NB_ROWS as i32 {
            return true;
        }

        if coord.x < 0 || coord.x > NB_COLS as i32 {
            return true;
        }

        self.grid[coord.y as usize][coord.x as usize].is_some()
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

#[derive(Component)]
struct Block;

struct Tetromino {
    blocks: HashMap<Entity, IVec2>,
    position: IVec2,
}

impl Tetromino {
    fn can_fall(&self, game_grid: &GameGrid) -> bool {
        for (_e, coords) in &self.blocks {
            let destination = self.position + coords - IVec2::Y;

            if game_grid.is_occupied(destination) {
                return false;
            }
        }

        true
    }

    fn fall(&mut self) {
        self.position.y -= 1;
    }
}

enum Shape {
    I,
    T,
    O,
    S,
    Z,
    L,
    J,
}

impl Shape {
    fn get_positions(&self) -> [IVec2; 4] {
        match self {
            Shape::I => [
                IVec2 { x: 0, y: 1 },
                IVec2 { x: 1, y: 1 },
                IVec2 { x: 2, y: 1 },
                IVec2 { x: 3, y: 1 },
            ],
            Shape::T => todo!(),
            Shape::O => todo!(),
            Shape::S => todo!(),
            Shape::Z => todo!(),
            Shape::L => todo!(),
            Shape::J => todo!(),
        }
    }
}

fn load_game(
    mut commands: Commands,
    blocks: Query<Entity, With<Block>>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    // Despawn all existing blocks
    for block_entity in &blocks {
        commands.entity(block_entity).despawn();
    }

    // Inserting or overwriting GameGrid
    commands.insert_resource(GameGrid::new());

    // Todo: Randomize
    let first_tetromino = spawn_next_tetromino(&mut commands, Shape::I);

    let game_state = GameState {
        score: 0,
        falling: first_tetromino,
        next: Handle::default(), // Todo: Randomize
        tick_timer: Timer::from_seconds(1.0, TimerMode::Repeating),
    };

    // Inserting or overwriting GameState
    commands.insert_resource(game_state);

    app_state.set(AppState::Game);
}

fn spawn_next_tetromino(commands: &mut Commands, shape: Shape) -> Tetromino {
    let blocks: Vec<Entity> = (0..4)
        .into_iter()
        .map(|_| {
            commands
                .spawn(Sprite {
                    color: palettes::basic::BLUE.into(),
                    custom_size: Some(Vec2::ONE * BLOCK_SIZE - Vec2::ONE),
                    anchor: Anchor::BottomLeft,
                    ..Default::default()
                })
                .insert(Block)
                .id()
        })
        .collect();

    let positions = shape.get_positions();
    Tetromino {
        blocks: blocks
            .into_iter()
            .enumerate()
            .map(|(i, e)| (e, positions[i]))
            .collect(),
        position: IVec2 { x: 4, y: 12 },
    }
}

fn update_game(
    time: Res<Time>,
    mut game_grid: ResMut<GameGrid>,
    mut game_state: ResMut<GameState>,
    mut transforms: Query<&mut Transform, With<Block>>,
) {
    game_state.tick_timer.tick(time.delta());
    if !game_state.tick_timer.just_finished() {
        return;
    }

    game_state.fall(&game_grid);
    game_state.place(&mut game_grid);

    for (block_entity, coord) in &game_state.falling.blocks {
        let mut block_transform = transforms.get_mut(*block_entity).unwrap();

        block_transform.translation = Vec3 {
            x: (game_state.falling.position.x + coord.x) as f32 * BLOCK_SIZE + GAME_LEFT,
            y: (game_state.falling.position.y + coord.y) as f32 * BLOCK_SIZE + GAME_BOTTOM,
            z: 0.0,
        }
    }
}

fn draw_grid(mut gizmos: Gizmos) {
    gizmos
        .grid_2d(
            Isometry2d {
                rotation: Rot2::default(),
                translation: Vec2 {
                    x: (GAME_LEFT + GAME_RIGHT) / 2.0,
                    y: (GAME_BOTTOM + GAME_TOP) / 2.0,
                },
            },
            UVec2 { x: 10, y: 16 },
            Vec2::ONE * BLOCK_SIZE,
            palettes::basic::GRAY,
        )
        .outer_edges();
}
