#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Side {
    White,
    Black,
}

impl Side {
    #[inline]
    #[must_use]
    pub fn opposite(self) -> Self {
        match self {
            Self::White => Self::Black,
            Self::Black => Self::White,
        }
    }

    pub fn iter() -> impl Iterator<Item = Side> {
        [Side::White, Side::Black].into_iter()
    }
}

impl From<Side> for char {
    fn from(value: Side) -> Self {
        match value {
            Side::White => 'w',
            Side::Black => 'b',
        }
    }
}

impl From<Side> for usize {
    fn from(value: Side) -> usize {
        match value {
            Side::White => 0,
            Side::Black => 1,
        }
    }
}
