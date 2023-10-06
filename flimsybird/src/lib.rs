use std::fmt::{self, Display};

use cheng::prelude::*;
use cheng::{Board, PseudoMove, Piece, Side, SidedPiece};

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Evaluation(pub i32);

impl Evaluation {
    pub const BLACK_WIN: Self = Evaluation(std::i32::MIN);
    pub const WHITE_WIN: Self = Evaluation(std::i32::MAX);

    pub fn favors(self, side: Side) -> bool {
        if self.0 < 0 {
            side == Side::Black
        } else {
            side == Side::White
        }
    }

    pub fn is_better_than(self, side: Side, ev2: Self) -> bool {
        if side == Side::White {
            self.0 > ev2.0
        } else {
            self.0 < ev2.0
        }
    }
}

impl Display for Evaluation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Evaluation::WHITE_WIN => writeln!(f, "white has forced win"),
            Evaluation::BLACK_WIN => writeln!(f, "black has forced win"),
            Evaluation(other) => writeln!(f, "{other:+}"),
        }
    }
}

pub trait Evaluable {
    fn evaluate(&mut self) -> (Option<PseudoMove>, Evaluation);
}

impl Evaluable for Board {
    fn evaluate(&mut self) -> (Option<PseudoMove>, Evaluation) {
        let max_depth = 3;
        board_rec_evaluate(self, max_depth)
    }
}

fn board_rec_evaluate(board: &mut Board, depth: u8) -> (Option<PseudoMove>, Evaluation) {
    let mut best_evaluation = if let Side::White = board.turn { Evaluation::BLACK_WIN } else { Evaluation::WHITE_WIN };
    let mut best_move = None;
    for movement in board.moves() {
        let mut board_clone = board.clone();
        board_clone.feed(movement.clone()).unwrap();
        let new_evaluation = if depth == 0 {
            board_evaluate(&mut board_clone)
        } else {
            board_rec_evaluate(&mut board_clone, depth - 1).1
        };

        if new_evaluation.is_better_than(board.turn, best_evaluation) {
            best_move = Some(movement);
            best_evaluation = new_evaluation;
        }
    }

    (best_move, best_evaluation)
}

fn board_evaluate(board: &Board) -> Evaluation {
    let mut result = 0;
    for (SidedPiece(side, piece), _) in board.into_iter() {
        let side_factor = if side == Side::Black { -1 } else { 1 };
        let piece_value = match piece {
            Piece::Pawn => 100,
            Piece::Knight => 300,
            Piece::Bishop => 325,
            Piece::Rook => 500,
            Piece::Queen => 900,
            Piece::King => 0,
        };

        result += side_factor * piece_value;
    }

    Evaluation(result)
}
