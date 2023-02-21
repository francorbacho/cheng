#![feature(portable_simd)]

mod board;
mod movegen;
mod movement;
mod pieces;
mod sides;
mod square;

pub use crate::{
    board::Board,
    movement::PseudoMove,
    pieces::Piece,
    sides::Side,
    square::{consts, Square},
};

#[cfg(test)]
mod test;
