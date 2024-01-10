use cheng::BorkedBoard;

#[must_use]
pub fn perft(board: &BorkedBoard, depth: usize) -> usize {
    if depth == 0 {
        return 1;
    }

    let moves = board.moves();
    let mut nodes = 0;
    for movement in moves {
        let mut clone = board.clone();
        clone.try_feed(movement).unwrap();
        nodes += perft(&clone, depth - 1);
    }

    nodes
}
