use std::{fmt::Display, mem::swap, ops::{Index, IndexMut, Not}};

pub const BOARD_HEIGHT: usize = 8;
pub const BOARD_WIDTH: usize = 8;

pub struct Slot {
    team: Team,
    kind: SlotKind
}

pub enum Team { White, Black }
pub enum SlotKind { Peça, Dama }


pub struct Board {
    pub slots: [[Option<Slot>; BOARD_WIDTH]; BOARD_HEIGHT],
}

pub struct Position {
    row: usize,
    col: usize,
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let letter: char = (self.col as u8 + b'a') as char;
        let number = self.row + 1;
        write!(f, "{letter}{number}")
    }
}

enum MoveError {
}

impl Board {
    pub fn make_move() -> Result<(), MoveError> {
        todo!()
    }

    pub fn coord_to_idx(&self, c: Position) -> usize { c.col + BOARD_WIDTH * c.row }
    pub fn idx_to_coords(&self, i: usize) -> Position { Position { row: i / BOARD_WIDTH, col: i % BOARD_WIDTH } }
}

impl Position {
    pub fn new(y: usize, x: usize) -> Position { Self { row: y, col: x } }
}
