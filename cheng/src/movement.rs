use std::{fmt::Display, str::FromStr};

use crate::{board::BoardMask, pieces::Piece, square::Square, Side};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PseudoMove {
    pub origin: Square,
    pub destination: Square,
    pub kind: MoveKind,
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
    pub fn move_could_be_castle(side: Side, movement: PseudoMove) -> Option<Castle> {
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

    pub fn rook_position_before_castle(self, side: Side) -> Square {
        use crate::consts::*;
        match (side, self) {
            (Side::White, Castle::KingSide) => H1,
            (Side::Black, Castle::KingSide) => H8,
            (Side::White, Castle::QueenSide) => A1,
            (Side::Black, Castle::QueenSide) => A8,
        }
    }

    pub const fn rook_position_after_castle(self, side: Side) -> Square {
        use crate::consts::*;
        match (side, self) {
            (Side::White, Castle::KingSide) => F1,
            (Side::Black, Castle::KingSide) => F8,
            (Side::White, Castle::QueenSide) => D1,
            (Side::Black, Castle::QueenSide) => D8,
        }
    }

    pub const fn king_square_before_castle(side: Side) -> Square {
        use crate::consts::*;
        match side {
            Side::White => E1,
            Side::Black => E8,
        }
    }

    pub const fn king_square_after_castle(self, side: Side) -> Square {
        use crate::consts::*;
        match (side, self) {
            (Side::White, Castle::KingSide) => G1,
            (Side::Black, Castle::KingSide) => G8,
            (Side::White, Castle::QueenSide) => C1,
            (Side::Black, Castle::QueenSide) => C8,
        }
    }

    pub const fn relevant_squares(self, side: Side) -> BoardMask {
        use crate::consts::*;
        match (side, self) {
            (Side::White, Castle::KingSide) => BoardMask::const_from_slice([F1, G1].as_slice()),
            (Side::Black, Castle::KingSide) => BoardMask::const_from_slice([F8, G8].as_slice()),
            (Side::White, Castle::QueenSide) => BoardMask::const_from_slice([C1, D1].as_slice()),
            (Side::Black, Castle::QueenSide) => BoardMask::const_from_slice([C8, D8].as_slice()),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum PseudoMoveParseError {
    TooShort,
    WrongOriginSquare,
    WrongDestinationSquare,
    WrongPiece,
}

impl FromStr for PseudoMove {
    type Err = PseudoMoveParseError;

    /// Parses a move in the format `{origin}{destination}{promotion}`, where `promotion`
    /// is a single character that can be omitted. Do not add `x` to mark whether the
    /// move takes a piece.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < 4 {
            return Err(PseudoMoveParseError::TooShort);
        }

        let origin: Square = s
            .get(0..2)
            .and_then(|sq| sq.parse().ok())
            .ok_or(PseudoMoveParseError::WrongOriginSquare)?;
        let destination = s
            .get(2..4)
            .and_then(|sq| sq.parse().ok())
            .ok_or(PseudoMoveParseError::WrongDestinationSquare)?;

        let kind = if s.ends_with(|chr: char| chr.is_ascii_alphabetic()) {
            MoveKind::Promote(
                s.chars()
                    .last()
                    .unwrap()
                    .try_into()
                    .map_err(|_| PseudoMoveParseError::WrongPiece)?,
            )
        } else {
            MoveKind::Move
        };

        Ok(PseudoMove {
            origin,
            destination,
            kind,
        })
    }
}

impl Display for PseudoMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}{:?}", self.origin, self.destination)
    }
}
