mod board_display;

use cheng::Board;
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;

use crate::board_display::BoardDisplay;

#[derive(Default)]
struct Context {
    board: Board,
}

fn main() -> rustyline::Result<()> {
    let mut rl = DefaultEditor::new()?;
    let mut context = Context::default();
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                let parts: Vec<&str> = line.split(' ').collect();
                let err = match parts[0] {
                    "perft" => perft(&mut context, parts).map_err(String::from),
                    "fen" => fen(&mut context, parts),
                    "d" => display_board(&mut context, parts),
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
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    Ok(())
}

fn display_board(context: &mut Context, _parts: Vec<&str>) -> Result<(), String> {
    println!("{}", BoardDisplay(&context.board));

    Ok(())
}

fn fen(context: &mut Context, parts: Vec<&str>) -> Result<(), String> {
    let fen = parts.get(1..).ok_or("Expected fen argument")?.join(" ");
    context.board = Board::from_fen(&fen).map_err(|err| format!("{err:?}"))?;
    Ok(())
}

fn perft(context: &mut Context, parts: Vec<&str>) -> Result<(), &'static str> {
    fn inner_perft(board: &Board, depth: usize, report_nodes: bool) -> usize {
        if depth == 0 {
            return 1;
        }

        let moves = board.moves();
        let mut nodes = 0;
        for movement in moves {
            let mut clone = board.clone();
            clone.feed(movement.clone());

            let move_nodes = inner_perft(board, depth - 1, false);
            nodes += move_nodes;

            if report_nodes {
                println!("{movement}: {move_nodes} nodes");
            }
        }

        if report_nodes {
            println!("total nodes: {nodes}");
        }

        nodes
    }

    let depth: usize = parts
        .get(1)
        .ok_or("missing depth")?
        .parse()
        .map_err(|_| "")?;

    inner_perft(&context.board, depth, true);

    Ok(())
}
