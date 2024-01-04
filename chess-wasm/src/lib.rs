use flimsybird::Evaluable;
use js_sys::JsString;
use wasm_bindgen::prelude::*;

use cheng::{Board, GameResult, MoveKind, Piece, PseudoMove, Side, SidedPiece};

static mut BOARD: Option<Board> = None;

fn get_board() -> &'static Board {
    unsafe { BOARD.as_ref() }.expect("BOARD was not initialized")
}

fn get_board_mut() -> &'static mut Board {
    unsafe { BOARD.as_mut() }.expect("BOARD was not initialized")
}

fn side_to_js_string(side: Side) -> JsString {
    JsString::from(match side {
        Side::White => "white",
        Side::Black => "black",
    })
}

#[wasm_bindgen(start)]
pub fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    cheng::init();

    unsafe {
        BOARD = Some(Board::default());
    }
}

#[wasm_bindgen(js_name = "getSideToMove")]
#[must_use]
pub fn get_side_to_move() -> JsString {
    side_to_js_string(get_board().turn)
}

#[wasm_bindgen(getter_with_clone)]
pub struct GameState {
    pub result: String,
    pub winner: Option<String>,
    #[wasm_bindgen(js_name = "kingInCheck")]
    pub king_in_check: bool,
}

#[wasm_bindgen(js_name = "getState")]
#[must_use]
pub fn get_state() -> GameState {
    let board = get_board();
    let result = board.result();
    GameState {
        result: match result {
            Some(GameResult::Checkmate { .. }) => "checkmate".to_string(),
            Some(GameResult::Draw { .. }) => "draw".to_string(),
            None => String::new(),
        },
        winner: result.and_then(|result| {
            if let GameResult::Checkmate { winner } = result {
                Some(format!("{winner:?}"))
            } else {
                None
            }
        }),
        king_in_check: board.side(board.turn).king_in_check,
    }
}

#[wasm_bindgen(js_name = "getPieces")]
#[must_use]
pub fn get_pieces() -> js_sys::Array {
    let board = get_board();
    let result = js_sys::Array::default();

    let side_field = JsString::from("side");
    let piece_field = JsString::from("piece");
    let position_field = JsString::from("position");

    for (piece, square) in board {
        let SidedPiece(side, piece) = piece;

        let piece_field_js_value = match piece {
            Piece::Pawn => JsString::from("pawn"),
            Piece::Knight => JsString::from("knight"),
            Piece::Bishop => JsString::from("bishop"),
            Piece::Rook => JsString::from("rook"),
            Piece::Queen => JsString::from("queen"),
            Piece::King => JsString::from("king"),
        };

        let side_field_js_value = side_to_js_string(side);

        let position_field_js_value = JsString::from(format!("{square:?}"));

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

    pub promotion: Option<String>,
    #[wasm_bindgen(js_name = "moveIsCapture")]
    pub move_is_capture: bool,
    #[wasm_bindgen(js_name = "passedEnPassantPawnSquare")]
    pub passed_en_passant_pawn_square: Option<String>,
    #[wasm_bindgen(js_name = "castleSide")]
    pub castle_side: Option<String>,
    #[wasm_bindgen(js_name = "rookSquareBeforeCastle")]
    pub rook_square_before_castle: Option<String>,
    #[wasm_bindgen(js_name = "rookSquareAfterCastle")]
    pub rook_square_after_castle: Option<String>,
}

#[wasm_bindgen(js_name = "feedMove")]
pub fn feed_move(movement: &JsString) -> Result<MoveFeedback, String> {
    let board = get_board_mut();
    let Some(movement_str) = movement.as_string() else {
        return Err("Argument must be string".to_string());
    };

    let movement: PseudoMove = match movement_str.parse() {
        Ok(movement) => movement,
        Err(e) => return Err(format!("Invalid movement: {e:?}")),
    };

    let moved_piece_is_pawn = board
        .side(board.turn)
        .pieces
        .piece(Piece::Pawn)
        .get(movement.origin);

    let en_passant_square = board.side(board.turn.opposite()).en_passant;
    let passed_en_passant_pawn_square = moved_piece_is_pawn
        .then_some(en_passant_square)
        .filter(|&square| square == Some(movement.destination))
        .flatten()
        .map(|square| format!("{:?}", square.next_rank(board.turn.opposite())));

    let move_is_capture = passed_en_passant_pawn_square.is_some()
        || board
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
        cheng::Castle::move_could_be_castle(board.turn, &movement)
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
        promotion: if let MoveKind::Promote(piece) = movement.kind {
            let mut piece = format!("{piece:?}");
            piece.make_ascii_lowercase();
            Some(piece)
        } else {
            None
        },
        move_is_capture,
        passed_en_passant_pawn_square,
        castle_side,
        rook_square_before_castle,
        rook_square_after_castle,
    };

    board.feed(movement).map_err(|e| format!("{e:?}"))?;

    Ok(move_feedback)
}

#[wasm_bindgen(js_name = "validMoves")]
#[must_use]
pub fn valid_moves() -> js_sys::Array {
    let board = get_board();
    let result = js_sys::Array::default();

    for movement in board.moves() {
        let movement_str = format!("{movement}");
        result.push(&JsString::from(movement_str));
    }

    result
}

#[wasm_bindgen]
#[must_use]
pub fn evaluate() -> i32 {
    let board = get_board_mut();

    Evaluable::evaluate(board).1 .0
}

#[wasm_bindgen(js_name = "flimsybirdRun")]
#[must_use]
pub async fn flimsybird_run() -> Result<String, String> {
    let board = get_board_mut();
    let (Some(best_move), ev) = Evaluable::evaluate(board) else {
        return Err("No move is possible".to_string());
    };

    let nodes = unsafe { flimsybird::EVALUATED_NODES };

    log::debug!("line: {best_move} :: {ev} ({nodes} nodes evaluated)");
    Ok(format!("{best_move}"))
}
