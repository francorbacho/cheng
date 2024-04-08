use cheng::Piece;

pub const DEPTH: u8 = 4;
pub const QUIESCENSE_DEPTH: u8 = 2;

pub const fn piece_value(piece: Piece) -> i32 {
    match piece {
        Piece::Pawn => 100,
        Piece::Knight => 300,
        Piece::Bishop => 325,
        Piece::Rook => 500,
        Piece::Queen => 900,
        Piece::King => 0,
    }
}
pub const KING_SHIELD: i32 = 65;
pub const ADVANCE_PAWN_GAIN: i32 = 10;

pub const MAX_GAIN_DIFF_MOVES: i32 = 100;
pub const MOVE_DIFF_WEIGHT: i32 = 5;

pub const QUEEN_EARLY_DEVELOPMENT: i32 = -80;
