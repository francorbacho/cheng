#[cfg(feature = "simd")]
use std::simd::{Simd, SimdOrd, SimdPartialEq, SimdUint};

use crate::{board::BoardMask, movement::PseudoMove, pieces::Piece, square::Square};

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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SideState {
    pub occupancy: BoardMask,
    pub pieces: SidePieces,
}

impl SideState {
    #[inline]
    pub fn empty() -> Self {
        Self {
            occupancy: Default::default(),
            pieces: Default::default(),
        }
    }

    pub fn put(&mut self, square: Square, piece: Piece) {
        self.occupancy.set(square);
        self.pieces.piece_mut(piece).set(square);
    }

    pub fn update(&mut self, movement: PseudoMove) {
        let PseudoMove {
            ref origin,
            ref destination,
            ..
        } = movement;

        assert!(self.occupancy.get(*origin));

        self.occupancy.reset(*origin);
        self.occupancy.set(*destination);
        self.pieces.update(movement);
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SidePieces([BoardMask; Piece::COUNT]);

impl SidePieces {
    #[inline]
    pub fn piece(&self, piece: Piece) -> BoardMask {
        self.0[usize::from(piece)]
    }

    #[inline]
    pub fn piece_mut(&mut self, piece: Piece) -> &mut BoardMask {
        &mut self.0[usize::from(piece)]
    }

    #[cfg(not(feature = "simd"))]
    fn find(&self, square: Square) -> Option<Piece> {
        for (i, mask) in self.0.iter().enumerate() {
            if mask.get(square) {
                return Some(Piece::try_from(i).unwrap());
            }
        }

        None
    }

    #[cfg(feature = "simd")]
    fn find(&self, square: Square) -> Option<Piece> {
        let pieces: [u64; Piece::COUNT] = unsafe { std::mem::transmute(self.0) };
        let pieces_lanes = Simd::from(pieces);
        let search_mask = Simd::splat(BoardMask::from(square).into());

        // This is computed at compile time on release.
        let idxs: [u64; Piece::COUNT] = (0..Piece::COUNT as u64)
            .collect::<Vec<_>>()
            .as_slice()
            .try_into()
            .unwrap();

        let idxs: Simd<u64, { Piece::COUNT }> = Simd::from(idxs);
        let result = (search_mask & pieces_lanes)
            .simd_ne(Simd::splat(0))
            .select(idxs, Simd::splat(0))
            .reduce_max();

        Some(Piece::try_from(result as usize).ok()).flatten()
    }

    pub fn update(&mut self, movement: PseudoMove) {
        use crate::movement::MoveKind;

        let PseudoMove {
            origin,
            destination,
            kind,
            ..
        } = movement;

        match kind {
            MoveKind::Move => {
                let piece = self.find(origin).unwrap();
                let mask = self.piece_mut(piece);
                mask.reset(origin);
                mask.set(destination);
            }
            MoveKind::Promote(piece) => {
                self.piece_mut(Piece::Pawn).reset(origin);
                self.piece_mut(piece).set(destination);
            }
            MoveKind::ShortCastle | MoveKind::LongCastle => {}
        }
    }
}
