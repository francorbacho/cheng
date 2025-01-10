mod inspector;
use inspector::DebugInspector;
use inspector::Inspector;
use inspector::NoInspector;

use cheng::LegalMove;
use cheng::Piece;
use cheng::Side;
use cheng::{Board, BorkedBoard};

pub type Evaluation = i32;

pub fn go_debug(board: &Board, depth: usize) -> LegalMove {
    go_inspect::<DebugInspector>(board, depth)
}

pub fn go(board: &Board, depth: usize) -> LegalMove {
    go_inspect::<NoInspector>(board, depth)
}

fn go_inspect<I: Inspector>(board: &Board, depth: usize) -> LegalMove {
    I::on_start();

    let mut alpha = i32::MIN;
    let mut beta = i32::MAX;

    let mut best_move = None;
    let mut best_eval = if board.turn() == Side::White {
        Evaluation::MIN
    } else {
        Evaluation::MAX
    };

    for movement in board.moves() {
        let mut clone = board.clone();
        clone.feed(movement.clone());

        let eval = minimax::<I>(&clone.inner(), depth, alpha, beta);
        if board.turn() == Side::White && best_eval < eval {
            I::on_new_best_move(&movement);

            alpha = eval;
            best_eval = eval;
            best_move = Some(movement);
        } else if board.turn() == Side::Black && eval < best_eval {
            I::on_new_best_move(&movement);

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
        let evaluation = evaluate::<I>(board);
        let depth_penalty = if board.turn == Side::White {
            depth as i32
        } else {
            -(depth as i32)
        };
        return evaluation + depth_penalty;
    }

    let mut result = if board.turn == Side::White {
        Evaluation::MIN
    } else {
        Evaluation::MAX
    };

    for movement in board.moves() {
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
    }

    I::on_evaluate();

    let mut result = 0i32;

    for piece in Piece::iter() {
        let wmask = board.white_side.pieces.piece(piece);
        let bmask = board.black_side.pieces.piece(piece);

        result += (wmask.count() as i32 - bmask.count() as i32) * piece_value(piece);
    }

    result
}
