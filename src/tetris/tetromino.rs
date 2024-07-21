use bevy::{math::uvec2, prelude::*};

#[derive(Clone, Copy)]
pub struct Tetromino {
    pub coords: UVec2,
    pub blocks: [Entity; 4],
    pub shape: TetrominoShape,
}

#[derive(Clone, Copy)]
pub enum TetrominoShape {
    I,
    O,
    T,
    S,
    Z,
    J,
    L,
}

impl TetrominoShape {
    pub fn get_local_coords(&self, rotation: Rotation) -> [UVec2; 4] {
        // (0, 3)  (1, 3)  (2, 3)  (3, 3)
        //
        // (0, 2)  (1, 2)  (2, 2)  (3, 2)
        //
        // (0, 1)  (1, 1)  (2, 1)  (3, 1)
        //
        // (0, 0)  (1, 0)  (2, 0)  (3, 0)

        match self {
            TetrominoShape::I => match rotation {
                Rotation::R0 | Rotation::R180 => [(2, 3), (2, 2), (2, 1), (2, 0)],
                Rotation::R90 | Rotation::R270 => [(0, 1), (1, 1), (2, 1), (3, 1)],
            },
            TetrominoShape::O => todo!(),
            TetrominoShape::T => todo!(),
            TetrominoShape::S => todo!(),
            TetrominoShape::Z => todo!(),
            TetrominoShape::J => todo!(),
            TetrominoShape::L => todo!(),
        }
        .map(|(x, y)| uvec2(x as u32, y as u32))
    }

    pub fn center_offset(&self) -> Vec2 {
        match self {
            TetrominoShape::I => Vec2::new(16.0, 0.0),
            TetrominoShape::O => todo!(),
            TetrominoShape::T => todo!(),
            TetrominoShape::S => todo!(),
            TetrominoShape::Z => todo!(),
            TetrominoShape::J => todo!(),
            TetrominoShape::L => todo!(),
        }
    }
}

#[derive(Clone, Copy)]
pub enum Rotation {
    R0,
    R90,
    R180,
    R270,
}

impl Rotation {
    pub fn rotate_left(&self) -> Self {
        match self {
            Self::R0 => Self::R270,
            Self::R90 => Self::R0,
            Self::R180 => Self::R90,
            Self::R270 => Self::R180,
        }
    }

    pub fn rotate_right(&self) -> Self {
        match self {
            Self::R0 => Self::R90,
            Self::R90 => Self::R180,
            Self::R180 => Self::R270,
            Self::R270 => Self::R0,
        }
    }
}
