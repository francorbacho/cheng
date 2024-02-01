use crate::{
    side_state::{CastlingRights, SideState},
    Board, BorkedBoard, Castle, FromIntoFen, Piece, Side, Square,
};

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
    WrongEnPassantSquare,
    InvalidHalfMoveClock,
    InvalidFullMoveClock,
}

impl FromIntoFen for Board {
    type Error = FENParsingError;

    fn into_fen(&self) -> String {
        self.inner.into_fen()
    }

    fn from_fen(fen: &str) -> Result<Self, Self::Error> {
        Ok(Board::try_from(BorkedBoard::from_fen(fen)?).unwrap())
    }
}

impl FromIntoFen for BorkedBoard {
    type Error = FENParsingError;

    #[must_use]
    fn into_fen(&self) -> String {
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

        let hmove = self.halfmove_clock;
        let fmove = self.fullmove_clock;

        write!(fen, " {}", char::from(self.turn)).unwrap();
        write!(fen, " {castling_rights} {en_passant_str} {hmove} {fmove}").unwrap();

        fen
    }

    fn from_fen(fen: &str) -> Result<Self, FENParsingError> {
        use FENParsingError::*;

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
                .map_or(false, |sq| sq.file::<usize>() != 0)
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

        if are_castling_rights_ok(white_castle_rights, &white_side) {
            white_side.castling_rights = white_castle_rights;
        }

        if are_castling_rights_ok(black_castle_rights, &black_side) {
            black_side.castling_rights = black_castle_rights;
        }

        let en_passant_square = parts.next().ok_or(MissingPart)?;
        if en_passant_square != "-" {
            let Ok(en_passant_square) = en_passant_square.parse() else {
                return Err(WrongEnPassantSquare);
            };

            if turn == Side::White {
                black_side.en_passant = Some(en_passant_square);
            } else {
                white_side.en_passant = Some(en_passant_square);
            }
        }

        let halfmove_clock = parts
            .next()
            .ok_or(MissingPart)?
            .parse()
            .map_err(|_| InvalidHalfMoveClock)?;
        let fullmove_clock = parts
            .next()
            .ok_or(MissingPart)?
            .parse()
            .map_err(|_| InvalidFullMoveClock)?;

        if parts.next().is_some() {
            Err(TooManyParts)
        } else {
            Ok(Self {
                white_side,
                black_side,
                turn,
                halfmove_clock,
                fullmove_clock,
            })
        }
    }
}

#[inline(always)]
fn are_castling_rights_ok(rights: CastlingRights, side: &SideState) -> bool {
    let queenside_ok = || -> bool {
        side.pieces
            .piece(Piece::Rook)
            .get(Castle::QueenSide.rook_square_before_castle(side.side))
    };

    let kingside_ok = || -> bool {
        side.pieces
            .piece(Piece::Rook)
            .get(Castle::KingSide.rook_square_before_castle(side.side))
    };

    match rights {
        CastlingRights::None => true,
        CastlingRights::QueenSide => queenside_ok(),
        CastlingRights::KingSide => kingside_ok(),
        CastlingRights::Both => queenside_ok() && kingside_ok(),
    }
}
