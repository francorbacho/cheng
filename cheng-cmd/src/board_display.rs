use std::fmt::Display;

use cheng::Board;

pub struct BoardDisplay<'a>(pub &'a Board);

impl Display for BoardDisplay<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let array = self.0.generate_array();
        writeln!(f, " +---+---+---+---+---+---+---+---+")?;
        for (i, rank) in array.chunks(8).enumerate().rev() {
            write!(f, " |")?;

            for sided_piece in rank {
                match sided_piece {
                    Some(sided_piece) => write!(f, " {} ", char::from(*sided_piece))?,
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
