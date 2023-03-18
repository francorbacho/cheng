use cheng::Board;

mod perft;
use perft::perft;

#[test]
fn perft_promotion() {
    // https://lichess.org/analysis/4k3/P7/8/8/8/8/8/4K3_w_-_-_0_1
    cheng::init();
    let board = Board::from_fen("4k3/P7/8/8/8/8/8/4K3 w - - 0 1").unwrap();
    assert_eq!(perft(&board, 3), 500);
    assert_eq!(perft(&board, 4), 2_994);
    assert_eq!(perft(&board, 5), 44_913);
}

#[test]
fn perft_promotion_both_sides_can_promote() {
    // https://lichess.org/analysis/4k3/PP6/8/8/8/8/p7/4K3_w_-_-_0_1
    cheng::init();
    let board = Board::from_fen("4k3/PP6/8/8/8/8/p7/4K3 w - - 0 1").unwrap();
    assert_eq!(perft(&board, 6), 2_066_895);
}
