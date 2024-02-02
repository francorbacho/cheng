use cheng::Board;
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
    if parts.get(1) != Some(&"startpos") {
        return Err(format!("bad word"));
    }

    context.board = Board::default();

    if let Some(&"moves") = parts.get(2) {
        for mv in &parts[3..] {
            context
                .board
                .try_feed(*mv)
                .map_err(|_| format!("received invalid move"))?;
        }
    }

    Ok(())
}

pub fn go(context: &mut Context, parts: &[&str]) -> Result<(), String> {
    let movetime = match parts[1..] {
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
