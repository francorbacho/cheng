use cheng::Board;

#[must_use]
pub fn perft(board: &Board, depth: usize) -> usize {
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
