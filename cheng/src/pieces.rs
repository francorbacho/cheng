#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Piece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl Piece {
    #[cfg(not(feature = "simd"))]
    pub const COUNT: usize = 6;

    #[cfg(feature = "simd")]
    pub const COUNT: usize = 8;

    pub fn iter() -> impl Iterator<Item = Piece> {
        [
            Piece::Pawn,
            Piece::Knight,
            Piece::Bishop,
            Piece::Rook,
            Piece::Queen,
            Piece::King,
        ]
        .iter()
        .copied()
    }

    pub fn iter_promotable_pieces() -> impl Iterator<Item = Piece> {
        [Piece::Knight, Piece::Bishop, Piece::Rook, Piece::Queen]
            .iter()
            .copied()
    }
}

impl From<Piece> for usize {
    #[inline]
    fn from(value: Piece) -> Self {
        value as usize
    }
}

impl TryFrom<usize> for Piece {
    type Error = ();

    #[inline]
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        Ok(match value {
            v if v == Self::Pawn as usize => Self::Pawn,
            v if v == Self::Knight as usize => Self::Knight,
            v if v == Self::Bishop as usize => Self::Bishop,
            v if v == Self::Rook as usize => Self::Rook,
            v if v == Self::Queen as usize => Self::Queen,
            v if v == Self::King as usize => Self::King,
            _ => return Err(()),
        })
    }
}

impl TryFrom<char> for Piece {
    type Error = ();

    #[inline]
    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            'p' => Piece::Pawn,
            'n' => Piece::Knight,
            'b' => Piece::Bishop,
            'r' => Piece::Rook,
            'q' => Piece::Queen,
            'k' => Piece::King,
            _ => return Err(()),
        })
    }
}

impl From<Piece> for char {
    fn from(value: Piece) -> Self {
        match value {
            Piece::Pawn => 'p',
            Piece::Knight => 'n',
            Piece::Bishop => 'b',
            Piece::Rook => 'r',
            Piece::Queen => 'q',
            Piece::King => 'k',
        }
    }
}
