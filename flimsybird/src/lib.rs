mod evaluation;
mod params;
mod static_evaluation_tracer;

pub use evaluation::Evaluation;
pub use static_evaluation_tracer::{LogTracer, NoopTracer, StaticEvaluationTracer, UciTracer};

use std::convert::TryFrom;

use cheng::{
    prelude as sq, Board, BorkedBoard, GameResult, LegalMove, MoveKind, Piece, PseudoMoveGenerator,
    Side, SidedPiece,
};

pub static mut EVALUATED_NODES: usize = 0;
pub trait Evaluable {
    fn evaluate(&mut self) -> (Option<LegalMove>, Evaluation);
}

impl Evaluable for Board {
    fn evaluate(&mut self) -> (Option<LegalMove>, Evaluation) {
        unsafe { EVALUATED_NODES = 0 }

        let max_depth = params::DEPTH;
        let best_i_can_do = Evaluation::winner(self.turn().opposite());
        let best_o_can_do = Evaluation::winner(self.turn());
        board_rec_evaluate(self.inner(), max_depth, best_i_can_do, best_o_can_do)
    }
}

fn board_rec_evaluate(
    board: &BorkedBoard,
    depth: u8,
    mut best_i_can_do: Evaluation,
    best_o_can_do: Evaluation,
) -> (Option<LegalMove>, Evaluation) {
    if depth == 0 {
        let board = Board::try_from(board.clone()).unwrap();
        return (
            None,
            quiescense_search(
                &board,
                best_i_can_do,
                best_o_can_do,
                params::QUIESCENSE_DEPTH,
            ),
        );
    }

    let mut best_move = None;
    let opposite = board.side(board.turn.opposite()).occupancy;
    let opposite_pieces = &board.side(board.turn.opposite()).pieces;

    let mut moves = board.moves();
    moves.cached_moves.sort_unstable_by_key(|mv| {
        let move_is_capture_gain = if opposite.get(mv.destination) {
            params::piece_value(opposite_pieces.find(mv.destination).unwrap())
        } else {
            0
        };
        let movekind_gain = match mv.kind {
            MoveKind::Castle(_) => 50,
            MoveKind::Promote(Piece::Queen) => 150,
            MoveKind::Promote(_) => 120,
            _ => 0,
        };
        -move_is_capture_gain - movekind_gain
    });

    for movement in moves {
        let mut board_clone = board.clone();
        board_clone.feed_unchecked(&movement);
        if board_clone.is_borked() {
            continue;
        }

        let (_, new_ev) = board_rec_evaluate(&board_clone, depth - 1, best_o_can_do, best_i_can_do);

        if new_ev.is_better_than(board.turn, best_o_can_do) {
            break;
        } else if new_ev.is_better_than(board.turn, best_i_can_do) || best_move.is_none() {
            best_move = Some(movement);
            best_i_can_do = new_ev;
        }
    }

    if let Some(best_move) = best_move {
        let best_move = unsafe { LegalMove::unchecked_new(best_move, board) };
        best_i_can_do.push();
        (Some(best_move), best_i_can_do)
    } else {
        // We don't have to do quiescense search here, *we are here because there are no possible moves*.
        let board = Board::try_from(board.clone()).unwrap();
        (None, board_static_evaluation::<NoopTracer>(&board))
    }
}

fn quiescense_search(
    board: &Board,
    mut best_i_can_do: Evaluation,
    best_o_can_do: Evaluation,
    depth: u8,
) -> Evaluation {
    let eval = board_static_evaluation::<NoopTracer>(board);

    if eval.is_better_than(board.turn(), best_o_can_do) {
        return best_o_can_do;
    }

    if eval.is_better_than(board.turn(), best_i_can_do) {
        best_i_can_do = eval;
    }

    if depth == 0 {
        return best_i_can_do;
    }

    let bb = board.inner();
    let opposite = bb.side(bb.turn.opposite()).occupancy;

    for movement in bb.moves() {
        if !opposite.get(movement.destination) {
            continue;
        }
        let mut board_clone = bb.clone();
        board_clone.feed_unchecked(&movement);

        let Ok(board_clone) = Board::try_from(board_clone) else {
            continue;
        };

        let new_ev = quiescense_search(&board_clone, best_o_can_do, best_i_can_do, depth - 1);
        if new_ev.is_better_than(bb.turn, best_o_can_do) {
            return best_o_can_do;
        }

        if new_ev.is_better_than(bb.turn, best_i_can_do) {
            best_i_can_do = new_ev;
        }
    }

    return best_i_can_do;
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
