use cheng::prelude::*;
use cheng::{Board, Side};

#[test]
fn test_fen_generation() {
    cheng::init();
    // https://lichess.org/pOzXhLHW/white#86
    let mut board = Board::from_fen("8/8/5r2/p1kP1p2/1p1R1K2/1P6/P7/8 w - - 4 44").unwrap();

    board.feed("f4e5".parse().unwrap()).unwrap();
    board.feed("f6f8".parse().unwrap()).unwrap();

    board.feed("d4c4".parse().unwrap()).unwrap();
    board.feed("c5b6".parse().unwrap()).unwrap();

    assert_eq!(
        board.into_fen(),
        "5r2/8/1k6/p2PKp2/1pR5/1P6/P7/8 w - - 8 46"
    );
}

#[test]
fn test_fen_parse_en_passant() {
    cheng::init();
    // https://lichess.org/CHxlnq14/white#40
    let mut board =
        Board::from_fen("4rrk1/p2q2bp/1p1pR3/2pP1p2/2Q2P2/3Pp1P1/PP4BP/4R1K1 w - c6 0 21").unwrap();
    assert_eq!(board.side(Side::Black).en_passant, Some(C6));

    board = Board::from_fen(&board.into_fen()).unwrap();
    assert_eq!(board.side(Side::Black).en_passant, Some(C6));
}
