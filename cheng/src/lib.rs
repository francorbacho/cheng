#![cfg_attr(feature = "simd", feature(portable_simd))]

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
    board::{Board, BoardMask, BorkedBoard, FENParsingError, GameResult, PseudoMoveGenerator},
    fen::FromIntoFen,
    movement::{Castle, LegalMove, MoveKind, MoveParseError, PseudoMove, SAN},
    pieces::{SidedPiece, Piece},
    side_state::CastlingRights,
    sides::Side,
    square::{prelude, Square},
};

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
