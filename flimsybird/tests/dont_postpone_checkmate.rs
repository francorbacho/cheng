use cheng::Board;
use flimsybird::{Evaluable, Evaluation};

#[test]
fn checkmate_in_one() {
    cheng::init();
    // https://lichess.org/analysis/5k2/8/8/4q3/8/3K4/1q6/8_b_-_-_0_1?color=white
    let mut board = Board::from_fen("5k2/8/8/4q3/8/3K4/1q6/8 b - - 0 1").unwrap();
    let (Some(_), evaluation) = board.evaluate() else {
        unreachable!()
    };

    assert_eq!(evaluation, Evaluation::BLACK_WIN);
}
