use cheng::Board;
use cheng::GameResult;

#[test]
fn test_fen_generation() {
    cheng::init();
    // https://lichess.org/pOzXhLHW/white#86
    let mut board = Board::from_fen("8/8/5r2/p1kP1p2/1p1R1K2/1P6/P7/8 w - - 4 44").unwrap();

    board.feed("f4e5".parse().unwrap()).unwrap();
    board.feed("f6f8".parse().unwrap()).unwrap();

    board.feed("d4c4".parse().unwrap()).unwrap();
    board.feed("c5b6".parse().unwrap()).unwrap();

    assert_eq!(board.into_fen(), "5r2/8/1k6/p2PKp2/1pR5/1P6/P7/8 w - - 8 1");
}
