//! Chess rules and validation

use crate::{types::{Board, Move, Piece, Color}, move_gen::generate_piece_moves};

/// Check if a move is legal
pub fn is_legal_move(board: &Board, mv: &Move) -> bool {
    // Ensure the move adheres to the rules of the piece
    if !is_valid_piece_move(board, mv) {
        return false;
    }

    // Ensure the move does not leave the king in check
    if leaves_king_in_check(board, mv) {
        return false;
    }

    true
}

/// Validate the board state
pub fn validate_board(board: &Board) -> bool {
    // Ensure there is exactly one king of each color
    let white_king_count = board.squares.iter().filter(|&sq| {
        matches!(sq, Some((Piece::King, Color::White)))
    }).count();

    let black_king_count = board.squares.iter().filter(|&sq| {
        matches!(sq, Some((Piece::King, Color::Black)))
    }).count();

    white_king_count == 1 && black_king_count == 1
}

/// Check if a move leaves the king in check
fn leaves_king_in_check(board: &Board, mv: &Move) -> bool {
    let mut new_board = board.clone();

    // Make the move on a cloned board
    new_board.squares[mv.to as usize] = new_board.squares[mv.from as usize];
    new_board.squares[mv.from as usize] = None;

    // Find the king's position
    let king_pos = new_board.squares.iter().position(|&sq| {
        matches!(sq, Some((Piece::King, color)) if color == new_board.side_to_move)
    });

    if let Some(king_sq) = king_pos {
        // Check if the king is attacked
        is_square_attacked(&new_board, king_sq as u8, new_board.side_to_move)
    } else {
        false
    }
}

/// Check if a square is attacked by the opponent
fn is_square_attacked(board: &Board, sq: u8, color: Color) -> bool {
    for (i, piece) in board.squares.iter().enumerate() {
        if let Some((p, c)) = piece {
            if *c != color {
                let moves = generate_piece_moves(board, *p, i as u8);
                if moves.iter().any(|m| m.to == sq) {
                    return true;
                }
            }
        }
    }
    false
}

/// Check if a move adheres to the rules of the piece
fn is_valid_piece_move(board: &Board, mv: &Move) -> bool {
    match board.squares[mv.from as usize] {
        Some((piece, color)) => match piece {
            Piece::Pawn => is_valid_pawn_move(board, mv, color),
            Piece::Knight => is_valid_knight_move(mv),
            Piece::Bishop => is_valid_bishop_move(board, mv, color),
            Piece::Rook => is_valid_rook_move(board, mv, color),
            Piece::Queen => is_valid_queen_move(board, mv, color),
            Piece::King => is_valid_king_move(board, mv, color),
        },
        None => false,
    }
}

/// Validate pawn moves
fn is_valid_pawn_move(board: &Board, mv: &Move, color: Color) -> bool {
    // TODO: Implement pawn movement rules (including en passant and promotion)
    true
}

/// Validate knight moves
fn is_valid_knight_move(mv: &Move) -> bool {
    // TODO: Implement knight movement rules
    true
}

/// Validate bishop moves
fn is_valid_bishop_move(board: &Board, mv: &Move, color: Color) -> bool {
    // TODO: Implement bishop movement rules
    true
}

/// Validate rook moves
fn is_valid_rook_move(board: &Board, mv: &Move, color: Color) -> bool {
    // TODO: Implement rook movement rules
    true
}

/// Validate queen moves
fn is_valid_queen_move(board: &Board, mv: &Move, color: Color) -> bool {
    // TODO: Implement queen movement rules
    true
}

/// Validate king moves
fn is_valid_king_move(board: &Board, mv: &Move, color: Color) -> bool {
    // TODO: Implement king movement rules (including castling)
    true
}

/// Validate castling rules
fn is_valid_castling(board: &Board, mv: &Move) -> bool {
    // TODO: Implement castling rules
    true
}