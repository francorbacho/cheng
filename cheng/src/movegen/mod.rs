use self::hash::magic_hash;
use crate::{board::BoardMask, square::Square};

pub use pieces::{Bishop, King, Knight, Rook};

mod hash;
mod pieces;
mod precomputed;
pub(crate) mod steady;

pub static mut ROOK_MOVES: [[BoardMask; 1 << <Rook as steady::SlidingPiece>::NBITS]; 64] =
    [[BoardMask::const_from(0); 1 << <Rook as steady::SlidingPiece>::NBITS]; 64];

pub static mut BISHOP_MOVES: [[BoardMask; 1 << <Bishop as steady::SlidingPiece>::NBITS]; 64] =
    [[BoardMask::const_from(0); 1 << <Bishop as steady::SlidingPiece>::NBITS]; 64];

pub trait PieceExt {
    fn init() {}

    fn moves(square: Square, friendly: BoardMask, opposite: BoardMask) -> BoardMask;
}

impl PieceExt for King {
    fn moves(square: Square, friendly: BoardMask, _opposite: BoardMask) -> BoardMask {
        precomputed::KING_MOVES[square.to_index()].without(friendly)
    }
}

impl PieceExt for Knight {
    fn moves(square: Square, friendly: BoardMask, _opposite: BoardMask) -> BoardMask {
        precomputed::KNIGHT_MOVES[square.to_index()].without(friendly)
    }
}

impl PieceExt for Rook {
    fn init() {
        for square in Square::iter_all() {
            let index = square.to_index();
            let relevant_occupancy = precomputed::ROOK_OCCUPANCY[index];
            let occupancy_variations = relevant_occupancy.variations();
            let magic = precomputed::ROOK_MAGICS[index];
            for i in 0..occupancy_variations {
                let occupancy = relevant_occupancy.variation(i);
                let hash = magic_hash(magic, occupancy, <Rook as steady::SlidingPiece>::NBITS);

                unsafe {
                    let moves = <Rook as steady::SlidingPiece>::moves(square, occupancy);
                    let collision = ROOK_MOVES[index][hash] != BoardMask::default()
                        && ROOK_MOVES[index][hash] != moves;
                    assert!(!collision);
                    ROOK_MOVES[index][hash] = moves;
                }
            }
        }
    }

    fn moves(square: Square, friendly: BoardMask, opposite: BoardMask) -> BoardMask {
        let index = square.to_index();
        let occupancy = precomputed::ROOK_OCCUPANCY[index].only(friendly.intersection(opposite));
        let hash = magic_hash(
            precomputed::ROOK_MAGICS[index],
            occupancy,
            <Rook as steady::SlidingPiece>::NBITS,
        );

        unsafe { ROOK_MOVES[index][hash] }
    }
}

impl PieceExt for Bishop {
    fn init() {
        for square in Square::iter_all() {
            let index = square.to_index();
            let relevant_occupancy = precomputed::BISHOP_OCCUPANCY[index];
            let occupancy_variations = relevant_occupancy.variations();
            let magic = precomputed::BISHOP_MAGICS[index];
            for i in 0..occupancy_variations {
                let occupancy = relevant_occupancy.variation(i);
                let hash = magic_hash(magic, occupancy, <Bishop as steady::SlidingPiece>::NBITS);

                unsafe {
                    let moves = <Bishop as steady::SlidingPiece>::moves(square, occupancy);
                    let collision = BISHOP_MOVES[index][hash] != BoardMask::default()
                        && BISHOP_MOVES[index][hash] != moves;
                    assert!(!collision);
                    BISHOP_MOVES[index][hash] = moves;
                }
            }
        }
    }

    fn moves(square: Square, friendly: BoardMask, opposite: BoardMask) -> BoardMask {
        let index = square.to_index();
        let occupancy = precomputed::ROOK_OCCUPANCY[index].only(friendly.intersection(opposite));
        let hash = magic_hash(
            precomputed::BISHOP_MAGICS[index],
            occupancy,
            <Bishop as steady::SlidingPiece>::NBITS,
        );

        unsafe { BISHOP_MOVES[index][hash] }
    }
}
