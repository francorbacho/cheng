use crate::{
    board::{Board, GameResult},
    sides::Side,
};

#[test]
fn test_checkmate_fast() {
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
    todo!()
}
