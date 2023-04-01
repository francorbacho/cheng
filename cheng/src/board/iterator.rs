use crate::{side_state::iterator::SidePiecesIterator, Board, Side, SidedPiece, Square};

impl<'a> IntoIterator for &'a Board {
    type Item = (SidedPiece, Square);

    type IntoIter = BoardIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        BoardIterator::new(self)
    }
}

pub struct BoardIterator<'a> {
    white_pieces_iterator: SidePiecesIterator<'a>,
    black_pieces_iterator: SidePiecesIterator<'a>,
}

impl<'a> BoardIterator<'a> {
    pub fn new(board: &'a Board) -> Self {
        Self {
            white_pieces_iterator: board.white_side.pieces.into_iter(),
            black_pieces_iterator: board.black_side.pieces.into_iter(),
        }
    }
}

impl Iterator for BoardIterator<'_> {
    type Item = (SidedPiece, Square);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(result) = self
            .white_pieces_iterator
            .next()
            .map(|(piece, square)| (SidedPiece(Side::White, piece), square))
        {
            return Some(result);
        }

        if let Some(result) = self
            .black_pieces_iterator
            .next()
            .map(|(piece, square)| (SidedPiece(Side::Black, piece), square))
        {
            return Some(result);
        }

        None
    }
}
