use rand::Rng;

use std::convert::TryFrom;
use std::fmt::{self, Display};

use cheng::{
    Board, BorkedBoard, GameResult, LegalMove, Piece, PseudoMoveGenerator, Side, SidedPiece,
};

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Evaluation(pub i32);

pub static mut EVALUATED_NODES: usize = 0;

impl Evaluation {
    pub const BLACK_WIN: Self = Evaluation(std::i32::MIN);
    pub const WHITE_WIN: Self = Evaluation(std::i32::MAX);
    pub const DRAW: Self = Evaluation(0);

    pub fn winner(side: Side) -> Self {
        match side {
            Side::White => Evaluation::WHITE_WIN,
            Side::Black => Evaluation::BLACK_WIN,
        }
    }

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

    pub fn wins(side: Side) -> Self {
        if let Side::White = side {
            Evaluation::WHITE_WIN
        } else {
            Evaluation::BLACK_WIN
        }
    }

    pub fn worst_evaluation(side: Side) -> Self {
        if let Side::White = side {
            Evaluation::BLACK_WIN
        } else {
            Evaluation::WHITE_WIN
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
    fn evaluate(&mut self) -> (Option<LegalMove>, Evaluation);
}

impl Evaluable for Board {
    fn evaluate(&mut self) -> (Option<LegalMove>, Evaluation) {
        unsafe { EVALUATED_NODES = 0 }

        let max_depth = 3;
        let alpha = Evaluation::BLACK_WIN;
        let beta = Evaluation::WHITE_WIN;
        board_rec_evaluate(self.inner(), max_depth, alpha, beta)
    }
}

fn board_rec_evaluate(
    board: &BorkedBoard,
    depth: u8,
    mut alpha: Evaluation,
    mut beta: Evaluation,
) -> (Option<LegalMove>, Evaluation) {
    if depth == 0 {
        let board = Board::try_from(board.clone()).unwrap();
        return (None, board_static_evaluation(&board));
    }

    let mut best_evaluation = Evaluation::worst_evaluation(board.turn);
    let mut best_move = None;

    let mut moves = board.moves();
    moves.cached_moves.sort_unstable_by_key(|mv| {
        let noise = rand::thread_rng().gen_range(0..10);
        mv.destination.to_index() + noise
    });

    if board.turn == Side::White {
        for movement in moves {
            let mut board_clone = board.clone();
            board_clone.feed_unchecked(&movement);
            if !board_clone.is_board_valid() {
                continue;
            }

            let new_ev = board_rec_evaluate(&mut board_clone, depth - 1, alpha, beta).1;
            alpha = Evaluation(alpha.0.max(new_ev.0));

            if new_ev.is_better_than(board.turn, best_evaluation) || best_move.is_none() {
                best_move = Some(movement);
                best_evaluation = new_ev;
            }
        }
    } else {
        for movement in moves {
            let mut board_clone = board.clone();
            board_clone.feed_unchecked(&movement);
            if !board_clone.is_board_valid() {
                continue;
            }

            let new_ev = board_rec_evaluate(&mut board_clone, depth - 1, alpha, beta).1;
            beta = Evaluation(beta.0.min(new_ev.0));

            if new_ev.is_better_than(board.turn, best_evaluation) || best_move.is_none() {
                best_move = Some(movement);
                best_evaluation = new_ev;
            }
        }
    }

    if let Some(best_move) = best_move {
        let best_move = unsafe { LegalMove::unchecked_new(best_move, board) };
        (Some(best_move), best_evaluation)
    } else {
        let board = Board::try_from(board.clone()).unwrap();
        (None, board_static_evaluation(&board))
    }
}

fn board_static_evaluation(board: &Board) -> Evaluation {
    unsafe { EVALUATED_NODES += 1 }

    match board.result() {
        GameResult::Undecided => {}
        GameResult::Draw => return Evaluation::DRAW,
        GameResult::Checkmate { winner } => return Evaluation::winner(winner),
    }

    let mut result = 0;
    for (SidedPiece(side, piece), _) in board.inner().into_iter() {
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

    let white_moves = PseudoMoveGenerator::new_for_side(board.inner(), Side::White).len() as i32;
    let black_moves = PseudoMoveGenerator::new_for_side(board.inner(), Side::Black).len() as i32;

    result += 100.min(5 * (white_moves - black_moves));
    Evaluation(result)
}
