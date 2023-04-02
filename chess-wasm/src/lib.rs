use wasm_bindgen::prelude::*;

use cheng::{Board, Piece, PseudoMove, Side, SidedPiece};

static mut BOARD: Option<Board> = None;

fn get_board() -> &'static Board {
    unsafe { BOARD.as_ref() }.expect("BOARD was not initialized")
}

fn get_board_mut() -> &'static mut Board {
    unsafe { BOARD.as_mut() }.expect("BOARD was not initialized")
}

#[wasm_bindgen(start)]
pub fn main() {
    cheng::init();

    unsafe {
        BOARD = Some(Board::default());
    }
}

#[wasm_bindgen(js_name = getPieces)]
pub fn get_pieces() -> js_sys::Array {
    let board = get_board();
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

#[wasm_bindgen(getter_with_clone)]
pub struct MoveFeedback {
    pub origin: String,
    pub destination: String,
    #[wasm_bindgen(js_name = moveIsCapture)]
    pub move_is_capture: bool,
    #[wasm_bindgen(js_name = "castleSide")]
    pub castle_side: Option<String>,
    #[wasm_bindgen(js_name = "rookSquareBeforeCastle")]
    pub rook_square_before_castle: Option<String>,
    #[wasm_bindgen(js_name = "rookSquareAfterCastle")]
    pub rook_square_after_castle: Option<String>,
}

#[wasm_bindgen(js_name = feedMove)]
pub fn feed_move(movement: js_sys::JsString) -> Result<MoveFeedback, String> {
    let board = get_board_mut();
    let Some(movement_str) = movement.as_string() else {
        return Err("Argument must be string".to_string());
    };

    let movement: PseudoMove = match movement_str.parse() {
        Ok(movement) => movement,
        Err(e) => return Err(format!("Invalid movement: {e:?}")),
    };

    let move_is_capture = board
        .side(board.turn.opposite())
        .occupancy
        .get(movement.destination);

    // TODO: This is code from the feed function. Obviously this is less than ideal.
    // We should be using a different interface other than PseudoMove.
    let moved_piece_is_king = board
        .side(board.turn)
        .pieces
        .piece(Piece::King)
        .get(movement.origin);
    let castle_side = if moved_piece_is_king {
        cheng::Castle::move_could_be_castle(board.turn, movement.clone())
    } else {
        None
    };

    let (castle_side, rook_square_before_castle, rook_square_after_castle) =
        if let Some(castle_side) = castle_side {
            let rook_square_before_castle = castle_side.rook_position_before_castle(board.turn);
            let rook_square_after_castle = castle_side.rook_position_after_castle(board.turn);
            (
                Some(format!("{castle_side:?}")),
                Some(format!("{rook_square_before_castle:?}")),
                Some(format!("{rook_square_after_castle:?}")),
            )
        } else {
            (None, None, None)
        };

    let move_feedback = MoveFeedback {
        origin: format!("{:?}", movement.origin),
        destination: format!("{:?}", movement.destination),
        move_is_capture,
        castle_side,
        rook_square_before_castle,
        rook_square_after_castle,
    };

    board.feed(movement).map_err(|e| format!("{e:?}"))?;

    Ok(move_feedback)
}

#[wasm_bindgen(js_name = validMoves)]
pub fn valid_moves() -> js_sys::Array {
    let board = get_board();
    let result = js_sys::Array::default();

    for movement in board.moves() {
        let movement_str = format!("{movement}");
        result.push(&js_sys::JsString::from(movement_str));
    }

    result
}
