mod board_display;
mod perft_bisect;
use perft_bisect::perft_bisect;

use std::convert::AsRef;
use std::env;
use std::ops::ControlFlow::{self, Break, Continue};
use std::time::Instant;

use cheng::{Board, FromIntoFen, LegalMove, PseudoMove, Square};
use flimsybird::Evaluable;

use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;

use crate::board_display::BoardDisplay;

#[derive(Default)]
pub struct Context {
    board: Board,
}

fn main() -> Result<(), String> {
    env_logger::init();

    log::info!("initializing cheng...");
    cheng::init();

    let argv: Vec<_> = env::args().collect();
    let argv: Vec<&str> = argv.iter().map(AsRef::as_ref).collect();

    if argv.len() > 1 {
        interpret(&mut Context::default(), &argv[1..])
    } else {
        repl().map_err(|err| err.to_string())
    }
}

fn repl() -> rustyline::Result<()> {
    let mut rl = DefaultEditor::new()?;
    let mut context = Context::default();
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                let parts: Vec<&str> = line.split(' ').collect();
                let err = interpret(&mut context, &parts);
                if let Err(msg) = err {
                    eprintln!("error: {msg}");
                }
            }
            Err(ReadlineError::Interrupted | ReadlineError::Eof) => {
                break;
            }
            Err(err) => {
                println!("Error: {err:?}");
                break;
            }
        }
    }
    Ok(())
}

fn interpret(context: &mut Context, parts: &[&str]) -> Result<(), String> {
    match parts[0] {
        "perft" => perft(context, parts).map_err(String::from),
        "perft-bisect" => perft_bisect(context, parts).map_err(String::from),
        "fen" => fen(context, parts),
        "feed" => feed(context, parts),
        "ev" => evaluate(context, parts),
        "d" => display_board(context, parts),
        "dump-tables" => dump_tables(),
        "version" => {
            version();
            Ok(())
        }
        other => Err(format!("command not found: {other}")),
    }
}

fn version() {
    use cheng::movegen::{Bishop, Rook};
    use std::mem::size_of_val;
    use std::ptr::addr_of;

    const GIT_HASH: &'static str = env!("GIT_HASH");
    const GIT_DIRTY: &'static str = env!("GIT_DIRTY");

    let version = format!("{GIT_HASH}-{GIT_DIRTY}");

    // SAFETY: This is safe because we don't actually care about its value.
    // This was done using addr_of!() because rust emitted the warning tracked in
    // https://github.com/rust-lang/rust/issues/114447
    let rook_hash_size = size_of_val(unsafe { &*addr_of!(cheng::movegen::ROOK_MOVES) });
    let bishop_hash_size = size_of_val(unsafe { &*addr_of!(cheng::movegen::BISHOP_MOVES) });

    println!("cheng-cmd - {version}");
    println!("Rook hash size: {rook_hash_size} (nbits={})", Rook::nbits());
    println!(
        "Bishop hash size: {bishop_hash_size} (nbits={})",
        Bishop::nbits()
    );
}

fn display_board(context: &mut Context, _parts: &[&str]) -> Result<(), String> {
    println!("{}", BoardDisplay(&context.board.inner()));

    Ok(())
}

fn fen(context: &mut Context, parts: &[&str]) -> Result<(), String> {
    let fen = parts.get(1..).ok_or("Expected fen argument")?.join(" ");
    context.board = Board::from_fen(&fen).map_err(|err| format!("{err:?}"))?;
    Ok(())
}

#[must_use]
pub fn continue_<E>(_movement: &LegalMove, _nodes: usize) -> ControlFlow<E, ()> {
    Continue(())
}

fn incremental_perft<E, F>(board: &Board, depth: usize, mut callback: F) -> Result<usize, E>
where
    F: FnMut(&LegalMove, usize) -> ControlFlow<E, ()>,
{
    if depth == 0 {
        return Ok(1);
    }

    let moves = board.moves();
    let mut nodes = 0;
    for movement in moves {
        let mut clone = board.clone();
        clone.feed(movement.clone());

        let move_nodes = incremental_perft(&clone, depth - 1, continue_)?;
        nodes += move_nodes;

        let control = callback(&movement, move_nodes);
        match control {
            Continue(_) => continue,
            Break(e) => return Err(e),
        }
    }

    Ok(nodes)
}

fn perft(context: &mut Context, parts: &[&str]) -> Result<(), &'static str> {
    let perft_start = Instant::now();

    let depth: usize = parts
        .get(1)
        .ok_or("missing depth")?
        .parse()
        .map_err(|_| "invalid depth")?;

    let total_nodes = incremental_perft(&context.board, depth, |movement, nodes| {
        println!("{movement}: {nodes}");
        Continue::<()>(())
    })
    .unwrap();

    let perft_end = Instant::now();
    let perft_duration = perft_end - perft_start;

    println!("total nodes: {total_nodes}");
    println!(
        "total time: {perft_duration:?} ({} n/s)",
        total_nodes as f32 / perft_duration.as_secs_f32()
    );

    Ok(())
}

fn feed(context: &mut Context, parts: &[&str]) -> Result<(), String> {
    let pseudomove: PseudoMove = parts
        .get(1)
        .ok_or("missing move")?
        .parse()
        .map_err(|_| "invalid move")?;

    context
        .board
        .try_feed(pseudomove)
        .map_err(|err| format!("Invalid move: {err:?}"))
}

fn evaluate(context: &mut Context, _parts: &[&str]) -> Result<(), String> {
    let (best_move, evaluation) = context.board.evaluate();

    if let Some(best_move) = best_move {
        println!("best move found to be {best_move:?}");
    }

    println!("evaluation: {evaluation}");

    Ok(())
}

fn dump_tables() -> Result<(), String> {
    for sq in Square::iter_all() {
        println!("{sq:?}");
        let table_of_moves = unsafe { cheng::internal::ROOK_MOVES[sq.to_index()] };
        let mut amount_zeros = 0;
        for moves in table_of_moves.iter() {
            let value = u64::from(*moves);
            if value == 0 {
                amount_zeros += 1;
            } else {
                if amount_zeros > 0 {
                    println!("\t{amount_zeros} zeros");
                    amount_zeros = 0;
                }

                println!("\t{value:016X}");
            }
        }

        println!();
    }

    Ok(())
}
