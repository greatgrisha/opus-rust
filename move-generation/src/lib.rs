use pyo3::prelude::*;
use pyo3::types::PyList;
use crate::types::{Board, Piece, Color};
use crate::move_gen::generate_moves;

/// PyO3 Python API
#[pyclass]
pub struct PyBoard {
    pub board: Board,
}

#[pymethods]
impl PyBoard {
    #[new]
    pub fn new() -> Self {
        // TODO: Accept FEN as argument if needed
        Self {
            board: Board {
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
            },
        }
    }

    pub fn generate_moves(&self, py: Python<'_>) -> PyObject {
        let moves = generate_moves(&self.board, self.board.side_to_move);
        let moves_uci: Vec<String> = moves.iter()
            .map(|m| {
                let file = |idx| (b'a' + (idx % 8) as u8) as char;
                let rank = |idx| (b'1' + (idx / 8) as u8) as char;
                format!("{}{}{}{}", file(m.from), rank(m.from), file(m.to), rank(m.to))
            })
            .collect();
        PyList::new(py, moves_uci).into()
    }
}

#[pymodule]
fn move_generation(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyBoard>()?;
    Ok(())
}

/// Rust-native API

pub mod move_gen;
pub mod rules;
pub mod types;

use crate::types::Move;
use crate::move_gen::generate_piece_moves;
use crate::rules::{is_legal_move, validate_board};

pub fn legal_moves(board: &Board, color: Color) -> Vec<Move> {
    generate_moves(board, color)
}

pub fn piece_moves(board: &Board, piece: Piece, sq: u8) -> Vec<Move> {
    generate_piece_moves(board, piece, sq)
}

pub fn is_move_legal(board: &Board, mv: &Move) -> bool {
    is_legal_move(board, mv)
}

pub fn is_board_valid(board: &Board) -> bool {
    validate_board(board)
}