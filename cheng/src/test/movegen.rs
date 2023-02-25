use crate::{
    board::BoardMask,
    movegen::{self, steady, Bishop, King, PieceExt, Rook},
    movement::{Castle, MoveKind},
    side_state::CastlingRights,
    square::consts::*,
    Board, Piece, Side,
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

#[test]
fn test_en_passant_as_white() {
    // https://lichess.org/analysis/8/5Kpk/8/5P2/8/8/8/8_b_-_-_0_1?color=white
    let mut board = Board::from_fen("8/5Kpk/8/5P2/8/8/8/8 b - - 0 1").unwrap();
    board.feed("g7g5".parse().unwrap());

    let contains_en_passant_capture = board
        .moves()
        .collect::<Vec<_>>()
        .contains(&"f5g6".parse().unwrap());
    assert!(contains_en_passant_capture);

    board.feed("f5g6".parse().unwrap());
    assert!(board.side(Side::Black).king_in_check);

    // Make sure the occupancy is right: the pawn is taken.
    assert_eq!(board.side(Side::Black).occupancy, BoardMask::from(H7));
}

#[test]
fn test_en_passant_as_black() {
    // https://lichess.org/analysis/8/8/8/2k5/1p6/3K4/2P5/8_w_-_-_0_1?color=white
    let mut board = Board::from_fen("8/8/8/2k5/1p6/3K4/2P5/8 w - - 0 1").unwrap();
    board.feed("c2c4".parse().unwrap());

    let contains_en_passant_capture = board
        .moves()
        .collect::<Vec<_>>()
        .contains(&"b4c3".parse().unwrap());

    assert!(contains_en_passant_capture);

    board.feed("b4c3".parse().unwrap());

    // Make sure the occupancy is right: the pawn is taken.
    assert_eq!(board.side(Side::White).occupancy, BoardMask::from(D3));
    assert_eq!(
        board.side(Side::White).pieces.piece(Piece::Pawn),
        BoardMask::default()
    );

    assert_eq!(
        board.side(Side::Black).occupancy,
        BoardMask::from([C3, C5].as_slice())
    );
}

#[test]
fn test_only_pawns_take_en_passant() {
    // https://lichess.org/analysis/8/8/8/8/5k2/8/6P1/4K3_w_-_-_0_1?color=white
    let mut board = Board::from_fen("8/8/8/8/5k2/8/6P1/4K3 w - - 0 1").unwrap();
    board.feed("g2g4".parse().unwrap());
    board.feed("f4g3".parse().unwrap());

    assert_eq!(
        board.side(Side::White).occupancy,
        BoardMask::from([E1, G4].as_slice())
    );

    assert_eq!(board.side(Side::Black).occupancy, BoardMask::from(G3));
}

#[test]
fn test_castling() {
    crate::init();

    // https://lichess.org/analysis/4k3/8/8/8/2r5/8/1R6/R3K2R_w_KQ_-_0_1?color=white
    let mut board = Board::from_fen("4k3/8/8/8/2r5/8/1R6/R3K2R w KQ - 0 1").unwrap();

    fn board_contains_castle(board: &Board, castle_kind: Castle) -> bool {
        board
            .moves()
            .find(|movement| movement.kind == MoveKind::Castle(castle_kind))
            .is_some()
    }

    board.white_side.update_threats(&board.black_side);
    board.black_side.update_threats(&board.white_side);

    assert!(board_contains_castle(&board, Castle::KingSide));
    assert!(!board_contains_castle(&board, Castle::QueenSide));

    board.feed("b2g2".parse().unwrap());
    board.feed("c4g4".parse().unwrap());

    assert!(board_contains_castle(&board, Castle::KingSide));
    assert!(board_contains_castle(&board, Castle::QueenSide));

    board.feed("e1g1".parse().unwrap());

    assert_eq!(
        board.white_side.occupancy,
        BoardMask::from([A1, F1, G1, G2].as_slice())
    );

    assert_eq!(
        board.white_side.pieces.piece(Piece::Rook),
        BoardMask::from([A1, F1, G2].as_slice())
    );

    assert_eq!(board.white_side.castling_rights, CastlingRights::None);
}
