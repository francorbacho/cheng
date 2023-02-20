use cheng::Board;

fn perft(board: &Board, depth: usize) -> usize {
    let moves = board.moves();

    if depth == 0 {
        return moves.count();
    }

    let mut nodes = 0;
    for movement in moves {
        let mut clone = board.clone();
        clone.feed(movement);
        nodes += perft(board, depth - 1);
    }

    nodes
}

#[test]
fn perft_position_initial() {
    let board = Board::default();
    assert_eq!(perft(&board, 0), 20);
}
