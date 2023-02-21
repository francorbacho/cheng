use self::{
    hash::magic_hash,
    precomputed::{PAWN_CAPTURES, PAWN_MOVES},
};
use crate::{board::BoardMask, pieces::Piece, square::Square};

pub use pieces::{Bishop, King, Knight, Pawn, Rook};

mod hash;
mod pieces;
mod precomputed;
pub(crate) mod steady;

pub static mut ROOK_MOVES: [[BoardMask; 1 << <Rook as steady::SlidingPiece>::NBITS]; 64] =
    [[BoardMask::const_from(0); 1 << <Rook as steady::SlidingPiece>::NBITS]; 64];

pub static mut BISHOP_MOVES: [[BoardMask; 1 << <Bishop as steady::SlidingPiece>::NBITS]; 64] =
    [[BoardMask::const_from(0); 1 << <Bishop as steady::SlidingPiece>::NBITS]; 64];

pub fn moves(piece: Piece, square: Square, friendly: BoardMask, opposite: BoardMask) -> BoardMask {
    match piece {
        Piece::Pawn => Pawn::moves(square, friendly, opposite),
        Piece::Knight => Knight::moves(square, friendly, opposite),
        Piece::Bishop => Bishop::moves(square, friendly, opposite),
        Piece::Rook => Rook::moves(square, friendly, opposite),
        Piece::Queen => Rook::moves(square, friendly, opposite)
            .intersection(Bishop::moves(square, friendly, opposite)),
        Piece::King => King::moves(square, friendly, opposite),
    }
}

pub fn threats(
    piece: Piece,
    square: Square,
    friendly: BoardMask,
    opposite: BoardMask,
) -> BoardMask {
    match piece {
        Piece::Pawn => Pawn::threats(square, friendly, opposite),
        Piece::Knight => Knight::threats(square, friendly, opposite),
        Piece::Bishop => Bishop::threats(square, friendly, opposite),
        Piece::Rook => Rook::threats(square, friendly, opposite),
        Piece::Queen => Rook::threats(square, friendly, opposite)
            .intersection(Bishop::threats(square, friendly, opposite)),
        Piece::King => King::threats(square, friendly, opposite),
    }
}

pub trait PieceExt {
    fn init() {}

    fn moves(square: Square, friendly: BoardMask, opposite: BoardMask) -> BoardMask {
        Self::threats(square, friendly, opposite).without(friendly)
    }

    fn threats(square: Square, friendly: BoardMask, opposite: BoardMask) -> BoardMask;
}

impl PieceExt for King {
    fn threats(square: Square, _friendly: BoardMask, _opposite: BoardMask) -> BoardMask {
        precomputed::KING_MOVES[square.to_index()]
    }
}

impl PieceExt for Knight {
    fn threats(square: Square, _friendly: BoardMask, _opposite: BoardMask) -> BoardMask {
        precomputed::KNIGHT_MOVES[square.to_index()]
    }
}

impl PieceExt for Pawn {
    fn threats(square: Square, _friendly: BoardMask, _opposite: BoardMask) -> BoardMask {
        PAWN_CAPTURES[square.to_index()]
    }

    fn moves(square: Square, friendly: BoardMask, opposite: BoardMask) -> BoardMask {
        let captures = PAWN_CAPTURES[square.to_index()].only(opposite);
        let moves = PAWN_MOVES[square.to_index()];
        let occupancy = friendly.intersection(opposite);

        let occupancy_next_rank_mask = square
            .checked_next_rank()
            .map(BoardMask::from)
            .unwrap_or_default()
            .only(occupancy);

        let occupancy_allows_two_square_move_mask = occupancy_next_rank_mask.push_rank();

        moves
            .without(occupancy)
            .without(occupancy_allows_two_square_move_mask)
            .intersection(captures)
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

    fn threats(square: Square, friendly: BoardMask, opposite: BoardMask) -> BoardMask {
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

    fn threats(square: Square, friendly: BoardMask, opposite: BoardMask) -> BoardMask {
        let index = square.to_index();
        let occupancy = precomputed::BISHOP_OCCUPANCY[index].only(friendly.intersection(opposite));
        let hash = magic_hash(
            precomputed::BISHOP_MAGICS[index],
            occupancy,
            <Bishop as steady::SlidingPiece>::NBITS,
        );

        unsafe { BISHOP_MOVES[index][hash] }
    }
}
