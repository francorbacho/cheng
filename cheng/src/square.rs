use std::{fmt::Debug, str::FromStr};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Square(usize);

impl Square {
    #[inline]
    pub const fn from_index(v: usize) -> Self {
        Self(v)
    }

    #[inline]
    pub const fn from_rank_file(rank: usize, file: usize) -> Self {
        assert!(rank < 8);
        assert!(file < 8);
        Self(rank * 8 + file)
    }

    #[inline]
    pub fn to_index(self) -> usize {
        self.0
    }

    #[inline]
    pub fn rank(self) -> usize {
        self.0 / 8
    }

    #[inline]
    pub fn file(self) -> usize {
        self.0 % 8
    }

    pub fn iter_all() -> SquareIterator {
        SquareIterator::default()
    }
}

#[derive(Debug)]
pub struct SquareParseError;

impl FromStr for Square {
    type Err = SquareParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let file = match chars.next().ok_or(SquareParseError)? {
            file @ 'a'..='h' => file as usize - 'a' as usize,
            _ => return Err(SquareParseError),
        };

        let rank = match chars.next().ok_or(SquareParseError)? {
            rank @ '1'..='8' => rank as usize - '1' as usize,
            _ => return Err(SquareParseError),
        };

        Ok(Square::from_rank_file(rank, file))
    }
}

impl Debug for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let file = char::from(b'a' + self.file() as u8);
        let rank = 1 + self.rank();
        write!(f, "{file}{rank}")
    }
}

pub struct SquareIterator {
    next: Option<Square>,
}

impl Default for SquareIterator {
    fn default() -> Self {
        Self {
            next: Some(Square::from_rank_file(0, 0)),
        }
    }
}

impl SquareIterator {
    pub fn next_non_consuming(&self) -> Option<Square> {
        self.next
    }
}

impl Iterator for SquareIterator {
    type Item = Square;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.next.take()?;
        let index = current.to_index() + 1;
        self.next = if index >= 64 {
            None
        } else {
            Some(Square::from_index(index))
        };
        Some(current)
    }
}

pub mod consts {
    #![allow(unused)]

    use super::Square;

    pub const A1: Square = Square::from_index(0);
    pub const B1: Square = Square::from_index(1);
    pub const C1: Square = Square::from_index(2);
    pub const D1: Square = Square::from_index(3);
    pub const E1: Square = Square::from_index(4);
    pub const F1: Square = Square::from_index(5);
    pub const G1: Square = Square::from_index(6);
    pub const H1: Square = Square::from_index(7);

    pub const A2: Square = Square::from_index(8);
    pub const B2: Square = Square::from_index(9);
    pub const C2: Square = Square::from_index(10);
    pub const D2: Square = Square::from_index(11);
    pub const E2: Square = Square::from_index(12);
    pub const F2: Square = Square::from_index(13);
    pub const G2: Square = Square::from_index(14);
    pub const H2: Square = Square::from_index(15);

    pub const A3: Square = Square::from_index(16);
    pub const B3: Square = Square::from_index(17);
    pub const C3: Square = Square::from_index(18);
    pub const D3: Square = Square::from_index(19);
    pub const E3: Square = Square::from_index(20);
    pub const F3: Square = Square::from_index(21);
    pub const G3: Square = Square::from_index(22);
    pub const H3: Square = Square::from_index(23);

    pub const A4: Square = Square::from_index(24);
    pub const B4: Square = Square::from_index(25);
    pub const C4: Square = Square::from_index(26);
    pub const D4: Square = Square::from_index(27);
    pub const E4: Square = Square::from_index(28);
    pub const F4: Square = Square::from_index(29);
    pub const G4: Square = Square::from_index(30);
    pub const H4: Square = Square::from_index(31);

    pub const A5: Square = Square::from_index(32);
    pub const B5: Square = Square::from_index(33);
    pub const C5: Square = Square::from_index(34);
    pub const D5: Square = Square::from_index(35);
    pub const E5: Square = Square::from_index(36);
    pub const F5: Square = Square::from_index(37);
    pub const G5: Square = Square::from_index(38);
    pub const H5: Square = Square::from_index(39);

    pub const A6: Square = Square::from_index(40);
    pub const B6: Square = Square::from_index(41);
    pub const C6: Square = Square::from_index(42);
    pub const D6: Square = Square::from_index(43);
    pub const E6: Square = Square::from_index(44);
    pub const F6: Square = Square::from_index(45);
    pub const G6: Square = Square::from_index(46);
    pub const H6: Square = Square::from_index(47);

    pub const A7: Square = Square::from_index(48);
    pub const B7: Square = Square::from_index(49);
    pub const C7: Square = Square::from_index(50);
    pub const D7: Square = Square::from_index(51);
    pub const E7: Square = Square::from_index(52);
    pub const F7: Square = Square::from_index(53);
    pub const G7: Square = Square::from_index(54);
    pub const H7: Square = Square::from_index(55);

    pub const A8: Square = Square::from_index(56);
    pub const B8: Square = Square::from_index(57);
    pub const C8: Square = Square::from_index(58);
    pub const D8: Square = Square::from_index(59);
    pub const E8: Square = Square::from_index(60);
    pub const F8: Square = Square::from_index(61);
    pub const G8: Square = Square::from_index(62);
    pub const H8: Square = Square::from_index(63);
}
