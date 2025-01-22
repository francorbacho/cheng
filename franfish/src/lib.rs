mod evaluation;
use evaluation::Evaluation;

mod inspector;
use inspector::DebugInspector;
use inspector::Inspector;
use inspector::NoInspector;

use cheng::Piece;
use cheng::PseudoMoveGenerator;
use cheng::Side;
use cheng::{Board, BorkedBoard};
use cheng::{LegalMove, PseudoMove};

pub fn go_debug(board: &Board, depth: usize) -> LegalMove {
    go_inspect::<DebugInspector>(board, depth)
}

pub fn go(board: &Board, depth: usize) -> LegalMove {
    go_inspect::<NoInspector>(board, depth)
}

fn go_inspect<I: Inspector>(board: &Board, depth: usize) -> LegalMove {
    I::on_start();

    let mut alpha = Evaluation::BLACK_WIN;
    let mut beta = Evaluation::WHITE_WIN;

    let mut best_move = None;
    let mut best_eval = Evaluation::wins(board.turn().opposite());

    for movement in board.moves() {
        I::on_evaluate(&PseudoMove::from(&movement), depth);

        let mut clone = board.clone();
        clone.feed(movement.clone());

        let eval = minimax::<I>(&clone.inner(), depth - 1, alpha, beta);
        if board.turn() == Side::White && best_eval <= eval {
            I::on_new_best_move(&movement, eval);

            alpha = eval;
            best_eval = eval;
            best_move = Some(movement);
        } else if board.turn() == Side::Black && eval <= best_eval {
            I::on_new_best_move(&movement, eval);

            beta = eval;
            best_eval = eval;
            best_move = Some(movement);
        }
    }

    I::on_end();

    best_move.unwrap()
}

fn minimax<I: Inspector>(
    board: &BorkedBoard,
    depth: usize,
    mut alpha: Evaluation,
    mut beta: Evaluation,
) -> Evaluation {
    if depth == 0 {
        return evaluate::<I>(board);
    }

    let moves = PseudoMoveGenerator::new(board);

    if moves.is_empty() {
        return Evaluation::checkmate_in(board.turn.opposite(), depth as u32);
    }

    let mut result = Evaluation::wins(board.turn.opposite());

    for movement in moves {
        I::on_evaluate(&movement, depth);

        let mut clone = board.clone();
        clone.feed_unchecked(&movement);
        if clone.is_borked() {
            continue;
        }
        let eval = minimax::<I>(&clone, depth - 1, alpha, beta);

        if board.turn == Side::White {
            result = Evaluation::max(result, eval);
            alpha = Evaluation::max(alpha, eval);
        } else {
            result = Evaluation::min(result, eval);
            beta = Evaluation::min(beta, eval);
        }

        if beta <= alpha {
            I::on_pruning();
            break;
        }
    }

    result
}

fn evaluate<I: Inspector>(board: &BorkedBoard) -> Evaluation {
    fn piece_value(piece: Piece) -> Evaluation {
        match piece {
            Piece::Pawn => 100,
            Piece::Knight => 300,
            Piece::Bishop => 325,
            Piece::Rook => 500,
            Piece::Queen => 900,
            Piece::King => 1 << 16,
        }
        .into()
    }

    I::on_evaluate_leaf();

    let mut result = Evaluation::default();

    for piece in Piece::iter() {
        let wmask = board.white_side.pieces.piece(piece);
        let bmask = board.black_side.pieces.piece(piece);
        let diff = Evaluation(wmask.count() as i32 - bmask.count() as i32);

        result += diff * piece_value(piece);
    }

    result
}
