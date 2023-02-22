#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Side {
    White,
    Black,
}

impl Side {
    #[inline]
    pub fn opposite(self) -> Self {
        match self {
            Self::White => Self::Black,
            Self::Black => Self::White,
        }
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
