use cheng::Board;
use cheng::GameResult;

#[test]
fn test_50_move_draw() {
    // https://lichess.org/analysis/fromPosition/8/2p1k3/8/1P2K3/8/8/8/8_w_-_-_98_1
    cheng::init();

    let mut board = Board::from_fen("8/2p1k3/8/1P2K3/8/8/8/8 w - - 98 1").unwrap();
    board.feed("e5f5".parse().unwrap()).unwrap();
    board.feed("e7f7".parse().unwrap()).unwrap();

    assert_eq!(board.result(), Some(GameResult::Draw));
    board.feed("f5e5".parse().unwrap()).unwrap_err();
}
