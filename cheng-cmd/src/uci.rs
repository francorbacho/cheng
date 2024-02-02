use cheng::{Board, FromIntoFen};
use flimsybird::Evaluable;

use crate::Context;

pub fn uci() {
    println!("uciok");
}

pub fn ucinewgame(_context: &mut Context) {}

pub fn isready() {
    println!("readyok");
}

pub fn position(context: &mut Context, parts: &[&str]) -> Result<(), String> {
    let mut parts = &parts[1..];
    if parts.get(0) == Some(&"startpos") {
        context.board = Board::default();
        parts = &parts[1..];
    } else if parts.get(0) == Some(&"fen") {
        let fen = match &parts.get(1..7) {
            Some(parts) => parts.join(" "),
            None => return Err(format!("invalid fen, missing parts")),
        };
        context.board = Board::from_fen(&fen).map_err(|e| format!("{e:?}"))?;
        parts = &parts[7..];
    } else {
        return Err(format!("bad word: '{}'", parts[1..].join(" ")));
    }

    match parts.get(0) {
        Some(&"moves") => {
            for mv in &parts[1..] {
                context
                    .board
                    .try_feed(*mv)
                    .map_err(|_| format!("received invalid move"))?;
            }

            Ok(())
        }
        Some(word) => Err(format!("invalid word: {word}")),
        None => Ok(()),
    }
}

pub fn go(context: &mut Context, parts: &[&str]) -> Result<(), String> {
    let movetime = match parts[1..] {
        // FIXME: Workaround to get `go` working.
        [] => "0",
        ["movetime", movetime] => movetime,
        ["wtime", wtime, "btime", _btime] => wtime,
        ["wtime", wtime, "btime", _btime, "winc", _winc, "binc", _binc] => wtime,
        _ => return Err("invalid format".to_string()),
    };

    let _movetime: usize = movetime.parse().map_err(|_| format!("invalid wtime"))?;

    let (best_move, _) = context.board.evaluate();

    if let Some(best_move) = best_move {
        println!("bestmove {best_move}");
    }

    Ok(())
}
