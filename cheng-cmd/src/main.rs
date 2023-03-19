mod board_display;
mod perft_bisect;
use perft_bisect::perft_bisect;

use std::ops::ControlFlow::{self, Break, Continue};

use cheng::{Board, PseudoMove};
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;

use crate::board_display::BoardDisplay;

#[derive(Default)]
pub struct Context {
    board: Board,
}

fn main() -> rustyline::Result<()> {
    cheng::init();

    let mut rl = DefaultEditor::new()?;
    let mut context = Context::default();
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                let parts: Vec<&str> = line.split(' ').collect();
                let err = match parts[0] {
                    "perft" => perft(&mut context, &parts).map_err(String::from),
                    "perft-bisect" => perft_bisect(&mut context, &parts).map_err(String::from),
                    "fen" => fen(&mut context, &parts),
                    "feed" => feed(&mut context, &parts),
                    "d" => display_board(&mut context, &parts),
                    other => Err(format!("command not found: {other}")),
                };

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

    println!("total nodes: {total_nodes}");

    Ok(())
}

fn feed(context: &mut Context, parts: &[&str]) -> Result<(), String> {
    let pseudomove: PseudoMove = parts
        .get(1)
        .ok_or("missing move")?
        .parse()
        .map_err(|_| "invalid move")?;

    context.board.feed(pseudomove);

    Ok(())
}
