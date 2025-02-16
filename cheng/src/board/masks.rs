use crate::BoardMask;
use crate::{Side, Piece, Square, SidedPiece};

#[derive(Clone, Default, PartialEq, Eq)]
pub struct MaskBySide([BoardMask; 2]);

impl MaskBySide {
    pub fn get(&self, side: Side) -> BoardMask {
        self.0[usize::from(side)]
    }

    pub fn get_mut(&mut self, side: Side) -> &mut BoardMask {
        &mut self.0[usize::from(side)]
    }
}

#[derive(Clone, Default, PartialEq, Eq)]
pub struct MaskBySidedPiece([[BoardMask; 6]; 2]);

impl MaskBySidedPiece {
    pub fn get_side(&self, side: Side) -> &[BoardMask] {
        &self.0[usize::from(side)]
    }

    pub fn get(&self, SidedPiece(side, piece): SidedPiece) -> BoardMask {
        self.0[usize::from(side)][usize::from(piece)]
    }

    pub fn get_mut(&mut self, SidedPiece(side, piece): SidedPiece) -> &mut BoardMask {
        &mut self.0[usize::from(side)][usize::from(piece)]
    }

    pub fn reset_all_for_side(&mut self, side: Side, square: Square) {
        for piece in Piece::iter() {
            self.get_mut(SidedPiece(side, piece)).reset(square);
        }
    }

    pub fn find_mask_by_side_and_square(&mut self, side: Side, square: Square) -> Option<&mut BoardMask> {
        for piece in Piece::iter() {
            let sided_piece = SidedPiece(side, piece);
            if self.get(sided_piece).get(square) {
                return Some(self.get_mut(sided_piece));
            }
        }

        None
    }
}
