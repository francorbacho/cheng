use cheng::BorkedBoard;

mod perft;
use perft::perft;

#[test]
fn test_perft_initial_position_0_to_4() {
    cheng::init();
    let board = BorkedBoard::default();
    assert_eq!(perft(&board, 0), 1);
    assert_eq!(perft(&board, 1), 20);
    assert_eq!(perft(&board, 2), 400);
    assert_eq!(perft(&board, 3), 8902);
    assert_eq!(perft(&board, 4), 197_281);
}

#[test]
fn test_perft_initial_position_5() {
    cheng::init();
    let board = BorkedBoard::default();
    assert_eq!(perft(&board, 5), 4_865_609);
}

#[test]
#[ignore = "expensive"]
fn test_perft_initial_position_6() {
    cheng::init();
    let board = BorkedBoard::default();
    assert_eq!(perft(&board, 6), 119_060_324);
}
