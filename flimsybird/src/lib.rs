use cheng::{Board, Piece, Side, SidedPiece};

#[derive(Default, Debug, Clone, Copy)]
pub struct Evaluation(pub i32);

impl Evaluation {
    pub const BLACK_WIN: Self = Evaluation(std::i32::MIN);
    pub const WHITE_WIN: Self = Evaluation(std::i32::MAX);

    pub fn winning(self) -> Side {
        if self.0 < 0 {
            Side::Black
        } else {
            Side::White
        }
    }

    pub fn is_better_than(self, side: Side, ev2: Self) -> bool {
        if side == Side::White {
            self.0 > ev2.0
        } else {
            self.0 < ev2.0
        }
    }
}

pub trait Evaluable {
    fn evaluate(&mut self) -> Evaluation;
}

impl Evaluable for Board {
    fn evaluate(&mut self) -> Evaluation {
        let mut result = 0;
        for (SidedPiece(side, piece), _) in self.into_iter() {
            let side_factor = if side == Side::Black { -1 } else { 1 };
            let piece_value = match piece {
                Piece::Pawn => 100,
                Piece::Knight => 300,
                Piece::Bishop => 325,
                Piece::Rook => 500,
                Piece::Queen => 900,
                Piece::King => 0,
            };

            result += side_factor * piece_value;
        }

        Evaluation(result)
    }
}
