pub mod parsing;
pub use parsing::PseudoMoveParseError;

use std::fmt::Display;

use crate::{board::BoardMask, pieces::Piece, square::Square, Side};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PseudoMove {
    pub origin: Square,
    pub destination: Square,
    pub kind: MoveKind,
}

impl Display for PseudoMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self {
            origin,
            destination,
            ..
        } = self;

        match self.kind {
            MoveKind::Move | MoveKind::Castle(_) => write!(f, "{origin:?}{destination:?}"),
            MoveKind::Promote(piece) => write!(f, "{origin:?}{destination:?}{}", char::from(piece)),
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
