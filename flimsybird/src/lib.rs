mod params;
mod static_evaluation_tracer;
pub use static_evaluation_tracer::{LogTracer, NoopTracer, StaticEvaluationTracer, UciTracer};

use rand::Rng;

use std::convert::TryFrom;
use std::fmt::{self, Display};

use cheng::{
    prelude as sq, Board, BorkedBoard, GameResult, LegalMove, Piece, PseudoMoveGenerator, Side,
    SidedPiece,
};

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Evaluation(pub i32);

pub static mut EVALUATED_NODES: usize = 0;

impl Evaluation {
    pub const BLACK_WIN: Self = Evaluation(std::i32::MIN);
    pub const WHITE_WIN: Self = Evaluation(std::i32::MAX);
    pub const DRAW: Self = Evaluation(0);

    const CHECKMATE_NET_SIZE: u32 = 10;

    pub fn winner(side: Side) -> Self {
        match side {
            Side::White => Evaluation::WHITE_WIN,
            Side::Black => Evaluation::BLACK_WIN,
        }
    }

    pub fn checkmate_in(side: Side, depth: u32) -> Self {
        assert!(depth < Self::CHECKMATE_NET_SIZE);

        let mut result = Self::winner(side);
        result.0 -= result.0.signum() * depth as i32;

        result
    }

    pub fn is_better_than(self, side: Side, ev2: Self) -> bool {
        if side == Side::White {
            self.0 > ev2.0
        } else {
            self.0 < ev2.0
        }
    }

    pub fn push(&mut self) {
        if self.is_forced_checkmate() {
            self.0 -= self.0.signum();
        }
    }

    pub fn is_forced_checkmate(self) -> bool {
        self.0.abs_diff(Self::WHITE_WIN.0) < Self::CHECKMATE_NET_SIZE
            || self.0.abs_diff(Self::BLACK_WIN.0) < Self::CHECKMATE_NET_SIZE
    }

    pub fn checkmate_depth(self) -> Option<u32> {
        if self.is_forced_checkmate() {
            let wd = self.0.abs_diff(Self::WHITE_WIN.0);
            let bd = self.0.abs_diff(Self::BLACK_WIN.0);
            return Some(wd.min(bd));
        }

        None
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
        if let Some(depth) = self.checkmate_depth() {
            let side = if self.0 > 0 { "white" } else { "black" };
            writeln!(f, "{side} has forced win in {depth}")
        } else {
            writeln!(f, "{:+}", self.0)
        }
    }
}

pub trait Evaluable {
    fn evaluate(&mut self) -> (Option<LegalMove>, Evaluation);
}

impl Evaluable for Board {
    fn evaluate(&mut self) -> (Option<LegalMove>, Evaluation) {
        unsafe { EVALUATED_NODES = 0 }

        let max_depth = 4;
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
        return (None, board_static_evaluation::<NoopTracer>(&board));
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
            if board_clone.is_borked() {
                continue;
            }

            let new_ev = board_rec_evaluate(&board_clone, depth - 1, alpha, beta).1;
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
            if board_clone.is_borked() {
                continue;
            }

            let new_ev = board_rec_evaluate(&board_clone, depth - 1, alpha, beta).1;
            beta = Evaluation(beta.0.min(new_ev.0));

            if new_ev.is_better_than(board.turn, best_evaluation) || best_move.is_none() {
                best_move = Some(movement);
                best_evaluation = new_ev;
            }
        }
    }

    if let Some(best_move) = best_move {
        let best_move = unsafe { LegalMove::unchecked_new(best_move, board) };
        best_evaluation.push();
        (Some(best_move), best_evaluation)
    } else {
        let board = Board::try_from(board.clone()).unwrap();
        (None, board_static_evaluation::<NoopTracer>(&board))
    }
}

pub fn board_static_evaluation<L>(board: &Board) -> Evaluation
where
    L: StaticEvaluationTracer,
{
    unsafe { EVALUATED_NODES += 1 }

    match board.result() {
        GameResult::Undecided => {}
        GameResult::Draw => return Evaluation::DRAW,
        GameResult::Checkmate { winner } => return Evaluation::winner(winner),
    }

    let bb = board.inner();
    let wk_square = bb
        .side(Side::White)
        .pieces
        .piece(Piece::King)
        .first()
        .unwrap();
    let bk_square = bb
        .side(Side::Black)
        .pieces
        .piece(Piece::King)
        .first()
        .unwrap();

    let wk_shield = wk_square.checked_next_rank(Side::White);
    let bk_shield = bk_square.checked_next_rank(Side::Black);

    let mut result = 0;
    let mut advance_pawn_gain = 0;
    let mut white_material = 0;
    let mut black_material = 0;
    let mut king_shield = 0;

    for (SidedPiece(side, piece), square) in board.inner().into_iter() {
        let piece_value = params::piece_value(piece);
        let side_factor = if side == Side::Black { -1 } else { 1 };

        if side == Side::White {
            white_material += piece_value;
        } else if side == Side::Black {
            black_material += piece_value;
        }

        if Some(square) == wk_shield || Some(square) == bk_shield {
            king_shield += params::KING_SHIELD * side_factor;
        }

        if bb.fullmove_clock > 40 && piece == Piece::Pawn {
            advance_pawn_gain += params::ADVANCE_PAWN_GAIN * square.rank::<i32>();
        }
    }

    L::trace("white material", white_material);
    L::trace("black material", black_material);
    L::trace("advance pawn gain", advance_pawn_gain);
    L::trace("king shield", king_shield);
    result += white_material - black_material + advance_pawn_gain + king_shield;

    let queen_early_development_penalty = if bb.fullmove_clock < 10 {
        let wq_square = bb.side(Side::White).pieces.piece(Piece::Queen).first();
        let bq_square = bb.side(Side::Black).pieces.piece(Piece::Queen).first();
        let mut result = 0;

        if wq_square != Some(sq::D1) {
            result += params::QUEEN_EARLY_DEVELOPMENT;
        }

        if bq_square != Some(sq::D8) {
            result -= params::QUEEN_EARLY_DEVELOPMENT;
        }

        result
    } else {
        0i32
    };

    L::trace("queen early penalty", queen_early_development_penalty);
    result += queen_early_development_penalty;

    let white_moves = PseudoMoveGenerator::new_for_side(board.inner(), Side::White).len() as i32;
    let black_moves = PseudoMoveGenerator::new_for_side(board.inner(), Side::Black).len() as i32;
    let move_diff = white_moves - black_moves;
    let diff_moves_gain = params::MAX_GAIN_DIFF_MOVES.min(params::MOVE_DIFF_WEIGHT * move_diff);
    L::trace("diff moves gain", diff_moves_gain);
    result += diff_moves_gain;

    L::trace("final evaluation", result);
    Evaluation(result)
}
