use bevy::state::state::States;

pub mod constants;
pub mod menu;
pub mod tetris;

#[derive(States, PartialEq, Eq, Clone, Debug, Hash, Copy, Default)]
enum TetrisState {
    #[default]
    InMenu,
    InGame,
}
