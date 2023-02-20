mod mask;
pub use mask::BoardMask;

mod movegen;

use crate::{
    movement::PseudoMove,
    pieces::Piece,
    sides::{Side, SideState},
    square::Square,
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
    #[inline]
    pub fn empty() -> Self {
        Self {
            white_side: SideState::empty(),
            black_side: SideState::empty(),
            turn: Side::White,
            result: None,
        }
    }

    #[inline]
    pub fn result(&self) -> Option<GameResult> {
        self.result
    }

    #[inline]
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

    pub fn feed(&mut self, movement: PseudoMove) {
        self.feed_unchecked(movement);
    }

    pub fn feed_unchecked(&mut self, movement: PseudoMove) {
        self.side_mut(self.turn).update(movement.clone());

        if movement.takes.unwrap_or(true) {
            self.side_mut(self.turn.opposite())
                .remove(movement.destination);
        }

        self.white_side.update_threats(&self.black_side);
        self.black_side.update_threats(&self.white_side);

        self.white_side.update_king_in_check(&self.black_side);
        self.black_side.update_king_in_check(&self.white_side);

        self.turn = self.turn.opposite();

        self.update_result();
    }

    pub fn update_result(&mut self) {
        if !self.side(self.turn).king_in_check {
            // TODO: Check stalemate.
            return;
        }

        let movegen = MoveGenerator::new(self);
        for movement in movegen {
            let mut clone = self.clone();
            clone.feed(movement);

            if !clone.side(self.turn).king_in_check {
                return;
            }
        }
        self.result = Some(GameResult::Checkmate {
            winner: self.turn.opposite(),
        });
    }

    pub fn moves(&self) -> MoveGenerator {
        MoveGenerator::new(self)
    }

    pub fn from_fen(fen: &str) -> Result<Self, FENParsingError> {
        use FENParsingError::*;

        let mut parts = fen.split(' ');
        let board = parts.next().ok_or(MissingPart)?;

        let mut white_side = SideState::empty();
        let mut black_side = SideState::empty();

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

        let _castle_permission = parts.next().ok_or(MissingPart)?;
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
        Self::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap()
    }
}
