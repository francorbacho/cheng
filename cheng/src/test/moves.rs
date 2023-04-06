use std::str::FromStr;

use crate::{
    board::{Board, BoardMask},
    movement::{MoveKind, MoveParseError, PseudoMove},
    pieces::Piece,
    square::prelude::*,
};

#[test]
fn test_move_parsing() {
    assert_eq!(
        PseudoMove::from_str("e2e4").expect("Error parsing"),
        PseudoMove {
            origin: E2,
            destination: E4,
            kind: MoveKind::Move,
        }
    );

    assert_eq!(
        PseudoMove::from_str("a7xb8").unwrap_err(),
        MoveParseError::WrongDestinationSquare
    );

    assert_eq!(
        PseudoMove::from_str("g7g8q").expect("Error parsing"),
        PseudoMove {
            origin: G7,
            destination: G8,
            kind: MoveKind::Promote(Piece::Queen),
        }
    );

    assert_eq!(
        PseudoMove::from_str("c7xb8n").unwrap_err(),
        MoveParseError::WrongDestinationSquare
    );

    assert_eq!(
        PseudoMove::from_str("c7x8n").unwrap_err(),
        MoveParseError::WrongDestinationSquare
    );

    assert_eq!(
        PseudoMove::from_str("e4").unwrap_err(),
        MoveParseError::TooShort
    );
}

#[test]
fn test_move_simple_opening() {
    let mut board = Board::default();

    board.feed("e2e4".parse().unwrap()).unwrap();
    board.feed("c7c5".parse().unwrap()).unwrap();

    assert_eq!(
        board.white_side.occupancy,
        BoardMask::from(&[A1, A2, B1, B2, C1, C2, D1, D2, E1, E4, F1, F2, G1, G2, H1, H2][..])
    );

    assert_eq!(
        board.white_side.pieces.piece(Piece::Pawn),
        BoardMask::from(&[A2, B2, C2, D2, E4, F2, G2, H2][..])
    );

    assert_eq!(
        board.black_side.occupancy,
        BoardMask::from(&[A7, A8, B7, B8, C5, C8, D7, D8, E7, E8, F7, F8, G7, G8, H7, H8][..])
    );

    assert_eq!(
        board.black_side.pieces.piece(Piece::Pawn),
        BoardMask::from(&[A7, B7, C5, D7, E7, F7, G7, H7][..])
    );
}

#[test]
fn test_move_promotion() {
    let mut board = Board::from_fen("8/6P1/6K1/8/8/5k2/5p2/8 w - - 0 1").unwrap();
    board.feed("g7g8q".parse().unwrap()).unwrap();
    board.feed("f2f1r".parse().unwrap()).unwrap();

    assert_eq!(
        board.white_side.pieces.piece(Piece::Pawn),
        BoardMask::default()
    );

    assert_eq!(
        board.black_side.pieces.piece(Piece::Pawn),
        BoardMask::default()
    );

    assert_eq!(
        board.white_side.pieces.piece(Piece::Queen),
        BoardMask::from(G8),
    );

    assert_eq!(
        board.black_side.pieces.piece(Piece::Rook),
        BoardMask::from(F1),
    );
}
