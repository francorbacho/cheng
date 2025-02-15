use std::time::Duration;

use cheng::{Board, FromIntoFen};

use franfish::GoResult;

use crate::args::Args;
use crate::Context;

pub fn uci() {
    println!("uciok");
}

pub fn ucinewgame(_context: &mut Context) {}

pub fn isready() {
    println!("readyok");
}

#[allow(clippy::needless_pass_by_value)]
pub fn position(context: &mut Context, args: Args) -> Result<(), String> {
    let input = args.parts();
    let mut iter = input.into_iter().peekable();

    match iter.next() {
        Some("position") => {}
        value => return Err(format!("Expected 'position' instead of {value:?}")),
    }

    context.board = match iter.next() {
        Some("startpos") => Board::default(),
        Some("fen") => {
            let fen_parts: Vec<&str> = iter
                .by_ref()
                .take_while(|&token| token != "moves")
                .collect();
            if fen_parts.is_empty() {
                return Err("Expected FEN string after 'fen'".to_string());
            }
            Board::from_fen(&fen_parts.join(" ")).map_err(|e| format!("{e:?}"))?
        }
        _ => return Err("Expected 'startpos' or 'fen'".to_string()),
    };

    let moves = if iter.peek() == Some(&"moves") {
        iter.next();
        iter.map(String::from).collect()
    } else {
        Vec::new()
    };

    for mv in moves {
        context
            .board
            .try_feed(mv.as_str())
            .map_err(|_| "received invalid move".to_string())?;
    }

    Ok(())
}

#[allow(clippy::needless_pass_by_value)]
pub fn go(context: &mut Context, args: Args) -> Result<(), String> {
    let movetime = match args.parts()[1..] {
        // FIXME: Workaround to get `go` working.
        [] => "0",
        ["movetime", movetime] => movetime,
        ["wtime", _wtime, "btime", _btime] => todo!(),
        ["wtime", _wtime, "btime", _btime, "winc", _winc, "binc", _binc] => todo!(),
        _ => return Err("invalid format".to_string()),
    };

    let _movetime: usize = movetime.parse().map_err(|_| "invalid wtime".to_string())?;

    let GoResult { movement, .. } = context.go_franfish();

    if let Some(best_move) = movement {
        println!("bestmove {best_move}");
    } else {
        println!("bestmove (none)");
    }

    Ok(())
}

pub fn eval(_context: &mut Context) {
    todo!()
}

#[allow(clippy::needless_pass_by_value)]
pub fn setoption(context: &mut Context, args: Args) -> Result<(), String> {
    match args.as_str("option", 1) {
        Ok("timeout") => {
            context.timeout = Some(Duration::from_millis(args.parse("timeout", 2).unwrap()));
        }
        Ok(option) => return Err(format!("no such option {option}")),
        Err(e) => return Err(e),
    }

    Ok(())
}
