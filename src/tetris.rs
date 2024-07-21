use bevy::prelude::*;

mod menu;
use menu::MenuPlugin;

mod game;
use game::GamePlugin;

mod grid;

mod tetromino;

use crate::application::CurrentGame;

pub struct TetrisPlugin;

impl Plugin for TetrisPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MenuPlugin)
            .add_plugins(GamePlugin)
            .insert_state(TetrisState::Exited)
            .add_systems(OnEnter(CurrentGame::Tetris), setup_tetris)
            .add_systems(OnExit(CurrentGame::Tetris), close_tetris);
    }
}

#[derive(States, PartialEq, Eq, Hash, Clone, Debug)]
enum TetrisState {
    Exited,
    InMenu,
    InGame,
}

fn setup_tetris(mut _commands: Commands, mut next_state: ResMut<NextState<TetrisState>>) {
    next_state.set(TetrisState::InMenu);
}

fn close_tetris(mut next_state: ResMut<NextState<TetrisState>>) {
    next_state.set(TetrisState::Exited);
}
