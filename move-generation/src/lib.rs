/// Fast chess move generation and board validation library

pub mod move_gen;
pub mod rules;
pub mod types;

pub use move_gen::{generate_moves, generate_piece_moves};
pub use rules::{is_legal_move, validate_board};
pub use types::{Board, Color, Piece, Move};

/// Generate all legal moves for a given board and color
pub fn legal_moves(board: &Board, color: Color) -> Vec<Move> {
    generate_moves(board, color)
}

/// Generate all valid moves for a specific piece on the board
pub fn piece_moves(board: &Board, piece: Piece, sq: u8) -> Vec<Move> {
    generate_piece_moves(board, piece, sq)
}

/// Validate if a move is legal
pub fn is_move_legal(board: &Board, mv: &Move) -> bool {
    is_legal_move(board, mv)
}

/// Validate the board state
pub fn is_board_valid(board: &Board) -> bool {
    validate_board(board)
}