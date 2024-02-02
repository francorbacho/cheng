use cheng::{Board, FromIntoFen, Side};
use flimsybird::{Evaluable, Evaluation};

fn assert_wins(side: Side, fen: &str) {
    let mut board = Board::from_fen(fen).unwrap();
    let (Some(_), evaluation) = board.evaluate() else {
        unreachable!()
    };

    assert_eq!(evaluation, Evaluation::checkmate_in(side, 1), "fen {fen}");
}

#[test]
fn checkmate_in_one() {
    cheng::init();
    // https://lichess.org/analysis/5k2/8/8/4q3/8/3K4/1q6/8_b_-_-_0_1?color=white
    assert_wins(Side::Black, "5k2/8/8/4q3/8/3K4/1q6/8 b - - 0 1");

    // https://lichess.org/analysis/rnbqkbnr/pppp1ppp/4p3/8/6P1/5P2/PPPPP2P/RNBQKBNR_b_KQkq_-_0_2?color=white
    assert_wins(
        Side::Black,
        "rnbqkbnr/pppp1ppp/4p3/8/6P1/5P2/PPPPP2P/RNBQKBNR b KQkq - 0 2",
    );

    assert_wins(Side::White, "8/7k/8/1P4RP/5P2/5PP1/8/Q2K4 w - - 23 75");
}
