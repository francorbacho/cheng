use crate::{
    board::{Board, GameResult},
    movegen::{Bishop, PieceExt, Rook},
    sides::Side,
};

#[test]
fn test_simple_queen_check() {
    crate::init();

    let mut board = Board::default();
    board.feed("e2e4".parse().unwrap());
    board.feed("e7e5".parse().unwrap());

    board.feed("d1f3".parse().unwrap());
    board.feed("a7a6".parse().unwrap());

    board.feed("f3f7".parse().unwrap());

    assert!(board.black_side.king_in_check);
    assert_eq!(board.result(), None);

    board.feed("e8f7".parse().unwrap());

    assert!(!board.black_side.king_in_check);
    assert_eq!(board.result(), None);
}

#[test]
fn test_checkmate_fast() {
    crate::init();

    // Scholar's mate.
    let mut board = Board::default();
    board.feed("e2e4".parse().unwrap());
    board.feed("e7e5".parse().unwrap());

    board.feed("d1h5".parse().unwrap());
    board.feed("b8c6".parse().unwrap());

    board.feed("f1c4".parse().unwrap());
    board.feed("g8f6".parse().unwrap());

    board.feed("h5f7".parse().unwrap());

    assert_eq!(
        board.result(),
        Some(GameResult::Checkmate {
            winner: Side::White
        })
    );

    // Fool's mate.
    let mut board = Board::default();
    board.feed("f2f3".parse().unwrap());
    board.feed("e7e5".parse().unwrap());

    board.feed("g2g4".parse().unwrap());
    board.feed("d8h4".parse().unwrap());

    assert_eq!(
        board.result(),
        Some(GameResult::Checkmate {
            winner: Side::Black
        })
    );
}
