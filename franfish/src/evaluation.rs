use cheng::Side;

use std::fmt::{self, Display};
use std::ops;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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

        let Evaluation(mut result) = Self::winner(side);
        result -= result.signum() * depth as i32;

        Evaluation(result)
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
        if side == Side::White {
            Evaluation::WHITE_WIN
        } else {
            Evaluation::BLACK_WIN
        }
    }
}

impl Display for Evaluation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(depth) = self.checkmate_depth() {
            let side = if self.0 > 0 { "white" } else { "black" };
            write!(f, "{side} has forced win in {depth}")
        } else {
            write!(f, "{:+}", self.0)
        }
    }
}

impl From<Evaluation> for i32 {
    fn from(Evaluation(value): Evaluation) -> i32 {
        value
    }
}

impl From<i32> for Evaluation {
    fn from(value: i32) -> Evaluation {
        Evaluation(value)
    }
}

macro_rules! impl_ops {
    ($struct:ty, $trait:path, $method:ident) => {
        impl $trait for $struct {
            type Output = Self;

            fn $method(self, rhs: Self) -> Self::Output {
                Self(self.0.$method(rhs.0))
            }
        }
    };
}

macro_rules! impl_assign_ops {
    ($struct:ty, $trait:path, $method:ident) => {
        impl $trait for $struct {
            fn $method(&mut self, rhs: Self) {
                self.0.$method(rhs.0);
            }
        }
    };
}

impl_ops!(Evaluation, ops::Add, add);
impl_ops!(Evaluation, ops::Sub, sub);
impl_ops!(Evaluation, ops::Mul, mul);
impl_ops!(Evaluation, ops::Div, div);
impl_assign_ops!(Evaluation, ops::AddAssign, add_assign);
impl_assign_ops!(Evaluation, ops::SubAssign, sub_assign);
impl_assign_ops!(Evaluation, ops::MulAssign, mul_assign);
impl_assign_ops!(Evaluation, ops::DivAssign, div_assign);
