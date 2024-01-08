use super::{LegalMove, MoveKind};
use crate::Square;

use std::convert::TryFrom;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub enum MoveParseError {
    TooShort,
    WrongOriginSquare,
    WrongDestinationSquare,
    WrongPiece,
}

impl TryFrom<&str> for LegalMove {
    type Error = MoveParseError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        LegalMove::from_str(s)
    }
}

impl FromStr for LegalMove {
    type Err = MoveParseError;

    /// Parses a move in the format `{origin}{destination}{promotion}`, where `promotion`
    /// is a single character that can be omitted. Do not add `x` to mark whether the
    /// move takes a piece.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < 4 {
            return Err(MoveParseError::TooShort);
        }

        let origin: Square = s
            .get(0..2)
            .and_then(|sq| sq.parse().ok())
            .ok_or(MoveParseError::WrongOriginSquare)?;
        let destination = s
            .get(2..4)
            .and_then(|sq| sq.parse().ok())
            .ok_or(MoveParseError::WrongDestinationSquare)?;

        let kind = if s.ends_with(|chr: char| chr.is_ascii_alphabetic()) {
            MoveKind::Promote(
                s.chars()
                    .last()
                    .unwrap()
                    .try_into()
                    .map_err(|_| MoveParseError::WrongPiece)?,
            )
        } else {
            MoveKind::Move
        };

        Ok(LegalMove {
            origin,
            destination,
            kind,
        })
    }
}
