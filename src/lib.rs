use std::{fmt::Display, mem::swap, ops::{Index, IndexMut, Not}};

pub const BOARD_HEIGHT: usize = 8;
pub const BOARD_WIDTH: usize = 8;

#[derive(Debug, Clone, Copy)]
pub struct Square {
    team: Team,
    kind: SquareKind
}

#[derive(Debug, Clone, Copy)]
pub enum Team { White, Black }
#[derive(Debug, Clone, Copy)]
pub enum SquareKind { Peça, Dama }


#[derive(Debug, Clone)]
pub struct Board {
    pub slots: [[Option<Square>; BOARD_WIDTH]; BOARD_HEIGHT],
}

#[derive(Debug, Clone, Copy)]
pub struct Position {
    row: usize,
    col: usize,
}

#[derive(Debug, Clone, Copy)]
pub enum MoveError {
    NoPieceThere,
    IllegalMove,
}

impl Position {
    pub fn new(y: usize, x: usize) -> Position { Self { row: y, col: x } }
}

impl Board {
    /// Assumes the turn is valid. Assuming no errors: moves the piece at position `start` to position `end`.
    /// If the player can move again, this function returns `true`, else `false`.
    pub fn make_move(&mut self, start: Position, end: Position) -> Result<bool, MoveError> {
        todo!()
    }

    pub fn move_is_legal(&self, from: Position, to: Position) -> bool {
        todo!()
    }

    pub fn legal_moves(&self, pos: Position) -> impl Iterator<Item = Position> + '_ {
        (0..BOARD_HEIGHT).map(|y| (0..BOARD_WIDTH).map(move |x| Position::new(y, x)).collect::<Vec<_>>())
            .flatten()
            .filter(move |p| self.move_is_legal(pos, *p))
    }

    pub fn coord_to_idx(&self, c: Position) -> usize { c.col + BOARD_WIDTH * c.row }
    pub fn idx_to_coords(&self, i: usize) -> Position { Position { row: i / BOARD_WIDTH, col: i % BOARD_WIDTH } }
}


impl Default for Board {
    fn default() -> Self {
        let mut out = Board {
            slots: [[None; BOARD_WIDTH]; BOARD_HEIGHT],
        };

        out
    }
}

impl Index<Position> for Board {
    type Output = Option<Square>;

    fn index(&self, index: Position) -> &Self::Output {
        &self.slots[index.row][index.col]
    }
}

impl IndexMut<Position> for Board {
    fn index_mut(&mut self, index: Position) -> &mut Self::Output {
        &mut self.slots[index.row][index.col]
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let letter: char = (self.col as u8 + b'a') as char;
        let number = self.row + 1;
        write!(f, "{letter}{number}")
    }
}
