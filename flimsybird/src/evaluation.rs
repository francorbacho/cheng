use cheng::Side;

use std::fmt::{self, Display};

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Evaluation(pub i32);

impl Evaluation {
    pub const BLACK_WIN: Self = Evaluation(std::i32::MIN);
    pub const WHITE_WIN: Self = Evaluation(std::i32::MAX);
    pub const DRAW: Self = Evaluation(0);

    const CHECKMATE_NET_SIZE: u32 = 10;

    pub fn winner(side: Side) -> Self {
        match side {
            Side::White => Evaluation::WHITE_WIN,
            Side::Black => Evaluation::BLACK_WIN,
        }
    }

    pub fn checkmate_in(side: Side, depth: u32) -> Self {
        assert!(depth < Self::CHECKMATE_NET_SIZE);

        let mut result = Self::winner(side);
        result.0 -= result.0.signum() * depth as i32;

        result
    }

    pub fn is_better_than(self, side: Side, ev2: Self) -> bool {
        if side == Side::White {
            self.0 > ev2.0
        } else {
            self.0 < ev2.0
        }
    }

    pub fn push(&mut self) {
        if self.is_forced_checkmate() {
            self.0 -= self.0.signum();
        }
    }

    pub fn is_forced_checkmate(self) -> bool {
        self.0.abs_diff(Self::WHITE_WIN.0) < Self::CHECKMATE_NET_SIZE
            || self.0.abs_diff(Self::BLACK_WIN.0) < Self::CHECKMATE_NET_SIZE
    }

    pub fn checkmate_depth(self) -> Option<u32> {
        if self.is_forced_checkmate() {
            let wd = self.0.abs_diff(Self::WHITE_WIN.0);
            let bd = self.0.abs_diff(Self::BLACK_WIN.0);
            return Some(wd.min(bd));
        }

        None
    }

    pub fn wins(side: Side) -> Self {
        if let Side::White = side {
            Evaluation::WHITE_WIN
        } else {
            Evaluation::BLACK_WIN
        }
    }

    pub fn worst_evaluation(side: Side) -> Self {
        if let Side::White = side {
            Evaluation::BLACK_WIN
        } else {
            Evaluation::WHITE_WIN
        }
    }
}

impl Display for Evaluation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(depth) = self.checkmate_depth() {
            let side = if self.0 > 0 { "white" } else { "black" };
            writeln!(f, "{side} has forced win in {depth}")
        } else {
            writeln!(f, "{:+}", self.0)
        }
    }
}
