#[cfg(feature = "simd")]
use std::simd::{Simd, SimdOrd, SimdPartialEq, SimdUint};

use crate::{board::BoardMask, movegen, Piece, PseudoMove, Side, SidedPiece, Square};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SideState {
    pub side: Side,
    pub occupancy: BoardMask,
    pub pieces: SidePieces,
    pub threats: BoardMask,
    pub pieces_threats: SidePiecesThreats,

    pub king_in_check: bool,
}

impl SideState {
    #[inline]
    pub fn empty(side: Side) -> Self {
        Self {
            side,
            occupancy: BoardMask::default(),
            pieces: SidePieces::default(),
            threats: BoardMask::default(),
            pieces_threats: SidePiecesThreats::default(),
            king_in_check: false,
        }
    }

    pub fn put(&mut self, square: Square, piece: Piece) {
        self.occupancy.set(square);
        self.pieces.piece_mut(piece).set(square);

        // NOTE: Here we don't update threaten pieces.
    }

    pub fn remove(&mut self, square: Square) {
        let piece = match self.pieces.find(square) {
            Some(piece) => piece,
            None => return,
        };
        // .expect("Tried to remove non-existing piece");

        self.pieces.piece_mut(piece).reset(square);
        self.occupancy.reset(square);
    }

    pub fn update(&mut self, movement: PseudoMove) {
        let PseudoMove {
            ref origin,
            ref destination,
            ..
        } = movement;

        assert!(self.occupancy.get(*origin));
        assert!(!self.occupancy.get(*destination));

        self.occupancy.reset(*origin);
        self.occupancy.set(*destination);
        self.pieces.update(movement);
    }

    pub fn update_threats(&mut self, opposite: &SideState) {
        self.pieces_threats.recalculate(
            self.side,
            &self.pieces,
            self.occupancy,
            opposite.occupancy,
        );
        self.threats = self.pieces_threats.threats();
    }

    pub fn update_king_in_check(&mut self, opposite: &SideState) {
        self.king_in_check = self
            .pieces
            .piece(Piece::King)
            .has_coincidences(opposite.threats);
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

    pub fn iter(&self) -> impl Iterator<Item = BoardMask> {
        self.0.into_iter()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct SidePiecesThreats([BoardMask; Piece::COUNT]);

impl SidePiecesThreats {
    fn recalculate(
        &mut self,
        side: Side,
        my_pieces: &SidePieces,
        friendly_occupancy: BoardMask,
        opposite_occupancy: BoardMask,
    ) {
        let (friendly, opposite) = (friendly_occupancy, opposite_occupancy);
        for (threats, (squares, piece)) in
            self.0.iter_mut().zip(my_pieces.iter().zip(Piece::iter()))
        {
            *threats = BoardMask::default();
            for square in squares {
                *threats = threats.intersection(movegen::threats(
                    SidedPiece(side, piece),
                    square,
                    friendly,
                    opposite,
                ));
            }
        }
    }

    fn threats(&self) -> BoardMask {
        self.0
            .iter()
            .copied()
            .reduce(|acc, e| acc.intersection(e))
            .unwrap()
    }
}
