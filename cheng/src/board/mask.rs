use crate::square::Square;
use std::fmt::{Debug, Display};

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub struct BoardMask(u64);

impl BoardMask {
    #[inline]
    pub const fn const_from(value: u64) -> Self {
        Self(value)
    }

    #[inline]
    pub fn get(&self, square: Square) -> bool {
        self.0 & (1 << square.to_index()) != 0
    }

    #[inline]
    pub fn set(&mut self, square: Square) {
        self.0 |= 1 << square.to_index();
    }

    #[inline]
    pub fn reset(&mut self, square: Square) {
        self.0 &= !(1 << square.to_index());
    }

    #[inline]
    pub fn count(&self) -> u32 {
        self.0.count_ones()
    }

    #[inline]
    pub fn opposite(&self) -> BoardMask {
        BoardMask(!self.0)
    }

    #[inline]
    pub fn only(&self, mask: BoardMask) -> BoardMask {
        BoardMask(self.0 & mask.0)
    }

    #[inline]
    pub fn without(&self, mask: BoardMask) -> BoardMask {
        BoardMask(self.0 & !mask.0)
    }

    #[inline]
    pub fn intersection(&self, mask: BoardMask) -> BoardMask {
        BoardMask(self.0 | mask.0)
    }

    pub fn variations(&self) -> usize {
        1 << self.count()
    }

    /// Returns a copy of this mask that unsets as many bits as indicated by
    /// index, generating variations.
    pub fn variation(&self, index: usize) -> BoardMask {
        let mut occupancy = u64::from(*self);
        let nbits = occupancy.count_ones();
        let mut result = 0u64;
        for i in 0..nbits {
            let first_bit_set = occupancy.trailing_zeros();
            occupancy &= !(1u64 << first_bit_set);
            if index & (1 << i) != 0 {
                result |= 1u64 << first_bit_set;
            }
        }
        BoardMask::from(result)
    }
}

impl Debug for BoardMask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BoardMask(0x{:16x?})", self.0)
    }
}

impl Display for BoardMask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let squares = Square::iter_all().collect::<Vec<_>>();
        for rank in squares.chunks(8).rev() {
            for square in rank {
                if self.get(*square) {
                    write!(f, "x")?;
                } else {
                    write!(f, ".")?;
                }
            }

            writeln!(f)?;
        }

        Ok(())
    }
}

impl From<u64> for BoardMask {
    fn from(value: u64) -> Self {
        Self::const_from(value)
    }
}

impl From<BoardMask> for u64 {
    fn from(value: BoardMask) -> Self {
        value.0
    }
}

impl From<Square> for BoardMask {
    fn from(square: Square) -> Self {
        Self::from(1 << square.to_index())
    }
}

impl From<&[Square]> for BoardMask {
    fn from(squares: &[Square]) -> Self {
        let mut mask = BoardMask::default();
        for square in squares {
            mask.set(*square);
        }
        mask
    }
}
