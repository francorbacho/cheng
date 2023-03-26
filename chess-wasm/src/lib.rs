use wasm_bindgen::prelude::*;

use cheng::{Board, Piece};

static mut BOARD: Option<Board> = None;

#[wasm_bindgen(start)]
pub fn main() {
    unsafe {
        BOARD = Some(Board::default());
    }
}

#[wasm_bindgen]
pub fn get_pawn_count() -> u32 {
    let board = unsafe { BOARD.as_ref().expect("BOARD was not initialized!") };
    let pawns = board.white_side.pieces.piece(Piece::Pawn);

    pawns.count() as u32
}
