use std::{fmt::Display, ops::{Index, IndexMut, Not}};

mod textures;
use textures::*;

pub const BOARD_HEIGHT: usize = 8;
pub const BOARD_WIDTH: usize = 8;

#[derive(Debug, Clone, Copy)]
pub struct Square {
    pub team: Team,
    pub kind: SquareKind
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Team { White, Black }
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SquareKind { Peça, Dama }


#[derive(Debug, Clone)]
pub struct Board {
    pub slots: [[Option<Square>; BOARD_WIDTH]; BOARD_HEIGHT],
    back_buffer: Vec<(Position, Option<Square>)>,
}

#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub row: usize,
    pub col: usize,
}

#[derive(Debug, Clone, Copy)]
pub enum MoveError {
    NoPieceThere,
    IllegalMove,
}

impl Position {
    pub fn new(x: usize, y: usize) -> Position { Self { row: y, col: x } }
}

impl Board {
    /// Assumes the turn is valid. Assuming no errors: moves the piece at position `start` to position `end`.
    /// If the player can move again, this function returns `true`, else `false`.
    pub fn make_move(&mut self, start: Position, end: Position) -> Result<bool, MoveError> {
        if let Some(piece) = self[start] {
            let mut can_move_again = false;
            if !self.move_is_legal(start, end) { return Err(MoveError::IllegalMove); }

            // We know it's legal and possible: do it
            self[end] = self[start];
            self[start] = None;

            for i in start.row.min(end.row)..start.row.max(end.row) {
                for j in start.col.min(end.col)..start.col.max(end.col) {
                    let p = Position {row: i, col: j};
                    // If the predicate is false, the `is_lega` function did a whoopsie
                    if self[p].is_some_and(|s| s.team != piece.team) { can_move_again = true; }
                    self[p] = None; // kills you kills you kills you kills you kills you kills you--
                }
            }

            Ok(can_move_again)
        } else {
            Err(MoveError::NoPieceThere)
        }
    }

    pub fn move_is_legal(&self, from: Position, to: Position) -> bool {
        if let Some(piece_to_move) = self[from] {
            todo!("There is so much to write here")

        } else { false }
    }

    pub fn legal_moves(&self, pos: Position) -> impl Iterator<Item = Position> + '_ {
        (0..BOARD_HEIGHT).map(|y| (0..BOARD_WIDTH).map(move |x| Position::new(y, x)).collect::<Vec<_>>())
            .flatten()
            .filter(move |p| self.move_is_legal(pos, *p))
    }

    pub fn iterate(&mut self) -> Vec<(Position, Option<Square>)> {
        self.back_buffer.clear();
        for (i, row) in self.slots.iter().enumerate() {
            for (j, elem) in row.iter().enumerate() {
                self.back_buffer.push((Position { row: i, col: j}, *elem));
            }
        }
        self.back_buffer.clone()
    }
    pub fn coord_to_idx(&self, c: Position) -> usize { c.col + BOARD_WIDTH * c.row }
    pub fn idx_to_coords(&self, i: usize) -> Position { Position { row: i / BOARD_WIDTH, col: i % BOARD_WIDTH } }
}


impl Default for Board {
    fn default() -> Self {
        let mut out = Board {
            slots: [[None; BOARD_WIDTH]; BOARD_HEIGHT],
            back_buffer: vec![],
        };

        for i in 0..3 {
            for j in ((i % 2)..BOARD_WIDTH).step_by(2) {
                out.slots[i][j] = Some(Square { team: Team::White, kind: SquareKind::Peça });
            }
        }
        for i in 5..8 {
            for j in ((i % 2)..BOARD_WIDTH).step_by(2) {
                out.slots[i][j] = Some(Square { team: Team::Black, kind: SquareKind::Peça });
            }
        }

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

impl Display for MoveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MoveError::NoPieceThere => write!(f, "no piece there"),
            MoveError::IllegalMove => write!(f, "move is illegal"),

        }
    }
}

impl Not for Team {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Team::White => Team::Black,
            Team::Black => Team::White,
        }
    }
}
