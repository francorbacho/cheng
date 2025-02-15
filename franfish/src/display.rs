use std::fmt::Display;

use crate::GoResult;

impl Display for GoResult<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(movement) = &self.movement {
            write!(f, "bestmove {movement}")
        } else {
            write!(f, "bestmove (none)")
        }
    }
}

