use cheng::prelude::*;
use cheng::PseudoMove;
use cheng::{BorkedBoard, FromIntoFen};
use cheng::{Castle, MoveKind};

#[test]
fn test_try_feed_invalid_castle() {
    cheng::init();
    // https://lichess.org/analysis/rnb1k1nr/pppp1ppp/4pq2/8/1bB1P3/5N2/PPPP1PPP/RNBQK2R_w_-_-
    let board =
        BorkedBoard::from_fen("rnb1k1nr/pppp1ppp/4pq2/8/1bB1P3/5N2/PPPP1PPP/RNBQK2R w - - 4 4")
            .unwrap();
    assert!(!board.is_move_valid(PseudoMove {
        origin: E1,
        destination: G1,
        kind: MoveKind::Move
    }));
    assert!(!board.is_move_valid(PseudoMove {
        origin: E1,
        destination: G1,
        kind: MoveKind::Castle(Castle::KingSide)
    }));

    // https://lichess.org/analysis/rnb1k1nr/pppp1ppp/4pq2/8/1bB1P3/5N2/PPPP1PPP/RNBQK2R_w_KQkq_-
    let board =
        BorkedBoard::from_fen("rnb1k1nr/pppp1ppp/4pq2/8/1bB1P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 4 4")
            .unwrap();
    assert!(board.is_move_valid(PseudoMove {
        origin: E1,
        destination: G1,
        kind: MoveKind::Move
    }));
    assert!(board.is_move_valid(PseudoMove {
        origin: E1,
        destination: G1,
        kind: MoveKind::Castle(Castle::KingSide)
    }));
}
