use crate::{
    board::{Board, GameResult},
    sides::Side,
};

#[test]
fn test_simple_queen_check() {
    crate::init();

    let mut board = Board::default();
    board.try_feed("e2e4").unwrap();
    board.try_feed("e7e5").unwrap();

    board.try_feed("d1f3").unwrap();
    board.try_feed("a7a6").unwrap();

    board.try_feed("f3f7").unwrap();

    assert!(board.black_side.king_in_check);
    assert_eq!(board.result(), None);

    board.try_feed("e8f7").unwrap();

    assert!(!board.black_side.king_in_check);
    assert_eq!(board.result(), None);
}

#[test]
fn test_checkmate_fast() {
    crate::init();

    // Scholar's mate.
    let mut board = Board::default();
    board.try_feed("e2e4").unwrap();
    board.try_feed("e7e5").unwrap();

    board.try_feed("d1h5").unwrap();
    board.try_feed("b8c6").unwrap();

    board.try_feed("f1c4").unwrap();
    board.try_feed("g8f6").unwrap();

    board.try_feed("h5f7").unwrap();

    assert_eq!(
        board.result(),
        Some(GameResult::Checkmate {
            winner: Side::White
        })
    );

    // Fool's mate.
    let mut board = Board::default();
    board.try_feed("f2f3").unwrap();
    board.try_feed("e7e5").unwrap();

    board.try_feed("g2g4").unwrap();
    board.try_feed("d8h4").unwrap();

    assert_eq!(
        board.result(),
        Some(GameResult::Checkmate {
            winner: Side::Black
        })
    );
}
