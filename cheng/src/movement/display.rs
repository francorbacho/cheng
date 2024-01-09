use super::{LegalMove, MoveKind, PseudoMove};

use std::fmt::Display;

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
