use crate::{
    movement::{Castle, MoveKind},
    pieces::Piece,
    side_state::CastlingRights,
    Board, PseudoMove, Side, SidedPiece,
};

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

        gen.generate_all_moves();
        gen
    }

    fn generate_all_moves(&mut self) {
        self.generate_moves();
        self.generate_castles();
    }

    fn generate_castles(&mut self) {
        let side = self.board.side(self.board.turn);
        let opposite_side = self.board.side(self.board.turn.opposite());

        if side.castling_rights == CastlingRights::None || side.king_in_check {
            return;
        }

        // The squares that must be unoccupied and unthreathened to be able
        // to castle. For example, in white's king side castle, F1 and G1.
        fn can_castle(
            relevant_squares: BoardMask,
            friendly_occupancy: BoardMask,
            opposite_threats: BoardMask,
        ) -> bool {
            !relevant_squares.has_coincidences(friendly_occupancy)
                && !relevant_squares.has_coincidences(opposite_threats)
        }

        use crate::consts::*;
        let king_square = Castle::king_square_before_castle(side.side);
        let (queen_side_castle_square, king_side_castle_square) = match side.side {
            Side::White => (C1, G1),
            Side::Black => (C8, G8),
        };

        if side.castling_rights.queen_side() {
            let relevant_squares = Castle::QueenSide.relevant_squares(side.side);

            if can_castle(relevant_squares, side.occupancy, opposite_side.threats) {
                let queen_side_castle = PseudoMove {
                    origin: king_square,
                    destination: queen_side_castle_square,
                    kind: MoveKind::Castle(Castle::QueenSide),
                };

                self.cached_moves.push(queen_side_castle);
            }
        }

        if side.castling_rights.king_side() {
            let relevant_squares = Castle::KingSide.relevant_squares(side.side);

            if can_castle(relevant_squares, side.occupancy, opposite_side.threats) {
                let king_side_castle = PseudoMove {
                    origin: king_square,
                    destination: king_side_castle_square,
                    kind: MoveKind::Castle(Castle::KingSide),
                };

                self.cached_moves.push(king_side_castle);
            }
        }
    }

    fn generate_moves(&mut self) {
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
