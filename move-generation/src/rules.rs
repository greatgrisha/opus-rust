//! Chess rules and validation

use crate::types::{Board, Color, Move, Piece};

/// Check if a move is legal
pub fn is_legal_move(_board: &Board, _mv: &Move) -> bool {
    // Ensure the move adheres to the rules of the piece
    if !is_valid_piece_move(_board, _mv) {
        return false;
    }

    // Ensure the move does not leave the king in check
    if leaves_king_in_check(_board, _mv) {
        return false;
    }

    true
}

/// Validate the board state
pub fn validate_board(_board: &Board) -> bool {
    // Ensure there is exactly one king of each color
    let white_king_count = _board.squares.iter().filter(|&sq| {
        matches!(sq, Some((Piece::King, Color::White)))
    }).count();

    let black_king_count = _board.squares.iter().filter(|&sq| {
        matches!(sq, Some((Piece::King, Color::Black)))
    }).count();

    white_king_count == 1 && black_king_count == 1
}

/// Check if a move adheres to the rules of the piece
fn is_valid_piece_move(_board: &Board, _mv: &Move) -> bool {
    match _board.squares[_mv.from as usize] {
        Some((piece, color)) => match piece {
            Piece::Pawn => is_valid_pawn_move(_board, _mv, color),
            Piece::Knight => is_valid_knight_move(_mv),
            Piece::Bishop => is_valid_bishop_move(_board, _mv, color),
            Piece::Rook => is_valid_rook_move(_board, _mv, color),
            Piece::Queen => is_valid_queen_move(_board, _mv, color),
            Piece::King => is_valid_king_move(_board, _mv, color),
        },
        None => false,
    }
}

/// Check if a move leaves the king in check
fn leaves_king_in_check(_board: &Board, _mv: &Move) -> bool {
    // TODO: Implement king safety check
    false
}

/// Validate pawn moves
fn is_valid_pawn_move(_board: &Board, _mv: &Move, _color: Color) -> bool {
    // TODO: Implement pawn movement rules (including en passant and promotion)
    true
}

/// Validate knight moves
fn is_valid_knight_move(_mv: &Move) -> bool {
    // TODO: Implement knight movement rules
    true
}

/// Validate bishop moves
fn is_valid_bishop_move(_board: &Board, _mv: &Move, _color: Color) -> bool {
    // TODO: Implement bishop movement rules
    true
}

/// Validate rook moves
fn is_valid_rook_move(_board: &Board, _mv: &Move, _color: Color) -> bool {
    // TODO: Implement rook movement rules
    true
}

/// Validate queen moves
fn is_valid_queen_move(_board: &Board, _mv: &Move, _color: Color) -> bool {
    // TODO: Implement queen movement rules
    true
}

/// Validate king moves
fn is_valid_king_move(_board: &Board, _mv: &Move, _color: Color) -> bool {
    // TODO: Implement king movement rules (including castling)
    true
}