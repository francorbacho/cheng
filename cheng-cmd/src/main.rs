mod board_display;
mod perft_bisect;
use perft_bisect::perft_bisect;

use std::convert::AsRef;
use std::env;
use std::ops::ControlFlow::{self, Break, Continue};
use std::time::Instant;

use cheng::{Board, PseudoMove, Square};
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;

use crate::board_display::BoardDisplay;

#[derive(Default)]
pub struct Context {
    board: Board,
}

fn main() -> Result<(), String> {
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
        "d" => display_board(context, parts),
        "dump-tables" => dump_tables(),
        other => Err(format!("command not found: {other}")),
    }
}

fn display_board(context: &mut Context, _parts: &[&str]) -> Result<(), String> {
    println!("{}", BoardDisplay(&context.board));

    Ok(())
}

fn fen(context: &mut Context, parts: &[&str]) -> Result<(), String> {
    let fen = parts.get(1..).ok_or("Expected fen argument")?.join(" ");
    context.board = Board::from_fen(&fen).map_err(|err| format!("{err:?}"))?;
    Ok(())
}

#[must_use]
pub fn continue_<E>(_movement: &PseudoMove, _nodes: usize) -> ControlFlow<E, ()> {
    Continue(())
}

fn incremental_perft<E, F>(board: &Board, depth: usize, mut callback: F) -> Result<usize, E>
where
    F: FnMut(&PseudoMove, usize) -> ControlFlow<E, ()>,
{
    if depth == 0 {
        return Ok(1);
    }

    let moves = board.moves();
    let mut nodes = 0;
    for movement in moves {
        let mut clone = board.clone();

        clone.feed(movement.clone()).unwrap();
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
        .feed(pseudomove)
        .map_err(|err| format!("Invalid move: {err:?}"))
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
