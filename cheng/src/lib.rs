#![feature(portable_simd)]

mod board;
mod movegen;
mod movement;
mod pieces;
mod side_state;
mod sides;
mod square;

use movegen::{Bishop, PieceExt, Rook};

pub use crate::{
    board::Board,
    movement::PseudoMove,
    pieces::Piece,
    sides::Side,
    square::{consts, Square},
};

pub type SidedPiece = (Side, Piece);

#[cfg(test)]
mod test;

pub fn init() {
    // TODO: Make these automatic.
    Rook::init();
    Bishop::init();
}
