use crate::{
    movement::{Castle, MoveKind},
    pieces::Piece,
    side_state::CastlingRights,
    square::Square,
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

    fn checked_add_move(&mut self, movement: PseudoMove) {
        let mut clone = self.board.clone();
        clone.feed_unchecked(&movement);
        if clone.side(self.board.turn).king_in_check {
            return;
        }
        self.unchecked_add_move(movement);
    }

    #[inline]
    fn unchecked_add_move(&mut self, movement: PseudoMove) {
        self.cached_moves.push(movement);
    }

    fn generate_castles(&mut self) {
        use crate::prelude::*;
        // The squares that must be unoccupied and unthreathened to be able
        // to castle. For example, in white's king side castle, F1 and G1.
        fn can_castle(
            relevant_squares_occupancy: BoardMask,
            relevant_squares_threats: BoardMask,
            occupancy: BoardMask,
            opposite_threats: BoardMask,
        ) -> bool {
            !relevant_squares_occupancy.has_coincidences(occupancy)
                && !relevant_squares_threats.has_coincidences(opposite_threats)
        }

        let side = self.board.side(self.board.turn);
        let opposite_side = self.board.side(self.board.turn.opposite());

        if side.castling_rights == CastlingRights::None || side.king_in_check {
            return;
        }

        let king_square = Castle::king_square_before_castle(side.side);
        let (queen_side_castle_square, king_side_castle_square) = match side.side {
            Side::White => (C1, G1),
            Side::Black => (C8, G8),
        };

        let occupancy = side.occupancy.intersection(opposite_side.occupancy);

        if side.castling_rights.queen_side() {
            let relevant_square_occupancy = Castle::QueenSide.relevant_square_occupancy(side.side);
            let relevant_square_threats = Castle::QueenSide.relevant_square_threats(side.side);

            if can_castle(
                relevant_square_occupancy,
                relevant_square_threats,
                occupancy,
                opposite_side.threats,
            ) {
                let queen_side_castle = PseudoMove {
                    origin: king_square,
                    destination: queen_side_castle_square,
                    kind: MoveKind::Castle(Castle::QueenSide),
                };

                self.unchecked_add_move(queen_side_castle);
            }
        }

        if side.castling_rights.king_side() {
            let relevant_squares_occupancy = Castle::KingSide.relevant_square_occupancy(side.side);
            let relevant_squares_threats = Castle::KingSide.relevant_square_threats(side.side);

            if can_castle(
                relevant_squares_occupancy,
                relevant_squares_threats,
                occupancy,
                opposite_side.threats,
            ) {
                let king_side_castle = PseudoMove {
                    origin: king_square,
                    destination: king_side_castle_square,
                    kind: MoveKind::Castle(Castle::KingSide),
                };

                self.unchecked_add_move(king_side_castle);
            }
        }
    }

    fn generate_moves(&mut self) {
        let friendly = self.board.side(self.board.turn).occupancy;
        let opposite = self.board.side(self.board.turn.opposite()).occupancy;
        let opposite_threats = self.board.side(self.board.turn.opposite()).threats;

        for piece in Piece::iter() {
            for piece_square in self.board.side(self.board.turn).pieces.piece(piece) {
                if piece == Piece::Pawn {
                    self.generate_pawn_moves(piece_square);
                    continue;
                }

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
                    let movement = PseudoMove {
                        origin: piece_square,
                        destination,
                        kind: MoveKind::Move,
                    };

                    self.checked_add_move(movement);
                }
            }
        }
    }

    fn generate_pawn_moves(&mut self, square: Square) {
        use crate::prelude::{A2, A7};

        let friendly = self.board.side(self.board.turn).occupancy;
        let opposite = self.board.side(self.board.turn.opposite()).occupancy;

        let opposite = match self.board.side(self.board.turn.opposite()).en_passant {
            Some(square) => opposite.intersection(BoardMask::from(square)),
            None => opposite,
        };

        let moves = crate::movegen::moves(
            SidedPiece(self.board.turn, Piece::Pawn),
            square,
            friendly,
            opposite,
        );

        let moves_are_promotion = match self.board.turn {
            Side::White => A7.rank::<usize>() == square.rank(),
            Side::Black => A2.rank::<usize>() == square.rank(),
        };

        if moves_are_promotion {
            for destination in moves {
                for piece in Piece::iter_promotable_pieces() {
                    let movement = PseudoMove {
                        origin: square,
                        destination,
                        kind: MoveKind::Promote(piece),
                    };
                    self.checked_add_move(movement);
                }
            }
        } else {
            for destination in moves {
                let movement = PseudoMove {
                    origin: square,
                    destination,
                    kind: MoveKind::Move,
                };
                self.checked_add_move(movement);
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
