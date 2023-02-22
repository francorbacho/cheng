use crate::{
    board::BoardMask,
    movegen::{self, steady, Bishop, King, PieceExt, Rook},
    square::consts::*,
    Board, Side,
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
fn test_movegen_pawn() {
    use Side::*;
    macro_rules! f {
        [$($squares:expr),*] => {
            [$($squares),*].as_slice()
        }
    }

    #[rustfmt::skip]
    let movegen_test = [
        (White, E2, f![],   f![],       f![E3, E4]),
        (White, E2, f![],   f![E3],     f![]),
        (White, F2, f![],   f![F4],     f![F3]),
        (White, H2, f![H3], f![G3, G4], f![G3]),
        (White, B7, f![B8], f![C8],     f![C8]),
        (Black, C7, f![],   f![],       f![C6, C5]),
        (Black, B5, f![],   f![],       f![B4]),
        (Black, A2, f![],   f![B1],     f![A1, B1]),
    ];

    for entry in &movegen_test {
        let side = entry.0;
        let square = entry.1;
        let friendly_occupancy = BoardMask::from(entry.2);
        let opposite_occupancy = BoardMask::from(entry.3);
        let expected_moves = BoardMask::from(entry.4);

        eprintln!("{side:?},{square:?}");
        eprintln!("{}", BoardMask::from(0x2000000));

        let moves = movegen::pawn_moves(side, square, friendly_occupancy, opposite_occupancy);
        assert_eq!(moves, expected_moves);
    }

    let moves = movegen::pawn_threats(Side::White, E2);
    let moves_expected = BoardMask::from([D3, F3].as_slice());
    assert_eq!(moves, moves_expected);
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
    crate::init();

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
    crate::init();

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
    crate::init();

    let friendly = BoardMask::from([A3, C1].as_slice());
    let opposite = BoardMask::from([H3, C8].as_slice());

    let moves = Rook::moves(C3, friendly, opposite);
    let moves_expected =
        BoardMask::from([B3, D3, E3, F3, G3, H3, C2, C4, C5, C6, C7, C8].as_slice());

    assert_eq!(moves, moves_expected);
}

#[test]
fn test_movegen_king_cant_move_to_threaten() {
    crate::init();

    let mut board = Board::default();
    board.feed("b2b3".parse().unwrap());
    board.feed("e7e5".parse().unwrap());

    board.feed("c1a3".parse().unwrap());
    let ok = !board
        .moves()
        .collect::<Vec<_>>()
        .contains(&"e8e7".parse().unwrap());

    assert!(ok);
}
