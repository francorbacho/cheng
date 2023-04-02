mod checkmate;
mod movegen;
mod moves;

use crate::{
    board::{Board, BoardMask, FENParsingError},
    pieces::Piece,
    side_state::SideState,
    sides::Side,
    square::prelude::*,
    square::Square,
};

#[test]
fn test_square_prelude() {
    assert_eq!(A1, Square::from_rank_file(0, 0));
    assert_eq!(B2, Square::from_rank_file(1, 1));
    assert_eq!(A8, Square::from_rank_file(7, 0));
    assert_eq!(G6, Square::from_rank_file(5, 6));
}

#[test]
fn test_square_correct_index() {
    let a1: Square = "a1".parse().unwrap();
    assert_eq!(a1.to_index(), 0);
    assert_eq!(a1, Square::from_rank_file(0, 0));

    let h1: Square = "h1".parse().unwrap();
    assert_eq!(h1.to_index(), 7);
    assert_eq!(h1, Square::from_rank_file(0, 7));

    let a2: Square = "a2".parse().unwrap();
    assert_eq!(a2.to_index(), 8);
    assert_eq!(a2, Square::from_rank_file(1, 0));
    assert_eq!(a2, Square::from_index(8));

    let a8: Square = "a8".parse().unwrap();
    assert_eq!(a8.to_index(), 8 * 7);
    assert_eq!(a8, Square::from_rank_file(7, 0));
    assert_eq!(a8, Square::from_index(8 * 7));
}

#[test]
fn test_impl_debug_square() {
    let c7 = Square::from_index(50);
    assert_eq!(&format!("{c7:?}"), "c7");

    let b6: Square = "b6".parse().unwrap();
    assert_eq!(&format!("{b6:?}"), "b6");

    let h8: Square = "h8".parse().unwrap();
    assert_eq!(&format!("{h8:?}"), "h8");
}

#[test]
fn test_correct_next_rank() {
    assert_eq!(A1.next_rank(Side::White), A2);
    assert_eq!(H7.next_rank(Side::White), H8);
    assert_eq!(F5.next_rank(Side::White), F6);

    assert_eq!(F8.checked_next_rank(Side::White), None);
    assert_eq!(G2.checked_next_rank(Side::White), Some(G3));
}

#[test]
fn test_occupancy_side_pieces_match() {
    let mut side_pieces = SideState::empty(Side::White);
    assert_eq!(side_pieces.occupancy, BoardMask::default());

    side_pieces.put(A2, Piece::Pawn);
    const A2_MASK: BoardMask = BoardMask::const_from(1 << 8);
    assert_eq!(side_pieces.occupancy, A2_MASK);
    assert_eq!(side_pieces.pieces.piece(Piece::Pawn), A2_MASK);
}

#[test]
fn test_fen_parsing() {
    let empty = Board::from_fen("8/8/8/8/8/8/8/8 w - - 0 1").unwrap();
    assert_eq!(empty, Board::empty());

    let a8: Square = "a8".parse().unwrap();
    let board_a8_rook = Board::from_fen("r7/8/8/8/8/8/8/8 w - - 0 1").unwrap();
    assert_eq!(board_a8_rook.turn, Side::White);
    assert_eq!(board_a8_rook.black_side.occupancy, BoardMask::from(a8));

    let b_only_w_pawns = Board::from_fen("8/8/8/8/8/8/PPPPPPPP/8 b - - 0 1").unwrap();
    assert_eq!(b_only_w_pawns.turn, Side::Black);
    assert_eq!(b_only_w_pawns.black_side, Board::empty().black_side);
    assert_eq!(b_only_w_pawns.white_side.occupancy, BoardMask::from(0xFF00));

    let b_wb_mix_pawns = Board::from_fen("8/8/8/8/8/8/PpPPPppP/8 w - - 0 1").unwrap();
    assert_eq!(b_wb_mix_pawns.white_side.occupancy, BoardMask::from(0x9D00));
    assert_eq!(b_wb_mix_pawns.black_side.occupancy, BoardMask::from(0x6200));

    let missing_board = Board::from_fen("w - - 0 1");
    assert!(missing_board.is_err());

    let misaligned_board = Board::from_fen("8/8/8/8/8/8/7/8 w - - 0 1");
    assert_eq!(
        misaligned_board.unwrap_err(),
        FENParsingError::InvalidAlignment
    );

    let board_fen_with_extra_spaces =
        Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1 ");
    assert!(board_fen_with_extra_spaces.is_ok());
}

#[test]
fn test_fen_generation() {
    let mut board = Board::default();
    assert_eq!(board.into_fen(), Board::DEFAULT_FEN);

    board.feed("e2e4".parse().unwrap()).unwrap();
    let expected_fen = "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1";
    assert_eq!(board.into_fen(), expected_fen);

    board.feed("e7e5".parse().unwrap()).unwrap();
    board.feed("e1e2".parse().unwrap()).unwrap();

    let expected_fen = "rnbqkbnr/pppp1ppp/8/4p3/4P3/8/PPPPKPPP/RNBQ1BNR b kq - 0 1";
    assert_eq!(board.into_fen(), expected_fen);
}

#[test]
fn test_default_game() {
    let board = Board::default();
    assert_eq!(board.turn, Side::White);
    assert_eq!(board.white_side.occupancy, BoardMask::from(0xFFFF));
    assert_eq!(
        board.black_side.occupancy,
        BoardMask::from(0xFFFF << (8 * 6))
    );
}
