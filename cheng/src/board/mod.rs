mod borked;
pub use borked::BorkedBoard;

mod mask;
pub use mask::BoardMask;

mod iterator;

mod movegen;
pub use movegen::MoveGenerator;

mod parsing;
pub use parsing::FENParsingError;

use crate::Side;

#[derive(Clone, Debug)]
pub enum TryFeedError<E> {
    Parsing(E),
    InvalidMove,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GameResult {
    Draw,
    Checkmate { winner: Side },
}
