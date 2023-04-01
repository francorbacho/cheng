use std::iter::Peekable;

use crate::{pieces::PieceIterator, side_state::BoardMask, Piece, Square};

use super::SidePieces;

impl<'a> IntoIterator for &'a SidePieces {
    type Item = (Piece, Square);

    type IntoIter = SidePiecesIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        SidePiecesIterator::new(self)
    }
}

pub struct SidePiecesIterator<'a> {
    pieces: &'a SidePieces,
    current_piece: Peekable<PieceIterator>,
    current_mask: BoardMask,
}

impl<'a> SidePiecesIterator<'a> {
    pub fn new(pieces: &'a SidePieces) -> Self {
        Self {
            pieces,
            current_piece: Piece::iter().peekable(),
            current_mask: pieces.piece(Piece::Pawn),
        }
    }

    fn update_current_mask(&mut self) {
        match self.current_piece.peek() {
            Some(piece) => {
                self.current_mask = self.pieces.piece(*piece);
            }
            None => {}
        }
    }
}

impl<'a> Iterator for SidePiecesIterator<'a> {
    type Item = (Piece, Square);

    fn next(&mut self) -> Option<Self::Item> {
        let piece = match self.current_piece.peek() {
            Some(piece) => *piece,
            None => return None,
        };

        let square = match self.current_mask.first() {
            Some(square) => square,
            None => {
                self.current_piece.next();
                self.update_current_mask();
                return self.next();
            }
        };

        self.current_mask.reset(square);
        Some((piece, square))
    }
}

#[cfg(test)]
mod test {
    use crate::{prelude::*, side_state::SideState, Piece, Side};

    use super::SidePiecesIterator;

    #[test]
    fn check() {
        let mut side_piece = SideState::empty(Side::White);

        let pieces = [
            (A1, Piece::Pawn),
            (A2, Piece::Pawn),
            (A3, Piece::Pawn),
            (B1, Piece::Knight),
            (B2, Piece::Knight),
            (C1, Piece::Bishop),
            (C2, Piece::Bishop),
            (D1, Piece::Rook),
            (D2, Piece::Rook),
            (E1, Piece::Queen),
            (E2, Piece::Queen),
            (F3, Piece::King),
        ];

        for (square, piece) in pieces {
            side_piece.put(square, piece);
        }

        let mut iterator = SidePiecesIterator::new(&side_piece.pieces);

        for (square, piece) in pieces {
            assert_eq!(iterator.next(), Some((piece, square)));
        }

        assert!(iterator.next().is_none());
    }
}
