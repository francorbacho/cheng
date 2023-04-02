use wasm_bindgen::prelude::*;

use cheng::{Board, Piece, Side, SidedPiece};

static mut BOARD: Option<Board> = None;

#[wasm_bindgen(start)]
pub fn main() {
    unsafe {
        BOARD = Some(Board::default());
    }
}

#[wasm_bindgen]
pub fn get_pieces() -> js_sys::Array {
    let board = unsafe { BOARD.as_ref().expect("BOARD was not initialized!") };
    let result = js_sys::Array::default();

    let side_field = js_sys::JsString::from("side");
    let piece_field = js_sys::JsString::from("piece");
    let position_field = js_sys::JsString::from("position");

    for (piece, square) in board {
        let SidedPiece(side, piece) = piece;

        let piece_field_js_value = match piece {
            Piece::Pawn => js_sys::JsString::from("pawn"),
            Piece::Knight => js_sys::JsString::from("knight"),
            Piece::Bishop => js_sys::JsString::from("bishop"),
            Piece::Rook => js_sys::JsString::from("rook"),
            Piece::Queen => js_sys::JsString::from("queen"),
            Piece::King => js_sys::JsString::from("king"),
        };

        let side_field_js_value = match side {
            Side::White => js_sys::JsString::from("white"),
            Side::Black => js_sys::JsString::from("black"),
        };

        let position_field_js_value = js_sys::JsString::from(format!("{square:?}"));

        let js_obj = js_sys::Object::new();

        js_sys::Reflect::set(&js_obj, &side_field, &side_field_js_value).unwrap();
        js_sys::Reflect::set(&js_obj, &piece_field, &piece_field_js_value).unwrap();
        js_sys::Reflect::set(&js_obj, &position_field, &position_field_js_value).unwrap();

        result.push(&js_obj);
    }

    result
}
