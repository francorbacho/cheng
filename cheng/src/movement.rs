use std::str::FromStr;

use crate::{pieces::Piece, square::Square};

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
    ShortCastle,
    LongCastle,
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
