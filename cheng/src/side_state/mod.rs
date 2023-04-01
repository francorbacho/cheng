pub mod iterator;

#[cfg(feature = "simd")]
use std::simd::{Simd, SimdOrd, SimdPartialEq, SimdUint};

use crate::{
    board::BoardMask,
    movegen,
    movement::{Castle, MoveKind},
    Piece, PseudoMove, Side, SidedPiece, Square,
};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CastlingRights {
    None,
    QueenSide,
    KingSide,
    #[default]
    Both,
}

impl CastlingRights {
    #[inline]
    pub fn checked_add(&mut self, rhs: CastlingRights) -> Result<(), CastlingRights> {
        match (*self, rhs) {
            (lhs, rhs) if lhs == rhs => Err(lhs),
            (CastlingRights::None, rhs) => {
                *self = rhs;
                Ok(())
            }
            (CastlingRights::Both, err) => Err(err),
            (CastlingRights::QueenSide, CastlingRights::KingSide)
            | (CastlingRights::KingSide, CastlingRights::QueenSide) => {
                *self = CastlingRights::Both;
                Ok(())
            }
            _ => unreachable!(),
        }
    }

    #[inline]
    pub fn without(self, rhs: CastlingRights) -> CastlingRights {
        match (self, rhs) {
            (CastlingRights::None, _) => CastlingRights::None,
            (lhs, rhs) if lhs == rhs => CastlingRights::None,
            (CastlingRights::Both, CastlingRights::KingSide) => CastlingRights::QueenSide,
            (CastlingRights::Both, CastlingRights::QueenSide) => CastlingRights::KingSide,
            (CastlingRights::KingSide, CastlingRights::QueenSide) => CastlingRights::KingSide,
            (CastlingRights::QueenSide, CastlingRights::KingSide) => CastlingRights::QueenSide,
            _ => unreachable!(),
        }
    }

    #[inline]
    pub fn queen_side(self) -> bool {
        match self {
            Self::None | Self::KingSide => false,
            Self::QueenSide | Self::Both => true,
        }
    }

    #[inline]
    pub fn king_side(self) -> bool {
        match self {
            Self::None | Self::QueenSide => false,
            Self::KingSide | Self::Both => true,
        }
    }

