mod borked;
pub use borked::BorkedBoard;

mod mask;
pub use mask::BoardMask;

mod iterator;

mod movegen;
pub use movegen::{MoveGenerator, PseudoMoveGenerator};

mod parsing;
pub use parsing::FENParsingError;

use crate::{FromIntoFen, LegalMove, PseudoMove, Side};

use std::convert::TryFrom;

#[derive(Clone, Debug)]
pub enum TryFeedError<E> {
    Parsing(E),
    InvalidMove,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GameResult {
    Undecided,
    Draw,
    Checkmate { winner: Side },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Board {
    inner: BorkedBoard,
    result: GameResult,
}

impl Default for Board {
    fn default() -> Board {
        Board {
            inner: BorkedBoard::from_fen(Board::DEFAULT_FEN).unwrap(),
            result: GameResult::Undecided,
        }
    }
}

impl TryFrom<BorkedBoard> for Board {
    type Error = ();

    fn try_from(borked: BorkedBoard) -> Result<Board, ()> {
        let result = borked.compute_result();
        Ok(Board {
            inner: borked,
            result,
        })
    }
}

impl Board {
    pub const DEFAULT_FEN: &'static str =
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

    #[inline]
    #[must_use]
    pub fn inner(&self) -> &BorkedBoard {
        &self.inner
    }

    #[inline]
    #[must_use]
    pub fn turn(&self) -> Side {
        self.inner.turn
    }

    #[inline]
    #[must_use]
    pub fn result(&self) -> GameResult {
        self.result
    }

    #[inline]
    pub fn try_feed<M>(&mut self, movement: M) -> Result<(), TryFeedError<M::Error>>
    where
        M: TryInto<PseudoMove>,
    {
        if self.result != GameResult::Undecided {
            return Err(TryFeedError::InvalidMove);
        }

        self.inner.try_feed(movement)?;
        self.result = self.inner.compute_result();
        Ok(())
    }

    #[inline]
    pub fn feed(&mut self, movement: LegalMove) {
        self.inner.feed_unchecked(&movement.into())
    }

    pub fn validate<'b>(&self, pseudomove: PseudoMove) -> Option<LegalMove<'b>> {
        LegalMove::new(pseudomove, &self.inner)
    }

    pub fn moves(&self) -> MoveGenerator {
        MoveGenerator::new(self)
    }
}
