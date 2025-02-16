use crate::{Square, Side, SidedPiece, Piece};
use crate::{BoardMask, PseudoMove};
use crate::movegen;
use super::masks::{MaskBySidedPiece, MaskBySide};

#[derive(Clone, PartialEq, Eq)]
pub struct BorkedBoard {
    turn: Side,
    occupancy: MaskBySidedPiece,
    // TODO: We could avoid recomputing this all the time
    // threats: MaskBySidedPiece,
    threats: MaskBySide,
}

impl BorkedBoard {
    pub fn new(turn: Side) -> Self {
        Self {
            turn,
            occupancy: MaskBySidedPiece::default(),
            threats: MaskBySide::default(),
        }
    }

    pub fn is_borked(&self) -> bool {
        let threats = self.threats.get(self.turn.opposite());
        self.occupancy.get(SidedPiece(self.turn, Piece::King)).has_coincidences(threats)
    }

    /// SAFETY: Caller must guaratee that the move is valid, otherwise the
    /// function may panic.
    pub unsafe fn feed_unchecked(&mut self, pseudomove: PseudoMove) {
        // XXX: Benchmark unwrap_unchecked()
        let mask = self.occupancy.find_mask_by_side_and_square(self.turn, pseudomove.origin).unwrap();
        mask.replace(pseudomove.origin, pseudomove.destination);

        self.occupancy.reset_all_for_side(self.turn.opposite(), pseudomove.destination);

        self.calculate_threats();

        self.turn = self.turn.opposite();
    }

    fn calculate_threats(&mut self) {
        self.threats = MaskBySide::default();

        let white_occupancy = self.occupancy.get_side(Side::White).into_iter().copied().reduce(|acc, m| acc.with(m)).unwrap();
        let black_occupancy = self.occupancy.get_side(Side::Black).into_iter().copied().reduce(|acc, m| acc.with(m)).unwrap();

        for sided_piece in SidedPiece::iter() {
            let SidedPiece(side, _) = sided_piece;
            let piece_mask = self.occupancy.get(sided_piece);
            let (friendly_occupancy, opposite_occupancy) = if side == Side::White {
                (white_occupancy, black_occupancy)
            } else {
                (black_occupancy, white_occupancy)
            };

            for square in piece_mask {
                let new_threats = movegen::threats(
                    sided_piece,
                    square,
                    friendly_occupancy,
                    opposite_occupancy,
                );
                let threats = self.threats.get_mut(side);
                *threats = threats.with(new_threats);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{SidedPiece, Side, Piece, Square};
    use crate::prelude::*;
    use super::*;

    #[test]
    fn threats() {
        crate::init();

        let mut borked_board = BorkedBoard::new(Side::White);
        let mask = borked_board.occupancy.get_mut(SidedPiece(Side::White, Piece::Rook));
        mask.set(A4);

        borked_board.calculate_threats();

        let actual = borked_board.threats.get(Side::White);
        let expected = BoardMask::from([A1, A2, A3, A5, A6, A7, A8, B4, C4, D4, E4, F4, G4, H4]);
        assert_eq!(actual, expected);
    }
}
