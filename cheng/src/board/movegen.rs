use crate::{movement::MoveKind, pieces::Piece, Board, PseudoMove, SidedPiece};

use super::BoardMask;

pub struct MoveGenerator<'a> {
    pub board: &'a Board,

    pub cached_moves: Vec<PseudoMove>,
    pub idx: usize,
}

impl<'a> MoveGenerator<'a> {
    pub fn new(board: &'a Board) -> Self {
        let mut gen = Self {
            board,
            cached_moves: Vec::with_capacity(20),
            idx: 0,
        };

        gen.genmoves();
        gen
    }

    fn genmoves(&mut self) {
        let friendly = self.board.side(self.board.turn).occupancy;
        let opposite = self.board.side(self.board.turn.opposite()).occupancy;
        let opposite_threats = self.board.side(self.board.turn.opposite()).threats;

        for piece in Piece::iter() {
            for piece_square in self.board.side(self.board.turn).pieces.piece(piece) {
                let opposite = if piece == Piece::Pawn {
                    match self.board.side(self.board.turn.opposite()).en_passant {
                        Some(square) => opposite.intersection(BoardMask::from(square)),
                        None => opposite,
                    }
                } else {
                    opposite
                };

                let moves = crate::movegen::moves(
                    SidedPiece(self.board.turn, piece),
                    piece_square,
                    friendly,
                    opposite,
                );

                let moves = if piece == Piece::King {
                    moves.without(opposite_threats)
                } else {
                    moves
                };

                for destination in moves {
                    let mut clone = self.board.clone();
                    let movement = PseudoMove {
                        origin: piece_square,
                        destination,
                        kind: MoveKind::Move,
                    };
                    clone.feed_unchecked(movement.clone());
                    if clone.side(self.board.turn).king_in_check {
                        continue;
                    }
                    self.cached_moves.push(movement);
                }
            }
        }
    }
}

impl<'a> Iterator for MoveGenerator<'a> {
    type Item = PseudoMove;

    fn next(&mut self) -> Option<Self::Item> {
        let pseudomove = self.cached_moves.get(self.idx)?;
        self.idx += 1;
        Some(pseudomove.clone())
    }
}
