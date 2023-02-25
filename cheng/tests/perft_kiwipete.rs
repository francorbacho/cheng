use cheng::Board;

mod perft;
use perft::perft;

#[test]
fn test_kiwipete_0_to_3() {
    // https://www.chessprogramming.org/Perft_Results#Position_2
    cheng::init();
    let board =
        Board::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1")
            .unwrap();
    assert_eq!(perft(&board, 0), 1);
    assert_eq!(perft(&board, 1), 48);
    assert_eq!(perft(&board, 2), 2039);
    assert_eq!(perft(&board, 3), 97_862);
}

#[test]
#[ignore = "doesn't pass yet"]
fn test_perft_kiwipete_4() {
    // https://www.chessprogramming.org/Perft_Results#Position_2
    cheng::init();
    let board =
        Board::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1")
            .unwrap();
    assert_eq!(perft(&board, 4), 4_085_603);
}
