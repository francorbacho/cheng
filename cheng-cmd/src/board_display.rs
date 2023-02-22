use std::fmt::Display;

use cheng::{Board, Piece, Side, SidedPiece};

pub struct BoardDisplay<'a>(pub &'a Board);

impl BoardDisplay<'_> {
    fn generate_array(&self) -> [Option<SidedPiece>; 64] {
        let mut board_vec = [None; 64];

        for (mask, piece) in self.0.white_side.pieces.iter().zip(Piece::iter()) {
            for square in mask {
                let idx = square.to_index();
                assert_eq!(board_vec[idx], None);

                board_vec[idx] = Some((Side::White, piece));
            }
        }

        for (mask, piece) in self.0.black_side.pieces.iter().zip(Piece::iter()) {
            for square in mask {
                let idx = square.to_index();
                assert_eq!(board_vec[idx], None);

                board_vec[idx] = Some((Side::Black, piece));
            }
        }

        board_vec
    }
}

impl Display for BoardDisplay<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn sided_piece_to_string((side, piece): SidedPiece) -> String {
            if side == Side::White {
                char::from(piece).to_uppercase().to_string()
            } else {
                char::from(piece).to_string()
            }
        }

        let array = self.generate_array();

        writeln!(f, " +---+---+---+---+---+---+---+---+")?;
        for (i, rank) in array.chunks(8).enumerate().rev() {
            write!(f, " |")?;

            for sided_piece in rank {
                match sided_piece {
                    Some(sided_piece) => write!(f, " {} ", sided_piece_to_string(*sided_piece))?,
                    None => write!(f, "   ")?,
                }

                write!(f, "|")?;
            }

            writeln!(f, "  {}", i + 1)?;
            writeln!(f, " +---+---+---+---+---+---+---+---+")?;
        }
        writeln!(f, "   a   b   c   d   e   f   g   h")?;

        Ok(())
    }
}
