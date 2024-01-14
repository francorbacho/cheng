use std::marker::PhantomData;

pub mod parsing;
pub use parsing::MoveParseError;

mod display;

use crate::{board::BoardMask, pieces::Piece, square::Square, BorkedBoard, Side};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PseudoMove {
    pub origin: Square,
    pub destination: Square,
    pub kind: MoveKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LegalMove<'a> {
    pub origin: Square,
    pub destination: Square,
    pub kind: MoveKind,
    _marker: PhantomData<&'a ()>,
}

impl<'a> LegalMove<'a> {
    pub unsafe fn unchecked_new(pseudo_move: PseudoMove, _: &'a BorkedBoard) -> LegalMove<'a> {
        LegalMove {
            origin: pseudo_move.origin,
            destination: pseudo_move.destination,
            kind: pseudo_move.kind,
            _marker: PhantomData::default(),
        }
    }

    pub fn new<'b>(mut pseudo_move: PseudoMove, board: &'b BorkedBoard) -> Option<LegalMove<'a>> {
        let moved_piece_is_king = board
            .side(board.turn)
            .pieces
            .piece(Piece::King)
            .get(pseudo_move.origin);
        if moved_piece_is_king {
            if let Some(c) = Castle::move_could_be_castle(board.turn, &pseudo_move) {
                pseudo_move.kind = MoveKind::Castle(c);
            }
        }

        if board.does_move_bork(pseudo_move.clone()) {
            None
        } else {
            Some(LegalMove {
                origin: pseudo_move.origin,
                destination: pseudo_move.destination,
                kind: pseudo_move.kind,
                _marker: PhantomData::default(),
            })
        }
    }
}

impl From<LegalMove<'_>> for PseudoMove {
    fn from(legalmove: LegalMove<'_>) -> PseudoMove {
        PseudoMove::from(&legalmove)
    }
}

impl From<&LegalMove<'_>> for PseudoMove {
    fn from(legalmove: &LegalMove<'_>) -> PseudoMove {
        PseudoMove {
            origin: legalmove.origin,
            destination: legalmove.destination,
            kind: legalmove.kind.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MoveKind {
    Move,
    Promote(Piece),
    Castle(Castle),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Castle {
    KingSide,
    QueenSide,
}

impl Castle {
    #[must_use]
    pub fn move_could_be_castle(side: Side, movement: &PseudoMove) -> Option<Castle> {
        let origin_matches_castle = movement.origin == Self::king_square_before_castle(side);

        let destination_matches_king_side_castle =
            movement.destination == Self::king_square_after_castle(Castle::KingSide, side);

        let destination_matches_queen_side_castle =
            movement.destination == Self::king_square_after_castle(Castle::QueenSide, side);

        assert!(!destination_matches_king_side_castle || !destination_matches_queen_side_castle);

        if !origin_matches_castle {
            None
        } else if destination_matches_king_side_castle {
            Some(Castle::KingSide)
        } else if destination_matches_queen_side_castle {
            Some(Castle::QueenSide)
        } else {
            None
        }
    }

    #[must_use]
    pub fn rook_position_before_castle(self, side: Side) -> Square {
        use crate::prelude::*;
        match (side, self) {
            (Side::White, Castle::KingSide) => H1,
            (Side::Black, Castle::KingSide) => H8,
            (Side::White, Castle::QueenSide) => A1,
            (Side::Black, Castle::QueenSide) => A8,
        }
    }

    #[must_use]
    pub const fn rook_position_after_castle(self, side: Side) -> Square {
        use crate::prelude::*;
        match (side, self) {
            (Side::White, Castle::KingSide) => F1,
            (Side::Black, Castle::KingSide) => F8,
            (Side::White, Castle::QueenSide) => D1,
            (Side::Black, Castle::QueenSide) => D8,
        }
    }

    #[must_use]
    pub const fn king_square_before_castle(side: Side) -> Square {
        use crate::prelude::*;
        match side {
            Side::White => E1,
            Side::Black => E8,
        }
    }

    #[must_use]
    pub const fn king_square_after_castle(self, side: Side) -> Square {
        use crate::prelude::*;
        match (side, self) {
            (Side::White, Castle::KingSide) => G1,
            (Side::Black, Castle::KingSide) => G8,
            (Side::White, Castle::QueenSide) => C1,
            (Side::Black, Castle::QueenSide) => C8,
        }
    }

    #[must_use]
    pub fn relevant_square_occupancy(self, side: Side) -> BoardMask {
        use crate::prelude::*;
        match (side, self) {
            (side, Castle::KingSide) => self.relevant_square_threats(side),
            (Side::White, Castle::QueenSide) => BoardMask::from([B1, C1, D1]),
            (Side::Black, Castle::QueenSide) => BoardMask::from([B8, C8, D8]),
        }
    }

    #[must_use]
    pub fn relevant_square_threats(self, side: Side) -> BoardMask {
        use crate::prelude::*;
        match (side, self) {
            (Side::White, Castle::KingSide) => BoardMask::from([F1, G1]),
            (Side::Black, Castle::KingSide) => BoardMask::from([F8, G8]),
            (Side::White, Castle::QueenSide) => BoardMask::from([C1, D1]),
            (Side::Black, Castle::QueenSide) => BoardMask::from([C8, D8]),
        }
    }
}
