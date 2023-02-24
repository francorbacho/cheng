use cheng::Board;

fn perft(board: &Board, depth: usize) -> usize {
    if depth == 0 {
        return 1;
    }

    let moves = board.moves();
    let mut nodes = 0;
    for movement in moves {
        let mut clone = board.clone();
        clone.feed(movement);
        nodes += perft(&clone, depth - 1);
    }

    nodes
}

#[test]
fn test_perft_0_to_4_initial_position() {
    cheng::init();
    let board = Board::default();
    assert_eq!(perft(&board, 0), 1);
    assert_eq!(perft(&board, 1), 20);
    assert_eq!(perft(&board, 2), 400);
    assert_eq!(perft(&board, 3), 8902);
    assert_eq!(perft(&board, 4), 197_281);
}

#[test]
fn test_perft_5_initial_position() {
    cheng::init();
    let board = Board::default();
    assert_eq!(perft(&board, 5), 4_865_609);
}

#[test]
fn test_perft_6_initial_position() {
    cheng::init();
    let board = Board::default();
    assert_eq!(perft(&board, 6), 119_060_324);
}
