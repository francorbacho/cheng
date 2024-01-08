use super::{LegalMove, MoveKind};

use std::fmt::Display;

impl Display for LegalMove {
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
