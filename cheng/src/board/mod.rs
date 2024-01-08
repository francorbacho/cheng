mod mask;
pub use mask::BoardMask;

mod iterator;
mod movegen;
mod parsing;

pub use parsing::FENParsingError;

use crate::{
    movement::{Castle, LegalMove, MoveKind},
    pieces::Piece,
    side_state::SideState,
    sides::Side,
    SidedPiece,
};

use self::movegen::MoveGenerator;

#[derive(Debug)]
pub enum FeedError {
    MoveIsNotValid,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GameResult {
    Draw,
    Checkmate { winner: Side },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Board {
    pub white_side: SideState,
    pub black_side: SideState,
    pub turn: Side,
    pub halfmove_clock: usize,
    pub fullmove_clock: usize,
    result: Option<GameResult>,
}

impl Board {
    pub const DEFAULT_FEN: &'static str =
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

    #[inline]
    #[must_use]
    pub fn empty() -> Self {
        Self {
            white_side: SideState::empty(Side::White),
            black_side: SideState::empty(Side::Black),
            turn: Side::White,
            halfmove_clock: 0,
            fullmove_clock: 1,
            result: None,
        }
    }

    #[inline]
    #[must_use]
    pub fn result(&self) -> Option<GameResult> {
        self.result
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

    #[must_use]
    pub fn check_valid_move(&self, movement: &LegalMove) -> bool {
        self.moves().any(|m| &m == movement)
    }

    pub fn feed(&mut self, mut movement: LegalMove) -> Result<(), FeedError> {
        let side = self.side(self.turn);
        let moved_piece_is_king = side.pieces.piece(Piece::King).get(movement.origin);
        if moved_piece_is_king {
            if let Some(c) = Castle::move_could_be_castle(self.turn, &movement) {
                movement.kind = MoveKind::Castle(c);
            }
        }

        // XXX: This is ultra bad for performance.
        if !self.check_valid_move(&movement) {
            return Err(FeedError::MoveIsNotValid);
        }

        self.feed_unchecked(&movement);
        self.update_result();

        Ok(())
    }

    pub fn feed_unchecked(&mut self, movement: &LegalMove) {
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

        self.side_mut(self.turn.opposite())
            .remove(movement.destination);

        self.white_side.update_threats(&self.black_side);
        self.black_side.update_threats(&self.white_side);

        self.white_side.update_king_in_check(&self.black_side);
        self.black_side.update_king_in_check(&self.white_side);

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

        self.turn = self.turn.opposite();
    }

    pub fn update_result(&mut self) {
        if self.halfmove_clock >= 100 {
            self.result = Some(GameResult::Draw);
        }

        if !self.side(self.turn).king_in_check {
            // TODO: Check stalemate.
            return;
        }

        let movegen = MoveGenerator::new(self);
        for movement in movegen {
            let mut clone = self.clone();
            clone.feed(movement).unwrap();

            if !clone.side(self.turn).king_in_check {
                return;
            }
        }
        self.result = Some(GameResult::Checkmate {
            winner: self.turn.opposite(),
        });
    }

    #[must_use]
    pub fn moves(&self) -> MoveGenerator {
        MoveGenerator::new(self)
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

impl Default for Board {
    #[inline]
    fn default() -> Self {
        Self::from_fen(Board::DEFAULT_FEN).unwrap()
    }
}
