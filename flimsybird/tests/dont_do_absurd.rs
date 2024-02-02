use cheng::{Board, FromIntoFen, PseudoMove};
use flimsybird::Evaluable;
use std::convert::TryInto;

fn run(fen: &str) -> PseudoMove {
    let mut board = Board::from_fen(fen).unwrap();
    let (Some(bm), _) = board.evaluate() else {
        unreachable!()
    };

    PseudoMove::from(bm)
}

#[test]
fn absurd_dont_yeet_rook() {
    cheng::init();
    let bm = run("8/3P4/8/4R1P1/5PK1/3r4/5k2/8 w - - 3 49");
    assert_ne!(bm, "e5e2".try_into().unwrap());

    let bm = run("8/6P1/8/3P1k1K/8/1r6/8/5R2 b - - 2 97");
    assert_ne!(bm, "b3f3".try_into().unwrap());
}
