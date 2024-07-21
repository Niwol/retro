use bevy::prelude::*;

use super::game::MoveDirection;

pub const NB_ROWS: usize = 16;
pub const NB_COLS: usize = 10;

#[derive(Resource)]
pub struct GameGrid {
    cells: [[Option<Entity>; NB_COLS]; NB_ROWS],
}

impl GameGrid {
    pub const fn new() -> Self {
        Self {
            cells: [[None; NB_COLS]; NB_ROWS],
        }
    }

    pub fn get(&self, coords: IVec2) -> Option<Entity> {
        if let Some(row) = self.cells.get(coords.y as usize) {
            if let Some(cell) = row.get(coords.x as usize) {
                return *cell;
            }
        }

        None
    }

    pub fn set(&mut self, coords: IVec2, entity: Entity) {
        assert!(self.is_free(coords));

        let x = coords.x as usize;
        let y = coords.y as usize;
        self.cells[y][x] = Some(entity);
    }

    pub fn is_free(&self, coords: IVec2) -> bool {
        GameGrid::in_bounds(coords) && self.get(coords).is_none()
    }

    pub fn in_bounds(coords: IVec2) -> bool {
        coords.x < 0 || coords.x >= NB_COLS as i32 || coords.y < 0 || coords.y >= NB_ROWS as i32
    }

    pub fn can_move_in_direction(&self, coords: IVec2, direction: MoveDirection) -> bool {
        let new_coords = match direction {
            MoveDirection::Up => IVec2 {
                y: coords.y + 1,
                ..coords
            },
            MoveDirection::Down => IVec2 {
                y: coords.y - 1,
                ..coords
            },
            MoveDirection::Left => IVec2 {
                x: coords.x - 1,
                ..coords
            },
            MoveDirection::Right => IVec2 {
                x: coords.x + 1,
                ..coords
            },
        };

        // Special case: the block is still above the grid and needs to fall down
        if direction == MoveDirection::Down && new_coords.y >= NB_ROWS as i32 {
            return true;
        }

        self.is_free(new_coords)
    }
}