    pub fn to_fen_str(self) -> &'static str {
        match self {
            Self::None => "",
            Self::QueenSide => "q",
            Self::KingSide => "k",
            Self::Both => "kq",
        }
    }

    /// Parses FEN castling rights for white and black.
    pub fn parse_fen_from_str(
        castling_rights: &str,
    ) -> Result<(CastlingRights, CastlingRights), ()> {
        if castling_rights == "-" {
            return Ok((CastlingRights::None, CastlingRights::None));
        }

        let mut white_cr = CastlingRights::None;
        let mut black_cr = CastlingRights::None;

        for chr in castling_rights.chars() {
            match chr {
                'K' => white_cr
                    .checked_add(CastlingRights::KingSide)
                    .map_err(|_| ())?,
                'Q' => white_cr
                    .checked_add(CastlingRights::QueenSide)
                    .map_err(|_| ())?,
                'k' => black_cr
                    .checked_add(CastlingRights::KingSide)
                    .map_err(|_| ())?,
                'q' => black_cr
                    .checked_add(CastlingRights::QueenSide)
                    .map_err(|_| ())?,
                _ => return Err(()),
            }
        }

        Ok((white_cr, black_cr))
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SideState {
    pub side: Side,
    pub occupancy: BoardMask,
    pub pieces: SidePieces,
    pub threats: BoardMask,
    pub pieces_threats: SidePiecesThreats,
    pub en_passant: Option<Square>,
    pub king_in_check: bool,
    pub castling_rights: CastlingRights,
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
            en_passant: None,
            king_in_check: false,
            castling_rights: CastlingRights::None,
        }
    }

    pub fn put(&mut self, square: Square, piece: Piece) {
        self.occupancy.set(square);
        self.pieces.piece_mut(piece).set(square);

        // NOTE: Here we don't update threaten pieces.
    }

    pub fn remove(&mut self, square: Square) {
        if !self.occupancy.get(square) {
            return;
        }

        let piece = match self.pieces.find(square) {
            Some(piece) => piece,
            None => unreachable!(),
        };

        self.pieces.piece_mut(piece).reset(square);
        self.occupancy.reset(square);

        if piece != Piece::Rook {
            return;
        }

        if square == Castle::KingSide.rook_position_before_castle(self.side) {
            self.castling_rights = self.castling_rights.without(CastlingRights::KingSide);
        } else if square == Castle::QueenSide.rook_position_before_castle(self.side) {
            self.castling_rights = self.castling_rights.without(CastlingRights::QueenSide);
        }
    }

    fn is_two_square_pawn_move(&self, movement: PseudoMove) -> bool {
        self.pieces.piece(Piece::Pawn).get(movement.origin)
            && (movement.destination.rank() as i32 - movement.origin.rank() as i32).abs() == 2
    }

    pub fn update(&mut self, movement: PseudoMove) {
        // NOTE: This only updates the state, and assumes the move is valid.
        let PseudoMove {
            ref origin,
            ref destination,
            ..
        } = movement;

        assert!(self.occupancy.get(*origin));
        assert!(!self.occupancy.get(*destination));

        if self.is_two_square_pawn_move(movement.clone()) {
            self.en_passant = Some(origin.next_rank(self.side));
        } else {
            self.en_passant = None;
        }

        self.occupancy.reset(*origin);
        self.occupancy.set(*destination);

        if let MoveKind::Castle(castle) = movement.kind {
            self.castling_rights = CastlingRights::None;

            self.occupancy
                .reset(castle.rook_position_before_castle(self.side));
            self.occupancy
                .set(castle.rook_position_after_castle(self.side));
        } else if let MoveKind::Move = movement.kind {
            self.update_castling_rights(movement.clone());
        }

        self.pieces.update(self.side, movement);
    }

    fn update_castling_rights(&mut self, movement: PseudoMove) {
        let PseudoMove { origin, .. } = movement;

        let is_king_move = self.pieces.piece(Piece::King).get(origin);
        if is_king_move {
            self.castling_rights = CastlingRights::None;
            return;
        }

        let is_rook_move = self.pieces.piece(Piece::Rook).get(origin);
        if !is_rook_move {
            return;
        }

        if origin == Castle::QueenSide.rook_position_before_castle(self.side) {
            self.castling_rights = self.castling_rights.without(CastlingRights::QueenSide);
        } else if origin == Castle::KingSide.rook_position_before_castle(self.side) {
            self.castling_rights = self.castling_rights.without(CastlingRights::KingSide);
        }
    }

    pub fn update_threats(&mut self, opposite: &SideState) {
        self.threats = self.pieces_threats.recalculate(
            self.side,
            &self.pieces,
            self.occupancy,
            opposite.occupancy,
        );
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

    pub fn update(&mut self, side: Side, movement: PseudoMove) {
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
            MoveKind::Castle(castle) => {
                let king_mask = self.piece_mut(Piece::King);
                king_mask.reset(origin);
                king_mask.set(destination);

                let rook_mask = self.piece_mut(Piece::Rook);
                rook_mask.reset(castle.rook_position_before_castle(side));
                rook_mask.set(castle.rook_position_after_castle(side));
            }
        }
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
    ) -> BoardMask {
        let mut result = BoardMask::default();

        for (threats, (squares, piece)) in
            self.0.iter_mut().zip(my_pieces.0.iter().zip(Piece::iter()))
        {
            *threats = BoardMask::default();
            for square in *squares {
                *threats = threats.intersection(movegen::threats(
                    SidedPiece(side, piece),
                    square,
                    friendly_occupancy,
                    opposite_occupancy,
                ));
            }
            result = result.intersection(*threats);
        }

        result
    }
}
