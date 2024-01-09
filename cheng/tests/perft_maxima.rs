mod perft;
use perft::perft;

use cheng::BorkedBoard;

#[test]
fn perft_maxima() {
    cheng::init();

    // https://www.chessprogramming.org/Chess#Chess_Maxima
    let board =
        BorkedBoard::from_fen("R6R/3Q4/1Q4Q1/4Q3/2Q4Q/Q4Q2/pp1Q4/kBNN1KB1 w - - 0 1").unwrap();
    assert_eq!(perft(&board, 1), 218)
}
