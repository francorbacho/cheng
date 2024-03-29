use cheng::{Board, FromIntoFen};

#[test]
fn perft_castling() {
    cheng::init();

    // https://lichess.org/analysis/4k3/8/8/8/8/8/8/Rb2K2R_w_KQ_-_0_1?color=white
    let board = Board::from_fen("4k3/8/8/8/8/8/8/Rb2K2R w KQ - 0 1").unwrap();

    assert_eq!(board.perft(1), 23);
    assert_eq!(board.perft(2), 241);
    assert_eq!(board.perft(3), 6_406);
    assert_eq!(board.perft(4), 77_181);
    assert_eq!(board.perft(5), 2_133_419);
}

#[test]
fn perft_promotion() {
    // https://lichess.org/analysis/4k3/P7/8/8/8/8/8/4K3_w_-_-_0_1
    cheng::init();
    let board = Board::from_fen("4k3/P7/8/8/8/8/8/4K3 w - - 0 1").unwrap();
    assert_eq!(board.perft(3), 500);
    assert_eq!(board.perft(4), 2_994);
    assert_eq!(board.perft(5), 44_913);
}

#[test]
fn perft_promotion_both_sides_can_promote() {
    // https://lichess.org/analysis/4k3/PP6/8/8/8/8/p7/4K3_w_-_-_0_1
    cheng::init();
    let board = Board::from_fen("4k3/PP6/8/8/8/8/p7/4K3 w - - 0 1").unwrap();
    assert_eq!(board.perft(6), 2_066_895);
}
