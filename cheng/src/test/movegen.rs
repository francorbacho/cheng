use crate::{
    board::BoardMask,
    movegen::{steady, Bishop, King, PieceExt, Rook},
    square::consts::*,
};

#[test]
fn test_movegen_king() {
    let opposite_occupancy = BoardMask::default();

    let a1_moves_no_friendly_occ = King::moves(A1, BoardMask::default(), opposite_occupancy);
    assert_eq!(a1_moves_no_friendly_occ.count(), 3);

    let a1_moves_friendly_occ = King::moves(A1, BoardMask::from(A2), opposite_occupancy);
    assert_eq!(a1_moves_friendly_occ.count(), 2);
}

#[test]
fn test_movegen_rook_steady() {
    let occupancy = BoardMask::from([A2, B8, D3, E1, E3, H2, H5, H7, H8].as_slice());
    let moves = <Rook as steady::SlidingPiece>::moves(H3, occupancy);
    let moves_expected = BoardMask::from([E3, F3, G3, H2, H4, H5].as_slice());
    assert_eq!(moves, moves_expected);
}

#[test]
fn test_relevant_occ_mask_steady() {
    use steady::SlidingPiece;

    let relevant_occ_mask = Rook::relevant_occupancy(D6);
    let expected = BoardMask::from([B6, C6, E6, F6, G6, D2, D3, D4, D5, D7].as_slice());
    assert_eq!(relevant_occ_mask, expected);

    let relevant_occ_mask = Rook::relevant_occupancy(A1);
    let expected = BoardMask::from([B1, C1, D1, E1, F1, G1, A2, A3, A4, A5, A6, A7].as_slice());
    assert_eq!(relevant_occ_mask, expected);

    let relevant_occ_mask = Bishop::relevant_occupancy(H1);
    let expected = BoardMask::from([B7, C6, D5, E4, F3, G2].as_slice());
    assert_eq!(relevant_occ_mask, expected);

    let relevant_occ_mask = Bishop::relevant_occupancy(H6);
    let expected = BoardMask::from([D2, E3, F4, G5, G7].as_slice());
    assert_eq!(relevant_occ_mask, expected);

    let relevant_occ_mask = Bishop::relevant_occupancy(C5);
    let expected = BoardMask::from([B4, D4, E3, F2, B6, D6, E7].as_slice());
    assert_eq!(relevant_occ_mask, expected);
}

#[test]
fn test_movegen_rook() {
    Rook::init();

    let occupancy = BoardMask::default();
    let moves = Rook::moves(D4, BoardMask::default(), occupancy);
    let moves_expected =
        BoardMask::from([D1, D2, D3, A4, B4, C4, E4, F4, G4, H4, D5, D6, D7, D8].as_slice());
    assert_eq!(moves, moves_expected);

    let occupancy = BoardMask::from([A2, B8, D3, E1, E3, H2, H5, H7, H8].as_slice());
    let moves = Rook::moves(H3, BoardMask::default(), occupancy);
    let moves_expected: BoardMask = BoardMask::from([E3, F3, G3, H2, H4, H5].as_slice());
    assert_eq!(moves, moves_expected);
}

#[test]
fn test_movegen_bishop_steady() {
    let occupancy = BoardMask::default();

    let moves = <Bishop as steady::SlidingPiece>::moves(C7, occupancy);
    let moves_expected = BoardMask::from([H2, G3, F4, E5, D6, B8, A5, B6, D8].as_slice());
    assert_eq!(moves, moves_expected);

    let occupancy = BoardMask::from([B2, C2, D2, F4, A5, E7, F7, G7].as_slice());

    let moves = <Bishop as steady::SlidingPiece>::moves(C7, occupancy);
    let moves_expected = BoardMask::from([F4, E5, D6, B8, A5, B6, D8].as_slice());
    assert_eq!(moves, moves_expected);
}

#[test]
fn test_movegen_bishop() {
    Bishop::init();

    let occupancy = BoardMask::default();

    let moves = Bishop::moves(C7, BoardMask::default(), occupancy);
    let moves_expected = BoardMask::from([H2, G3, F4, E5, D6, B8, A5, B6, D8].as_slice());
    assert_eq!(moves, moves_expected);

    let occupancy = BoardMask::from([B2, C2, D2, F4, A5, E7, F7, G7].as_slice());

    let moves = Bishop::moves(C7, BoardMask::default(), occupancy);
    let moves_expected = BoardMask::from([F4, E5, D6, B8, A5, B6, D8].as_slice());
    assert_eq!(moves, moves_expected);
}

#[test]
fn test_movegen_cant_slide_to_friendly_occupation() {
    Rook::init();

    let friendly = BoardMask::from([A3, C1].as_slice());
    let opposite = BoardMask::from([H3, C8].as_slice());

    let moves = Rook::moves(C3, friendly, opposite);
    let moves_expected =
        BoardMask::from([B3, D3, E3, F3, G3, H3, C2, C4, C5, C6, C7, C8].as_slice());

    assert_eq!(moves, moves_expected);
}
