mod args;

mod uci;

mod board_display;
mod perft_bisect;
use perft_bisect::perft_bisect;

use args::Args;

use std::io::{self, Write};
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

    let args = Args::from_argv();

    if args.len() > 1 {
        interpret(&mut Context::default(), args)
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
                let args = args::Args::from_line(&line);

                if args.is_quit() {
                    break;
                } else if let Err(msg) = interpret(&mut context, args) {
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

fn interpret(context: &mut Context, args: Args) -> Result<(), String> {
    let ok = match args.cmd() {
        // UCI
        "uci" => Ok(uci::uci()),
        "ucinewgame" => Ok(uci::ucinewgame(context)),
        "isready" => Ok(uci::isready()),
        "position" => uci::position(context, args),
        "go" => uci::go(context, args),
        "eval" => Ok(uci::eval(context)),

        "ff" => ff::go(context, args),

        // our protocol
        "goinfo" => goinfo(context).map_err(String::from),
        "perft" => perft(context, args).map_err(String::from),
        "perft-bisect" => perft_bisect(context, args).map_err(String::from),
        "fen" => fen(context, args),
        "feed" => feed(context, args),
        "ev" => Ok(evaluate(context)),
        "d" => Ok(display_board(context, args)),
        "dump-tables" => Ok(dump_tables()),
        "bench" => bench(args),
        "version" => Ok(version()),
        other => Err(format!("command not found: {other}")),
    };

    io::stdout().flush().unwrap();

    ok
}

fn version() {
    use cheng::movegen::{Bishop, Rook};
    use std::mem::size_of_val;
    use std::ptr::addr_of;

    const GIT_HASH: &str = env!("GIT_HASH");
    const GIT_DIRTY: &str = env!("GIT_DIRTY");
    const DATE: &str = env!("DATE");

    let version = format!("{GIT_HASH}-{GIT_DIRTY}");

    // SAFETY: This is safe because we don't actually care about its value.
    // This was done using addr_of!() because rust emitted the warning tracked in
    // https://github.com/rust-lang/rust/issues/114447
    let rook_hash_size = size_of_val(unsafe { &*addr_of!(cheng::movegen::ROOK_MOVES) });
    let bishop_hash_size = size_of_val(unsafe { &*addr_of!(cheng::movegen::BISHOP_MOVES) });

    println!("cheng-cmd - {version}");
    println!("Built: {DATE}");
    println!("Rook hash size: {rook_hash_size} (nbits={})", Rook::nbits());
    println!(
        "Bishop hash size: {bishop_hash_size} (nbits={})",
        Bishop::nbits()
    );
}

fn display_board(context: &mut Context, _args: Args) {
    println!("{}", BoardDisplay(context.board.inner()));
    println!("fen: {}", context.board.as_fen());
    println!("result: {:?}", context.board.result());
}

fn fen(context: &mut Context, args: Args) -> Result<(), String> {
    let fen = args.join_from("fen", 1)?;
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

fn goinfo(context: &mut Context) -> Result<(), &'static str> {
    let mut board_clone = context.board.clone();
    let (mv, _) = board_clone.evaluate();
    let mv = mv.unwrap();
    println!("info pv {mv}");
    Ok(())
}

fn perft(context: &mut Context, args: Args) -> Result<(), String> {
    let perft_start = Instant::now();

    let depth: usize = args.parse("depth", 1)?;

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

fn feed(context: &mut Context, args: Args) -> Result<(), String> {
    let pseudomove: PseudoMove = args.parse::<PseudoMove>("move", 1)?;

    context
        .board
        .try_feed(pseudomove)
        .map_err(|err| format!("Invalid move: {err:?}"))
}

fn evaluate(context: &mut Context) {
    let mut binding = context.board.clone();
    let (best_move, evaluation) = binding.evaluate();

    if let Some(best_move) = best_move {
        println!("{}", cheng::SAN(&best_move, &context.board));
    }

    println!("evaluation: {evaluation}");
}

fn dump_tables() {
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
}

fn bench(args: Args) -> Result<(), String> {
    match args.as_str("what to bench", 1)? {
        "magics" => Ok(bench_magics()),
        "fen" => Ok(bench_fen()),
        word => Err(format!("bad word: {word}")),
    }
}

fn bench_magics() {
    use cheng::movegen::PieceExt;
    use cheng::movegen::{Bishop, Rook};
    use cheng::BoardMask;

    let before = Instant::now();

    for i in 0..1_000_000_000 {
        let square = i % 64;

        std::hint::black_box(Rook::moves(
            Square::from_index(square),
            BoardMask::default(),
            BoardMask::default(),
        ));
        std::hint::black_box(Bishop::moves(
            Square::from_index(square),
            BoardMask::default(),
            BoardMask::default(),
        ));
    }

    let after = Instant::now();
    let took = after - before;

    println!("1 billion interations took :: {took:?}");
}

fn bench_fen() {
    let before = Instant::now();
    let fen = "8/k7/1NpP1K2/6B1/Pp2P1pp/1P4rr/1PpbNP2/5R2 w - - 0 1";
    let board = Board::from_fen(fen).unwrap();
    evaluate(&mut Context { board });
    let after = Instant::now();
    let took = after - before;
    println!("evaluation took :: {took:?}");
}

mod ff {
    use crate::Context;
    use crate::args::Args;
    use std::time::Instant;

    pub fn go(context: &mut Context, args: Args) -> Result<(), String> {
        let go_start = Instant::now();

        let depth: usize = args.parse::<usize>("depth", 1)?;

        let movement = franfish::go_debug(&context.board, depth);

        let go_end = Instant::now();
        let go_duration = go_end - go_start;

        println!("total time: {go_duration:?}\nmove: {movement}",);

        Ok(())
    }
}
