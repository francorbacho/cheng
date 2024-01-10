#![feature(generic_const_exprs)]

#[path = "src/square.rs"]
mod square;

#[path = "src/board/mask.rs"]
mod board;

#[path = "src/movegen/steady.rs"]
mod steady;

#[path = "src/movegen/hash.rs"]
mod hash;

mod pieces {
    pub struct Bishop;
    pub struct Rook;
}

#[path = "src/sides.rs"]
mod sides;

use std::{
    collections::{hash_map::Entry, HashMap},
    fmt, fs,
    io::{self, Write},
};

use board::BoardMask;
use hash::magic_hash;
use pieces::{Bishop, Rook};
use square::Square;
use steady::SlidingPiece;

const FILE: &str = "src/movegen/precomputed.rs";

fn main() -> io::Result<()> {
    println!("cargo:rerun-if-changed=build.rs");

    let mut file = fs::File::create(FILE).unwrap();
    write_prelude(&mut file)?;

    write_to_file(&mut file, "KNIGHT_MOVES", |square, target| {
        let rank_diff = (target.rank::<i32>() - square.rank::<i32>()).abs();
        let file_diff = (target.file::<i32>() - square.file::<i32>()).abs();

        matches!((rank_diff, file_diff), (2, 1) | (1, 2))
    })?;

    write_to_file(&mut file, "KING_MOVES", |square, target| {
        let rank_diff = (target.rank::<i32>() - square.rank::<i32>()).abs();
        let file_diff = (target.file::<i32>() - square.file::<i32>()).abs();

        matches!((rank_diff, file_diff), (0 | 1, 1) | (1, 0))
    })?;

    write_to_file(&mut file, "PAWN_MOVES_WHITE", |square, target| {
        if square.file::<usize>() != target.file() {
            return false;
        }

        let rank_diff = target.rank::<i32>() - square.rank::<i32>();

        match square.rank() {
            0 => false,
            1 => rank_diff == 1 || rank_diff == 2,
            _ => rank_diff == 1,
        }
    })?;

    write_to_file(&mut file, "PAWN_MOVES_BLACK", |square, target| {
        if square.file::<usize>() != target.file() {
            return false;
        }

        let rank_diff = target.rank::<i32>() - square.rank::<i32>();

        match square.rank() {
            7 => false,
            6 => rank_diff == -1 || rank_diff == -2,
            _ => rank_diff == -1,
        }
    })?;

    write_to_file(&mut file, "PAWN_CAPTURES_WHITE", |square, target| {
        let file_diff = (target.file::<i32>() - square.file::<i32>()).abs();
        let rank_diff = target.rank::<i32>() - square.rank::<i32>();

        match square.rank() {
            0 => false,
            _ => rank_diff == 1 && file_diff == 1,
        }
    })?;

    write_to_file(&mut file, "PAWN_CAPTURES_BLACK", |square, target| {
        let file_diff = (target.file::<i32>() - square.file::<i32>()).abs();
        let rank_diff = target.rank::<i32>() - square.rank::<i32>();

        match square.rank() {
            7 => false,
            _ => rank_diff == -1 && file_diff == 1,
        }
    })?;

    write_sliding_piece::<Bishop>(&mut file, "BISHOP")?;
    write_sliding_piece::<Rook>(&mut file, "ROOK")?;

    Ok(())
}

fn write_prelude(f: &mut fs::File) -> io::Result<()> {
    writeln!(f, "use crate::board::BoardMask;")?;
    writeln!(f)?;

    Ok(())
}

fn write_to_file<F>(f: &mut fs::File, name: &str, filter: F) -> io::Result<()>
where
    F: Fn(Square, Square) -> bool,
{
    writeln!(f, "pub const {name}: [BoardMask; 64] = [")?;

    for square in Square::iter_all() {
        let mut mask = BoardMask::default();
        for target in Square::iter_all() {
            if filter(square, target) {
                mask.set(target);
            }
        }

        writeln!(f, "    BoardMask::const_from({}),", BoardFormatter(mask),)?;
    }

    writeln!(f, "];")?;
    writeln!(f)?;

    Ok(())
}

fn write_sliding_piece<P: SlidingPiece>(f: &mut fs::File, name: &str) -> io::Result<()>
where
    [(); 1 << P::NBITS]: Sized,
{
    write_sliding_piece_occupancy::<P>(f, name)?;
    write_sliding_piece_magic::<P>(f, name)?;

    Ok(())
}

fn write_sliding_piece_occupancy<P: SlidingPiece>(f: &mut fs::File, name: &str) -> io::Result<()> {
    writeln!(f, "pub const {name}_OCCUPANCY: [BoardMask; 64] = [")?;

    for square in Square::iter_all() {
        let mask = P::relevant_occupancy(square);
        writeln!(f, "    BoardMask::const_from({}),", BoardFormatter(mask))?;
    }

    writeln!(f, "];")?;
    writeln!(f)?;

    Ok(())
}

fn write_sliding_piece_magic<P: SlidingPiece>(f: &mut fs::File, name: &str) -> io::Result<()>
where
    [(); 1 << P::NBITS]: Sized,
{
    writeln!(f, "#[allow(clippy::mistyped_literal_suffixes)]")?;
    writeln!(f, "pub const {name}_MAGICS: [u64; 64] = [")?;

    for square in Square::iter_all() {
        let (mask, collisions) = find_magic::<P>(square).expect("Could not find magic");
        let board_formatter = BoardFormatter(BoardMask::from(mask));
        writeln!(f, "    {board_formatter}, // #{collisions} collisions",)?;
    }

    writeln!(f, "];")?;
    writeln!(f)?;

    Ok(())
}

// https://www.chessprogramming.org/Looking_for_Magics#Feeding_in_Randoms
fn random_few_bits() -> u64 {
    rand::random::<u64>() & rand::random::<u64>() & rand::random::<u64>()
}

fn find_magic<P: SlidingPiece>(square: Square) -> Result<(u64, u32), ()>
where
    [(); 1 << P::NBITS]: Sized,
{
    let basic_occupancy = P::relevant_occupancy(square);
    let basic_occupancy_bits_set = basic_occupancy.count();

    // Maybe replace these with vectors, to get rid of the 'where' clauses.
    let mut occupancy_variations = [BoardMask::default(); 1 << P::NBITS];
    let mut available_moves = [BoardMask::default(); 1 << P::NBITS];

    for i in 0..(1 << basic_occupancy_bits_set) {
        occupancy_variations[i] = basic_occupancy.variation(i);
        available_moves[i] = P::moves(square, occupancy_variations[i]);
    }

    let mut collisions: u32;

    'lfm: for _ in 0..999_999 {
        collisions = 0;

        let magic = random_few_bits();
        let mut used: HashMap<usize, BoardMask> = HashMap::default();

        for i in 0..(1 << basic_occupancy_bits_set) {
            let hash = magic_hash(magic, occupancy_variations[i], P::NBITS);
            match used.entry(hash) {
                Entry::Occupied(ref v) => {
                    if v.get() != &available_moves[i] {
                        continue 'lfm;
                    }
                    collisions += 1;
                }
                Entry::Vacant(v) => {
                    v.insert(available_moves[i]);
                }
            }
        }

        return Ok((magic, collisions));
    }

    Err(())
}

struct BoardFormatter(BoardMask);

impl fmt::Display for BoardFormatter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let bytes = u64::from(self.0).to_be_bytes();
        write!(f, "0x")?;

        for (i, byte) in bytes.iter().enumerate() {
            if i != 0 {
                write!(f, "_")?;
            }

            write!(f, "{byte:02X}")?;
        }

        Ok(())
    }
}
