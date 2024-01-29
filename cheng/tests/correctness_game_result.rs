use cheng::GameResult;
use cheng::{Board, FromIntoFen};

#[test]
fn correctness_game_result_50_move_draw() {
    // https://lichess.org/analysis/fromPosition/8/2p1k3/8/1P2K3/8/8/8/8_w_-_-_98_1
    cheng::init();

    let mut board = Board::from_fen("8/2p1k3/8/1P2K3/8/8/8/8 w - - 98 1").unwrap();
    board.try_feed("e5f5").unwrap();
    board.try_feed("e7f7").unwrap();

    assert_eq!(board.result(), GameResult::Draw);
    board.try_feed("f5e5").unwrap_err();
}

#[test]
fn correctness_game_result_50_move_draw_capture() {
    // https://lichess.org/analysis/fromPosition/4k3/8/8/1p6/8/2b5/3N4/4K3_b_-_-_99_1
    cheng::init();

    let mut board = Board::from_fen("4k3/8/8/1p6/8/2b5/3N4/4K3 b - - 99 1").unwrap();
    board.try_feed("c3d2").unwrap();

    assert_eq!(board.result(), GameResult::Undecided);
}

#[test]
fn correctness_game_result_stalemate() {
    // https://lichess.org/analysis/7k/8/8/6Q1/8/8/8/4K3_w_-_-_0_1?color=white
    cheng::init();

    let mut board = Board::from_fen("7k/8/8/6Q1/8/8/8/4K3 w - - 0 1").unwrap();
    board.try_feed("g5g6").unwrap();
    assert_eq!(board.result(), GameResult::Draw);
}

#[test]
fn correctness_move_is_invalid() {
    cheng::init();

    let mut board = Board::default();
    let err = board.try_feed("e2f5");
    assert!(err.is_err());
}
