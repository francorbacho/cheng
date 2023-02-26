use cheng::Board;

mod perft;
use perft::perft;

#[test]
fn perft_castling() {
    cheng::init();

    // https://lichess.org/analysis/4k3/8/8/8/8/8/8/Rb2K2R_w_KQ_-_0_1?color=white
    let board = Board::from_fen("4k3/8/8/8/8/8/8/Rb2K2R w KQ - 0 1").unwrap();

    assert_eq!(perft(&board, 1), 23);
    assert_eq!(perft(&board, 2), 241);
    assert_eq!(perft(&board, 3), 6_406);
    assert_eq!(perft(&board, 4), 77_181);
    assert_eq!(perft(&board, 5), 2_133_419);
}
