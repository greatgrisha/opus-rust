use move_generation::types::{Board, Color, Piece};
use move_generation::move_gen::generate_moves;
use std::process::{Command, Stdio};
use std::io::Write;
use serde_json::json;

fn square_idx_to_uci(idx: u8) -> String {
    let file = (b'a' + (idx % 8) as u8) as char;
    let rank = (b'1' + (idx / 8) as u8) as char;
    format!("{}{}", file, rank)
}

#[test]
fn test_random_board_position() {
    let board = Board {
        squares: [
            Some((Piece::Rook, Color::White)), None, None, None, Some((Piece::King, Color::White)), None, None, Some((Piece::Rook, Color::White)),
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            Some((Piece::Rook, Color::Black)), None, None, None, Some((Piece::King, Color::Black)), None, None, Some((Piece::Rook, Color::Black)),
        ],
        side_to_move: Color::White,
    };

    let moves = generate_moves(&board, Color::White);

    // Debugging output
    println!("Generated moves: {:?}", moves);

    // TODO: Validate moves against Stockfish or PyChess
    assert!(!moves.is_empty(), "No moves generated for the given board position.");
}

#[test]
fn test_validate_with_pychess() {
    let board = Board {
        squares: [
            Some((Piece::Rook, Color::White)), None, None, None, Some((Piece::King, Color::White)), None, None, Some((Piece::Rook, Color::White)),
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            Some((Piece::Rook, Color::Black)), None, None, None, Some((Piece::King, Color::Black)), None, None, Some((Piece::Rook, Color::Black)),
        ],
        side_to_move: Color::White,
    };

    let rust_moves = generate_moves(&board, Color::White);
    let rust_moves_uci: Vec<String> = rust_moves.iter()
        .map(|m| format!("{}{}", square_idx_to_uci(m.from), square_idx_to_uci(m.to)))
        .collect();

    let board_fen = "r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1";

    let input = serde_json::json!({
        "board_fen": board_fen,
        "rust_moves": rust_moves_uci,
    });

    let mut child = std::process::Command::new("python3")
        .arg("validate_with_pychess.py")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .expect("Failed to start Python script");

    {
        let stdin = child.stdin.as_mut().expect("Failed to open stdin");
        stdin.write_all(input.to_string().as_bytes()).expect("Failed to write to stdin");
    }

    let output = child.wait_with_output().expect("Failed to read stdout");
    let result: serde_json::Value = serde_json::from_slice(&output.stdout).expect("Failed to parse JSON output");

    assert!(result["valid"].as_bool().unwrap(), "Moves validation failed: {:?}", result);
}
