mod evaluation;
use evaluation::Evaluation;

mod debugger;
pub use debugger::{Debugger, LogAllDebugger, NoDebugger};

use cheng::Piece;
use cheng::PseudoMoveGenerator;
use cheng::Side;
use cheng::{Board, BorkedBoard, GameResult};
use cheng::{LegalMove, PseudoMove};

use std::time::{Duration, Instant};

pub fn go(board: &Board) -> GoResult {
    let mut franfish = Franfish::<NoDebugger>::default();
    franfish.go(board)
}

pub fn go_debug(board: &Board) -> GoResult {
    let mut franfish = Franfish::new(LogAllDebugger::default(), Some(Duration::from_secs(15)));
    franfish.go(board)
}

static mut NODES_VISITED: usize = 0;

const EV_DEPTH: usize = 4;

#[derive(PartialEq, Eq)]
pub enum SearchExit {
    FullDepth,
    Timeout,
}

#[derive(PartialEq, Eq)]
pub struct GoResult<'a> {
    pub exit: SearchExit,
    pub movement: LegalMove<'a>,
}

#[derive(PartialEq, Eq)]
struct SearchResult {
    exit: SearchExit,
    eval: Evaluation,
}

pub struct Franfish<D: Debugger> {
    search_started_at: Option<Instant>,
    max_search_time: Option<Duration>,
    debugger: D,
}

impl<D: Default + Debugger> Default for Franfish<D> {
    fn default() -> Self {
        Self::new(D::default(), Some(Duration::from_secs(2)))
    }
}

impl<D: Debugger> Franfish<D> {
    pub fn new(debugger: D, max_search_time: Option<Duration>) -> Self {
        Self {
            search_started_at: Some(Instant::now()),
            max_search_time,
            debugger,
        }
    }

    fn elapsed(&self) -> Option<Duration> {
        Some(Instant::now() - self.search_started_at?)
    }

    pub fn go<'a>(&mut self, board: &'a Board) -> GoResult<'a> {
        unsafe {
            NODES_VISITED = 0;
        }

        let mut alpha = Evaluation::BLACK_WIN;
        let mut beta = Evaluation::WHITE_WIN;

        let mut best_move = None;
        let mut best_eval = Evaluation::wins(board.turn().opposite());
        let mut exit = SearchExit::FullDepth;

        for movement in board.moves() {
            let pseudomove = PseudoMove::from(movement.clone());
            self.debugger.on_feed(&pseudomove, EV_DEPTH);

            let mut clone = board.inner().clone();
            clone.feed_unchecked(&pseudomove);
            if clone.is_borked() {
                continue;
            }

            let result = self.minimax(&clone, EV_DEPTH - 1, alpha, beta);
            if result.exit == SearchExit::Timeout {
                log::debug!("got timeout :(");
                exit = SearchExit::Timeout;
                break;
            }

            if board.turn() == Side::White && best_eval <= result.eval {
                alpha = result.eval;
                best_eval = result.eval;
                best_move = Some(movement);
            } else if board.turn() == Side::Black && result.eval <= best_eval {
                beta = result.eval;
                best_eval = result.eval;
                best_move = Some(movement);
            }
        }

        log::trace!("visited {} nodes", unsafe { NODES_VISITED });

        GoResult {
            exit,
            movement: board.validate(best_move.unwrap()).unwrap(),
        }
    }

    fn minimax(
        &mut self,
        board: &BorkedBoard,
        depth: usize,
        mut alpha: Evaluation,
        mut beta: Evaluation,
    ) -> SearchResult {
        if depth == 0 {
            let eval = evaluate(board);
            self.debugger.on_leaf(eval);

            return SearchResult {
                exit: SearchExit::FullDepth,
                eval,
            };
        }

        // OPTIMIZATION: We check if it's Some(..) so we don't have to call elapsed.
        if self.max_search_time.is_some() && self.elapsed() > self.max_search_time {
            return SearchResult {
                exit: SearchExit::Timeout,
                eval: evaluate(board),
            };
        }

        let gen = PseudoMoveGenerator::new(board);
        if gen.is_empty() {
            let eval = if board.side(board.turn).king_in_check {
                Evaluation::wins(board.turn.opposite())
            } else {
                Evaluation::DRAW
            };

            self.debugger.on_leaf(eval);

            return SearchResult {
                exit: SearchExit::FullDepth,
                eval,
            };
        }

        let mut best_eval = Evaluation::wins(board.turn.opposite());
        let mut legal_move_exists = false;

        for movement in gen {
            let mut clone = board.clone();
            clone.feed_unchecked(&movement);
            if clone.is_borked() {
                continue;
            }

            legal_move_exists = true;

            self.debugger.on_feed(&movement, depth);

            let result = self.minimax(&clone, depth - 1, alpha, beta);
            if board.turn == Side::White && best_eval <= result.eval {
                alpha = result.eval;
                best_eval = result.eval;
            } else if board.turn == Side::Black && result.eval <= best_eval {
                beta = result.eval;
                best_eval = result.eval;
            }

            // TODO: alpha-beta pruning
        }

        if !legal_move_exists {
            self.debugger.on_leaf(best_eval);
        }

        SearchResult {
            exit: SearchExit::FullDepth,
            eval: best_eval,
        }
    }
}

fn evaluate(board: &BorkedBoard) -> Evaluation {
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

    unsafe {
        NODES_VISITED += 1;
    }

    match board.compute_result() {
        GameResult::Draw => return Evaluation::DRAW,
        GameResult::Checkmate { winner } => return Evaluation::wins(winner),
        GameResult::Undecided => {}
    }

    let mut evaluation = Evaluation::default();

    for piece in Piece::iter() {
        let wmask = board.white_side.pieces.piece(piece);
        let bmask = board.black_side.pieces.piece(piece);
        let diff = Evaluation(wmask.count() as i32 - bmask.count() as i32);

        evaluation += diff * piece_value(piece);
    }

    evaluation
}
