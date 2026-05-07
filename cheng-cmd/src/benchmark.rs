use cheng::{Board, FromIntoFen, GameResult, LegalMove, Piece, SidedPiece};
use franfish::go_with_timeout;
use rand::seq::SliceRandom;
use rand::Rng;
use std::time::Duration;

const DEFAULT_POSITIONS: usize = 100;
const DEFAULT_MOVES: usize = 20;
const DEFAULT_TIMEOUT_MS: u64 = 1000;

const PIECE_VALUES: [i16; 6] = [100, 300, 300, 500, 900, 10000];

pub fn run_benchmark(args: &[&str]) -> Result<(), String> {
    let positions: usize = args
        .first()
        .and_then(|s| s.parse().ok())
        .unwrap_or(DEFAULT_POSITIONS);
    let moves: usize = args
        .get(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(DEFAULT_MOVES);
    let timeout_ms: u64 = args
        .get(2)
        .and_then(|s| s.parse().ok())
        .unwrap_or(DEFAULT_TIMEOUT_MS);

    println!(
        "Running benchmark: {} positions, {} random moves each, {}ms timeout",
        positions, moves, timeout_ms
    );
    println!("Generating random positions...\n");

    let mut rng = rand::thread_rng();
    let mut wins = 0;
    let mut losses = 0;
    let mut draws = 0;

    for i in 0..positions {
        let board = generate_random_position(&mut rng, moves);

        if board.result() != GameResult::Undecided {
            continue;
        }

        let timeout = Duration::from_millis(timeout_ms);
        let franfish_result = std::panic::catch_unwind(|| {
            go_with_timeout(&board, Some(timeout))
        });

        let Ok(franfish_result) = franfish_result else {
            continue;
        };

        let franfish_move = franfish_result.movement;

        let moves_vec: Vec<LegalMove> = board.moves().collect();
        if moves_vec.is_empty() {
            continue;
        }

        let random_move = moves_vec.choose(&mut rng).cloned().unwrap();

        let franfish_eval = evaluate_board(&board, franfish_move);
        let random_eval = evaluate_board(&board, random_move);

        let outcome = if franfish_eval > random_eval {
            wins += 1;
            "WIN"
        } else if franfish_eval < random_eval {
            losses += 1;
            "LOSS"
        } else {
            draws += 1;
            "DRAW"
        };

        if i < 10 || i % 100 == 0 {
            println!(
                "[{:>4}] franfish: {} vs random: {} => {}",
                i,
                format_evaluation(franfish_eval),
                format_evaluation(random_eval),
                outcome
            );
        }
    }

    let total = wins + losses + draws;
    println!("\n--- Results ---");
    println!(
        "franfish wins:  {:>4} ({:.1}%)",
        wins,
        (wins as f64 / total as f64) * 100.0
    );
    println!(
        "franfish losses: {:>4} ({:.1}%)",
        losses,
        (losses as f64 / total as f64) * 100.0
    );
    println!(
        "draws:          {:>4} ({:.1}%)",
        draws,
        (draws as f64 / total as f64) * 100.0
    );
    println!("total positions: {}", total);

    Ok(())
}

fn evaluate_board(board: &Board, mv: LegalMove) -> i16 {
    let mut board_clone = board.clone();
    board_clone.feed(mv);
    static_evaluation(&board_clone)
}

fn static_evaluation(board: &Board) -> i16 {
    let mut eval: i16 = 0;
    for (SidedPiece(side, piece), _square) in board.inner() {
        let value = match piece {
            Piece::Pawn => PIECE_VALUES[0],
            Piece::Knight => PIECE_VALUES[1],
            Piece::Bishop => PIECE_VALUES[2],
            Piece::Rook => PIECE_VALUES[3],
            Piece::Queen => PIECE_VALUES[4],
            Piece::King => PIECE_VALUES[5],
        };
        let sign = if side == cheng::Side::White { 1 } else { -1 };
        eval += sign * value;
    }
    eval
}

fn generate_random_position<R: Rng>(rng: &mut R, num_moves: usize) -> Board {
    let mut board = Board::default();

    for _ in 0..num_moves {
        let moves: Vec<LegalMove> = {
            let b = &board;
            b.moves().collect()
        };
        if moves.is_empty() {
            break;
        }
        let mov = moves.choose(rng).unwrap().clone();
        let mut board_clone = board.clone();
        board_clone.feed(mov);
        if board_clone.result() != GameResult::Undecided {
            break;
        }
        board = board_clone;
    }

    board
}

fn format_evaluation(eval: i16) -> String {
    format!("{}", eval)
}

pub fn generate_positions(args: &[&str]) -> Result<(), String> {
    let count: usize = args
        .first()
        .and_then(|s| s.parse().ok())
        .unwrap_or(100);
    let moves: usize = args
        .get(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(20);

    let mut rng = rand::thread_rng();

    for _ in 0..count {
        let board = generate_random_position(&mut rng, moves);
        if board.result() == GameResult::Undecided {
            let fen = board.inner().as_fen();
            println!("{}", fen);
        }
    }

    Ok(())
}