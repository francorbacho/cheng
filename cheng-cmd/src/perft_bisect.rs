use std::{
    collections::HashMap,
    ops::ControlFlow::{Break, Continue},
};

use cheng::Board;
use uci::Engine;

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

pub fn perft_bisect(context: &mut Context, parts: Vec<&str>) -> Result<(), String> {
    let depth: usize = parts
        .get(1)
        .ok_or("missing depth")?
        .parse()
        .map_err(|_| "invalid depth")?;

    let stockfish = Engine::new("stockfish").map_err(|e| format!("{e}"))?;
    let mut depth_remaining = depth;
    let mut board = context.board.clone();
    stockfish.set_position(&board.into_fen()).unwrap();

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
                board.feed(movement.parse().unwrap());
                stockfish.set_position(&board.into_fen()).unwrap();
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
        let expected_nodes = move_perft_table.remove(movement);
        match expected_nodes {
            Some(expected) => {
                if expected != nodes {
                    return Break(PerftBisectErr::WrongNodeCount {
                        movement: movement.to_string(),
                        expected,
                        got: nodes,
                    });
                }
                Continue(())
            }
            None => {
                println!("{move_perft_table:?}");
                return Break(PerftBisectErr::UnexpectedMove {
                    movement: movement.to_string(),
                });
            }
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
    let result = stockfish
        .command(&format!("go perft {depth}"))
        .map_err(|e| format!("{e}"))?;

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
