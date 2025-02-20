use std::{
    collections::HashMap,
    ops::ControlFlow::{Break, Continue},
};

use cheng::{Board, FromIntoFen};
use uci::Engine;

use crate::args::Args;
use crate::{incremental_perft, Context};

#[derive(Debug)]
pub enum PerftBisectErr {
    UnexpectedMove {
        movement: String,
    },
    MissingMove {
        movement: String,
    },
    WrongNodeCount {
        movement: String,
        expected: usize,
        got: usize,
    },
}

#[allow(clippy::needless_pass_by_value)]
pub fn perft_bisect(context: &mut Context, args: Args) -> Result<(), String> {
    let depth: usize = args.parse("depth", 1)?;

    let stockfish = Engine::new("stockfish").map_err(|e| format!("{e}"))?;
    let mut depth_remaining = depth;
    let mut board = context.board.clone();
    stockfish.set_position(&board.as_fen()).unwrap();

    while let Err(e) = perft_bisect_iteration(&stockfish, &board, depth_remaining) {
        match e {
            PerftBisectErr::UnexpectedMove { movement } => {
                println!("Unexpected move: {movement}");
                break;
            }
            PerftBisectErr::MissingMove { movement } => {
                println!("Missing move: {movement}");
                break;
            }
            PerftBisectErr::WrongNodeCount {
                movement,
                expected,
                got,
            } => {
                println!("Wrong node count in {movement} (got: {got}, expected: {expected})...");
                board.try_feed(movement.as_str()).unwrap();
                stockfish.set_position(&board.as_fen()).unwrap();
                depth_remaining -= 1;
            }
        }
    }

    Ok(())
}

fn perft_bisect_iteration(
    stockfish: &Engine,
    board: &Board,
    depth: usize,
) -> Result<(), PerftBisectErr> {
    let mut move_perft_table = perft_stockfish(stockfish, depth).unwrap();
    let bisect_result = incremental_perft(board, depth, |movement, nodes| {
        let movement_str = format!("{movement}");
        let expected_nodes = move_perft_table.remove(&movement_str);
        if let Some(expected) = expected_nodes {
            if expected != nodes {
                return Break(PerftBisectErr::WrongNodeCount {
                    movement: movement.to_string(),
                    expected,
                    got: nodes,
                });
            }
            Continue(())
        } else {
            println!("{move_perft_table:?}");
            Break(PerftBisectErr::UnexpectedMove {
                movement: movement.to_string(),
            })
        }
    });

    match bisect_result {
        Ok(nodes) => {
            if move_perft_table.is_empty() {
                println!("total nodes: {nodes}");
                Ok(())
            } else {
                let (first_missing_movement, _) = move_perft_table.drain().next().unwrap();
                Err(PerftBisectErr::MissingMove {
                    movement: first_missing_movement,
                })
            }
        }
        Err(err) => Err(err),
    }
}

fn perft_stockfish(stockfish: &Engine, depth: usize) -> Result<HashMap<String, usize>, String> {
    let first_part = stockfish
        .command(&format!("go perft {depth}"))
        .map_err(|e| format!("{e}"))?;

    // We need to do this due to a limitation in the `uci` library.
    // https://docs.rs/uci/0.1.3/src/uci/lib.rs.html#154
    let mut result = first_part;
    loop {
        let leftovers = stockfish.command("").map_err(|e| format!("{e}"))?;
        if leftovers.is_empty() {
            break;
        }
        result.push('\n');
        result.push_str(&leftovers);
        result.push('\n');
    }

    let mut map = HashMap::new();
    for line in result.lines() {
        if line.is_empty() || line.starts_with("Nodes searched: ") {
            continue;
        }

        let parts: Vec<&str> = line.split(": ").collect();
        let movement = parts[0].to_string();
        let count: usize = parts[1].parse().unwrap();

        map.insert(movement, count);
    }

    Ok(map)
}
