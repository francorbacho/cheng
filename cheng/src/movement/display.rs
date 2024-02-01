use super::{LegalMove, MoveKind, PseudoMove};
use crate::Board;

use std::fmt::Display;

pub struct SAN<'a>(pub &'a LegalMove<'a>, pub &'a Board);

impl Display for SAN<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let piece = self
            .1
            .inner()
            .side(self.1.turn())
            .pieces
            .find(self.0.origin)
            .unwrap();
        write!(f, "{}", char::from(piece).to_uppercase().to_string())?;
        self.0.fmt(f)
    }
}

impl Display for LegalMove<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        PseudoMove::from(self).fmt(f)
    }
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
