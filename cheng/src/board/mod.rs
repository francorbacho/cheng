mod mask;
pub use mask::BoardMask;

mod iterator;
mod movegen;

use crate::{
    movement::{Castle, MoveKind, PseudoMove},
    pieces::Piece,
    side_state::{CastlingRights, SideState},
    sides::Side,
    square::Square,
    SidedPiece,
};

use self::movegen::MoveGenerator;

#[derive(Debug, PartialEq, Eq)]
pub enum FENParsingError {
    MissingPart,
    TooManyParts,
    SquareUnderflow,
    SquareOverflow,
    UnknownPiece,
    InvalidTurn,
    InvalidAlignment,
    InvalidCastleRights,
}

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

    pub fn check_valid_move(&self, movement: &PseudoMove) -> bool {
        self.moves().any(|m| &m == movement)
    }

    pub fn feed(&mut self, mut movement: PseudoMove) -> Result<(), FeedError> {
        let side = self.side(self.turn);
        let moved_piece_is_king = side.pieces.piece(Piece::King).get(movement.origin);
        if moved_piece_is_king {
            if let Some(c) = Castle::move_could_be_castle(self.turn, movement.clone()) {
                movement.kind = MoveKind::Castle(c);
            }
        }

        // XXX: This is ultra bad for performance.
        if !self.check_valid_move(&movement) {
            return Err(FeedError::MoveIsNotValid);
        }

        self.feed_unchecked(movement);
        self.update_result();

        Ok(())
    }

    pub fn feed_unchecked(&mut self, movement: PseudoMove) {
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

        self.turn = self.turn.opposite();
    }

    pub fn update_result(&mut self) {
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

    #[must_use]
    pub fn into_fen(&self) -> String {
        use std::fmt::Write;

        let mut fen = String::new();
        let array = self.generate_array();

        for (i, pieces) in array.chunks(8).enumerate().rev() {
            let mut iterator = pieces.iter().peekable();
            while let Some(piece) = iterator.next() {
                match piece {
                    Some(sided_piece) => fen.push(char::from(*sided_piece)),
                    None => {
                        let mut accum = 1;
                        while iterator.peek().map_or(false, |piece| piece.is_none()) {
                            accum += 1;
                            iterator.next();
                        }
                        fen.push(char::from_digit(accum, 10).unwrap());
                    }
                }
            }

            if i != 0 {
                fen.push('/');
            }
        }

        // We always include the en passant square, if it exists. Note that lichess
        // only includes it if the capture is possible (i.e. there is a pawn to make
        // an en passant capture). This is simpler for now.
        let en_passant_str = self
            .side(self.turn.opposite())
            .en_passant
            .map_or("-".to_string(), |sq| format!("{sq:?}"));

        let castling_rights = {
            let white_castling_rights = self
                .white_side
                .castling_rights
                .to_fen_str()
                .to_ascii_uppercase();
            let black_castling_rights = self.black_side.castling_rights.to_fen_str();

            if white_castling_rights.is_empty() && black_castling_rights.is_empty() {
                String::from("-")
            } else {
                format!("{white_castling_rights}{black_castling_rights}")
            }
        };

        write!(fen, " {}", char::from(self.turn)).unwrap();
        write!(fen, " {castling_rights} {en_passant_str} 0 1").unwrap();

        fen
    }

    pub fn from_fen(fen: &str) -> Result<Self, FENParsingError> {
        use FENParsingError::{
            InvalidAlignment, InvalidCastleRights, InvalidTurn, MissingPart, SquareOverflow,
            SquareUnderflow, TooManyParts, UnknownPiece,
        };

        let fen = fen.trim();
        let mut parts = fen.split(' ');
        let board = parts.next().ok_or(MissingPart)?;

        let mut white_side = SideState::empty(Side::White);
        let mut black_side = SideState::empty(Side::Black);

        let mut squares = Square::iter_all();

        for rank in board.split('/').rev() {
            for piece_char in rank.chars() {
                if let Some(digit) = piece_char.to_digit(10) {
                    squares.nth(digit as usize - 1).ok_or(SquareOverflow)?;
                    continue;
                }

                let square = squares.next().ok_or(SquareOverflow)?;

                let side = if piece_char.is_ascii_uppercase() {
                    &mut white_side
                } else {
                    &mut black_side
                };

                let piece: Piece = piece_char
                    .to_ascii_lowercase()
                    .try_into()
                    .or(Err(UnknownPiece))?;
                side.put(square, piece);
            }

            if squares
                .next_non_consuming()
                .map_or(false, |sq| sq.file() != 0)
            {
                return Err(InvalidAlignment);
            }
        }

        if squares.next().is_some() {
            return Err(SquareUnderflow);
        }

        let turn = match parts.next() {
            Some("w") => Side::White,
            Some("b") => Side::Black,
            Some(_) => return Err(InvalidTurn),
            None => return Err(MissingPart),
        };

        let (white_castle_rights, black_castle_rights) = {
            let castle_rights_str = parts.next().ok_or(MissingPart)?;
            match CastlingRights::parse_fen_from_str(castle_rights_str) {
                Ok(castle_rights) => castle_rights,
                Err(_) => return Err(InvalidCastleRights),
            }
        };

        white_side.castling_rights = white_castle_rights;
        black_side.castling_rights = black_castle_rights;

        let _en_passant_target_square = parts.next().ok_or(MissingPart)?;
        let _halfmove_clock = parts.next().ok_or(MissingPart)?;
        let _fullmove_clock = parts.next().ok_or(MissingPart)?;

        if parts.next().is_some() {
            Err(TooManyParts)
        } else {
            Ok(Self {
                white_side,
                black_side,
                turn,
                result: None,
            })
        }
    }
}

impl Default for Board {
    #[inline]
    fn default() -> Self {
        Self::from_fen(Board::DEFAULT_FEN).unwrap()
    }
}
