use super::pieces::{Bishop, Rook};

use crate::{board::BoardMask, square::Square};

fn filter_squares<F>(discriminant: F) -> BoardMask
where
    F: Fn(Square) -> bool,
{
    let mut result = BoardMask::default();
    for target in Square::iter_all() {
        if discriminant(target) {
            result.set(target);
        }
    }
    result
}

pub trait SlidingPiece {
    const NBITS: u32;

    fn relevant_occupancy(square: Square) -> BoardMask;
    fn moves(square: Square, occupancy: BoardMask) -> BoardMask;
}

impl SlidingPiece for Rook {
    #[cfg(release)]
    const NBITS: u32 = 12;

    #[cfg(not(release))]
    const NBITS: u32 = 14;

    fn relevant_occupancy(square: Square) -> BoardMask {
        filter_squares(|target| match (target.rank(), target.file()) {
            (0 | 7, 0 | 7) => false,
            (_, 0 | 7) => target.file::<usize>() == square.file(),
            (0 | 7, _) => target.rank::<usize>() == square.rank(),
            (rank, file) => (rank == square.rank()) ^ (file == square.file()),
        })
    }

    fn moves(square: Square, occupancy: BoardMask) -> BoardMask {
        let (rank, file) = (square.rank(), square.file());
        let mut result = BoardMask::default();

        for i in (0..file).rev() {
            let target = Square::from_rank_file(rank, i);
            result.set(target);
            if occupancy.get(target) {
                break;
            }
        }

        for i in (file + 1)..8 {
            let target = Square::from_rank_file(rank, i);
            result.set(target);
            if occupancy.get(target) {
                break;
            }
        }

        for i in (0..rank).rev() {
            let target = Square::from_rank_file(i, file);
            result.set(target);
            if occupancy.get(target) {
                break;
            }
        }

        for i in (rank + 1)..8 {
            let target = Square::from_rank_file(i, file);
            result.set(target);
            if occupancy.get(target) {
                break;
            }
        }

        result
    }
}

impl SlidingPiece for Bishop {
    #[cfg(release)]
    const NBITS: u32 = 9;

    #[cfg(not(release))]
    const NBITS: u32 = 14;

    fn relevant_occupancy(square: Square) -> BoardMask {
        let sqrank = square.rank::<i32>();
        let sqfile = square.file::<i32>();
        filter_squares(|target| match (target.rank(), target.file()) {
            (_, 0 | 7) | (0 | 7, _) => false,
            (rank, file) => (sqrank - rank).abs() == (sqfile - file).abs() && sqrank != rank,
        })
    }

    fn moves(square: Square, occupancy: BoardMask) -> BoardMask {
        let (rank, file): (i32, i32) = (square.rank(), square.file());
        let mut result = BoardMask::default();

        let distance_to_top = 8 - rank;
        let distance_to_bottom = rank + 1;
        let distance_to_right = 8 - file;
        let distance_to_left = file + 1;

        // goes top-right
        for i in 1..distance_to_right.min(distance_to_top) {
            let target = Square::from_rank_file(rank + i, file + i);
            result.set(target);
            if occupancy.get(target) {
                break;
            }
        }

        // goes top-left
        for i in 1..distance_to_left.min(distance_to_top) {
            let target = Square::from_rank_file(rank + i, file - i);
            result.set(target);
            if occupancy.get(target) {
                break;
            }
        }

        // goes bottom-right
        for i in 1..distance_to_right.min(distance_to_bottom) {
            let target = Square::from_rank_file(rank - i, file + i);
            result.set(target);
            if occupancy.get(target) {
                break;
            }
        }

        // goes bottom-left
        for i in 1..distance_to_left.min(distance_to_bottom) {
            let target = Square::from_rank_file(rank - i, file - i);
            result.set(target);
            if occupancy.get(target) {
                break;
            }
        }

        result
    }
}
