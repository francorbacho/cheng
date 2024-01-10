#![feature(portable_simd)]

mod board;
mod fen;
pub mod movegen;
mod movement;
mod pieces;
mod side_state;
mod sides;
mod square;

use movegen::{Bishop, PieceExt, Rook};

pub use crate::{
    board::{Board, BorkedBoard, FENParsingError, GameResult, MoveGenerator},
    fen::FromIntoFen,
    movement::{Castle, LegalMove, MoveKind, MoveParseError, PseudoMove},
    pieces::Piece,
    sides::Side,
    square::{prelude, Square},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SidedPiece(pub Side, pub Piece);

impl From<SidedPiece> for char {
    fn from(SidedPiece(side, piece): SidedPiece) -> Self {
        if side == Side::White {
            char::from(piece).to_ascii_uppercase()
        } else {
            char::from(piece)
        }
    }
}

#[cfg(test)]
mod test;

pub fn init() {
    // TODO: Make these automatic.
    Rook::init();
    Bishop::init();
}

pub mod internal {
    pub use crate::movegen::BISHOP_MOVES;
    pub use crate::movegen::ROOK_MOVES;
}
