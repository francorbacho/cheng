use cheng::{Board, Piece};

static mut BOARD: Option<Board> = None;

#[no_mangle]
fn initialize() {
    unsafe {
        BOARD = Some(Board::default());
    }
}

#[no_mangle]
fn get_pawn_count() -> u32 {
    let board = unsafe { BOARD.as_ref().expect("BOARD was not initialized!") };
    let pawns = board.white_side.pieces.piece(Piece::Pawn);

    pawns.count() as u32
}
