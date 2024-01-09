use cheng::BorkedBoard;

mod perft;
use perft::perft;

#[test]
fn test_kiwipete_0_to_3() {
    // https://www.chessprogramming.org/Perft_Results#Position_2
    cheng::init();
    let board = BorkedBoard::from_fen(
        "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1",
    )
    .unwrap();
    assert_eq!(perft(&board, 0), 1);
    assert_eq!(perft(&board, 1), 48);
    assert_eq!(perft(&board, 2), 2039);
    assert_eq!(perft(&board, 3), 97_862);
}

#[test]
#[ignore = "expensive"]
fn test_perft_kiwipete_4() {
    // https://www.chessprogramming.org/Perft_Results#Position_2
    cheng::init();
    let board = BorkedBoard::from_fen(
        "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1",
    )
    .unwrap();
    assert_eq!(perft(&board, 4), 4_085_603);
}

#[test]
fn test_position_3() {
    // https://www.chessprogramming.org/Perft_Results#Position_3
    cheng::init();
    let board = BorkedBoard::from_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1").unwrap();
    assert_eq!(perft(&board, 1), 14);
    assert_eq!(perft(&board, 2), 191);
    assert_eq!(perft(&board, 3), 2_812);
    assert_eq!(perft(&board, 4), 43_238);
    assert_eq!(perft(&board, 5), 674_624);
}

#[test]
#[ignore = "expensive"]
fn test_position_4() {
    // https://www.chessprogramming.org/Perft_Results#Position_4
    cheng::init();
    let board =
        BorkedBoard::from_fen("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1")
            .unwrap();
    assert_eq!(perft(&board, 1), 6);
    assert_eq!(perft(&board, 2), 264);
    assert_eq!(perft(&board, 3), 9_467);
    assert_eq!(perft(&board, 4), 422_333);
    assert_eq!(perft(&board, 5), 15_833_292);
}

#[test]
fn test_position_5() {
    // https://www.chessprogramming.org/Perft_Results#Position_5
    cheng::init();
    let board =
        BorkedBoard::from_fen("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8").unwrap();
    assert_eq!(perft(&board, 1), 44);
    assert_eq!(perft(&board, 2), 1_486);
    assert_eq!(perft(&board, 3), 62_379);
    assert_eq!(perft(&board, 4), 2_103_487);
}

#[test]
fn test_position_6() {
    // https://www.chessprogramming.org/Perft_Results#Position_6
    cheng::init();
    let board = BorkedBoard::from_fen(
        "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10",
    )
    .unwrap();
    assert_eq!(perft(&board, 1), 46);
    assert_eq!(perft(&board, 2), 2_079);
    assert_eq!(perft(&board, 3), 89_890);
    assert_eq!(perft(&board, 4), 3_894_594);
}
