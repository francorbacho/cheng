use cheng::Board;

#[test]
fn test_perft_initial_position_0_to_4() {
    cheng::init();
    let board = Board::default();
    assert_eq!(board.perft(0), 1);
    assert_eq!(board.perft(1), 20);
    assert_eq!(board.perft(2), 400);
    assert_eq!(board.perft(3), 8902);
    assert_eq!(board.perft(4), 197_281);
}

#[test]
fn test_perft_initial_position_5() {
    cheng::init();
    let board = Board::default();
    assert_eq!(board.perft(5), 4_865_609);
}

#[test]
#[ignore = "expensive"]
fn test_perft_initial_position_6() {
    cheng::init();
    let board = Board::default();
    assert_eq!(board.perft(6), 119_060_324);
}
