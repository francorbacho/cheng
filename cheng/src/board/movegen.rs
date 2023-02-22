use crate::{movement::MoveKind, pieces::Piece, Board, PseudoMove};

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

        for piece in Piece::iter() {
            for piece_square in self.board.side(self.board.turn).pieces.piece(piece) {
                let moves = crate::movegen::moves(
                    (self.board.turn, piece),
                    piece_square,
                    friendly,
                    opposite,
                );
                for destination in moves {
                    self.cached_moves.push(PseudoMove {
                        origin: piece_square,
                        destination,
                        kind: MoveKind::Move,
                    });
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
