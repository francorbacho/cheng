use crate::{
    board::BoardMask, side_state::SideState, GameResult, LegalMove, Piece, PseudoMove,
    PseudoMoveGenerator, Side, SidedPiece,
};

use super::TryFeedError;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BorkedBoard {
    pub white_side: SideState,
    pub black_side: SideState,
    pub turn: Side,
    pub halfmove_clock: usize,
    pub fullmove_clock: usize,
}

impl Default for BorkedBoard {
    #[inline]
    fn default() -> Self {
        Self::empty()
    }
}

impl BorkedBoard {
    #[inline]
    #[must_use]
    pub fn empty() -> Self {
        Self {
            white_side: SideState::empty(Side::White),
            black_side: SideState::empty(Side::Black),
            turn: Side::White,
            halfmove_clock: 0,
            fullmove_clock: 1,
        }
    }

    #[inline]
    #[must_use]
    pub fn side(&self, side: Side) -> &SideState {
        match side {
            Side::White => &self.white_side,
            Side::Black => &self.black_side,
        }
    }

    #[inline]
    pub fn side_mut(&mut self, side: Side) -> &mut SideState {
        match side {
            Side::White => &mut self.white_side,
            Side::Black => &mut self.black_side,
        }
    }

    #[inline]
    #[must_use]
    pub fn is_borked(&self) -> bool {
        self.side(self.turn.opposite()).king_in_check
    }

    pub fn does_move_bork(&self, pseudomove: PseudoMove) -> bool {
        // XXX: The `feed_unchecked` does a lot of computation that
        //      maybe we don't need (update threats, passed pawns...).
        //      Can this be improved?

        let mut clone = self.clone();
        clone.feed_unchecked(&pseudomove);
        clone.is_borked()
    }

    // FIXME: Can we integrate this function with does_move_bork()?
    //        What is the difference?
    pub fn is_move_valid(&self, pseudomove: PseudoMove) -> bool {
        // TODO: Refactor this. Shares some code with PseudoMoveGenerator and others.

        let Some(piece) = self.side(self.turn).pieces.find(pseudomove.origin) else {
            return false;
        };

        let friendly = self.side(self.turn).occupancy;
        let mut opposite = self.side(self.turn.opposite()).occupancy;

        if piece == Piece::King {
            if let Some(c) = crate::movement::Castle::move_could_be_castle(self.turn, &pseudomove) {
                return self.side(self.turn).castling_rights.contains(c);
            }
        } else if piece == Piece::Pawn {
            opposite = match self.side(self.turn.opposite()).en_passant {
                Some(square) => opposite.with(BoardMask::from(square)),
                None => opposite,
            };
        }

        let piece = SidedPiece(self.turn, piece);

        crate::movegen::moves(piece, pseudomove.origin, friendly, opposite)
            .get(pseudomove.destination)
    }

    pub fn try_feed<M>(&mut self, movement: M) -> Result<(), TryFeedError<M::Error>>
    where
        M: TryInto<PseudoMove>,
    {
        let movement = match movement.try_into() {
            Ok(movement) => movement,
            Err(err) => return Err(TryFeedError::Parsing(err)),
        };

        if !self.is_move_valid(movement.clone()) {
            return Err(TryFeedError::InvalidMove);
        }

        let Some(legalmove) = LegalMove::new(movement, self) else {
            return Err(TryFeedError::InvalidMove);
        };

        self.feed_unchecked(&legalmove.into());

        Ok(())
    }

    pub fn feed_unchecked(&mut self, movement: &PseudoMove) {
        let piece_is_pawn = self
            .side(self.turn)
            .pieces
            .piece(Piece::Pawn)
            .get(movement.origin);

        self.side_mut(self.turn).update(movement.clone());

        if Some(movement.destination) == self.side(self.turn.opposite()).en_passant && piece_is_pawn
        {
            // En passant capture
            let side = self.side_mut(self.turn.opposite());
            let pawn_pieces = side.pieces.piece_mut(Piece::Pawn);
            let actual_pawn_square = movement.destination.next_rank(side.side);
            side.occupancy.reset(actual_pawn_square);
            pawn_pieces.reset(actual_pawn_square);
        }

        // This handles en passant capture as well.
        if piece_is_pawn
            || self
                .side(self.turn.opposite())
                .occupancy
                .get(movement.destination)
        {
            self.halfmove_clock = 0;
        } else {
            self.halfmove_clock += 1;
        }

        if self.turn == Side::Black {
            self.fullmove_clock += 1;
        }

        self.side_mut(self.turn.opposite())
            .remove(movement.destination);

        self.white_side.update_threats(&self.black_side);
        self.black_side.update_threats(&self.white_side);

        self.white_side.update_king_in_check(&self.black_side);
        self.black_side.update_king_in_check(&self.white_side);

        self.turn = self.turn.opposite();
    }

    pub fn compute_result(&self) -> GameResult {
        debug_assert!(!self.is_borked());

        if self.halfmove_clock >= 100 {
            return GameResult::Draw;
        }

        for pseudomove in self.moves() {
            let mut clone = self.clone();
            clone.feed_unchecked(&pseudomove);
            if !clone.is_borked() {
                return GameResult::Undecided;
            }
        }

        if self.side(self.turn).king_in_check {
            GameResult::Checkmate {
                winner: self.turn.opposite(),
            }
        } else {
            GameResult::Draw
        }
    }

    #[must_use]
    pub fn moves(&self) -> PseudoMoveGenerator {
        PseudoMoveGenerator::new(self)
    }

    #[must_use]
    pub fn generate_array(&self) -> [Option<SidedPiece>; 64] {
        let mut board_vec = [None; 64];

        for (sided_piece, square) in self {
            let idx = square.to_index();
            assert_eq!(board_vec[idx], None);
            board_vec[idx] = Some(sided_piece);
        }

        board_vec
    }
}
