use pyo3::prelude::*;
use pyo3::{exceptions, Bound};
use pyo3::types::{PyList, PyTuple, PyModule};
use crate::types::{Board, Piece, Color};
use crate::pgn::{Position, PgnReader, ChessError};
use crate::move_gen::{generate_moves, generate_piece_moves};
// ...existing code...
use rayon::prelude::*;

/// PyO3 Python API
#[pyclass]
pub struct PyBoard {
    pub board: Board,
}

#[pymethods]
impl PyBoard {
    #[new]
    pub fn new() -> Self {
        Self {
            board: Board {
                squares: [None; 64],  // Empty board
                side_to_move: Color::White,
                castling_rights: "KQkq".to_string(),
                en_passant: None,
                halfmove_clock: 0,
                fullmove_number: 1,
            },
        }
    }

    /// Set pieces on the board from a list of (piece_type, color, square) tuples
    pub fn set_pieces(&mut self, pieces: &Bound<'_, PyList>) -> PyResult<()> {
        // Clear the board first
        self.board.squares = [None; 64];
        
        for item in pieces.iter() {
            let tuple = item.downcast::<PyTuple>()?;
            let piece_str: String = tuple.get_item(0)?.extract()?;
            let color_str: String = tuple.get_item(1)?.extract()?;
            let square: u8 = tuple.get_item(2)?.extract()?;
            
            if square >= 64 {
                return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                    format!("Invalid square index: {}", square)
                ));
            }

            let piece = match piece_str.to_lowercase().as_str() {
                "pawn" => Piece::Pawn,
                "knight" => Piece::Knight,
                "bishop" => Piece::Bishop,
                "rook" => Piece::Rook,
                "queen" => Piece::Queen,
                "king" => Piece::King,
                _ => return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                    format!("Invalid piece type: {}", piece_str)
                )),
            };

            let color = match color_str.to_lowercase().as_str() {
                "white" => Color::White,
                "black" => Color::Black,
                _ => return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                    format!("Invalid color: {}", color_str)
                )),
            };

            self.board.squares[square as usize] = Some((piece, color));
        }
        Ok(())
    }

    /// Set the side to move
    pub fn set_side_to_move(&mut self, color_str: &str) -> PyResult<()> {
        self.board.side_to_move = match color_str.to_lowercase().as_str() {
            "white" => Color::White,
            "black" => Color::Black,
            _ => return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                format!("Invalid color: {}", color_str)
            )),
        };
        Ok(())
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
        PyList::new_bound(py, moves_uci).into()
    }

    pub fn generate_moves_for_pieces_parallel(&self, py: Python<'_>, piece_sq_list: &Bound<'_, PyList>) -> PyObject {
        // Convert Python list of (piece: str, square: int) to Rust Vec<(Piece, u8)>
        let mut native_vec = Vec::with_capacity(piece_sq_list.len());
        for item in piece_sq_list.iter() {
            let tuple = item.downcast::<PyTuple>().unwrap();
            let piece_str: String = tuple.get_item(0).unwrap().extract().unwrap();
            let sq: u8 = tuple.get_item(1).unwrap().extract().unwrap();
            let piece = match piece_str.to_lowercase().as_str() {
                "pawn" => Piece::Pawn,
                "knight" => Piece::Knight,
                "bishop" => Piece::Bishop,
                "rook" => Piece::Rook,
                "queen" => Piece::Queen,
                "king" => Piece::King,
                _ => Piece::Pawn, // fallback
            };
            native_vec.push((piece, sq));
        }
        // Now parallelize over the Rust Vec
        let results: Vec<Vec<String>> = native_vec
            .par_iter()
            .map(|(piece, sq)| {
                let moves = generate_piece_moves(&self.board, *piece, *sq);
                moves.iter().map(|m| {
                    let file = |idx| (b'a' + (idx % 8) as u8) as char;
                    let rank = |idx| (b'1' + (idx / 8) as u8) as char;
                    format!("{}{}{}{}", file(m.from), rank(m.from), file(m.to), rank(m.to))
                }).collect::<Vec<_>>()
            })
            .collect();
        PyList::new_bound(py, results).into()
    }

    /// Load a position from FEN string
    pub fn load_fen(&mut self, fen: &str) -> PyResult<()> {
        let position: Position = fen.parse()
            .map_err(|e: ChessError| PyErr::new::<exceptions::PyValueError, _>(e.to_string()))?;

        // Set all board fields from position
        self.board.squares = [None; 64];
        for (piece, color, square) in position.pieces {
            self.board.squares[square as usize] = Some((piece, color));
        }
        self.board.side_to_move = position.side_to_move;
        self.board.castling_rights = position.castling_rights;
        self.board.en_passant = position.en_passant;
        self.board.halfmove_clock = position.halfmove_clock;
        self.board.fullmove_number = position.fullmove_number;
        Ok(())
    }

    /// Load positions from a PGN file
    pub fn load_pgn(&mut self, path: &str) -> PyResult<Vec<(String, String)>> {
        let file = std::fs::File::open(path)
            .map_err(|e| PyErr::new::<exceptions::PyValueError, _>(e.to_string()))?;
        
        let mut reader = PgnReader::new(file);
        let mut positions = Vec::new();
        
        while let Some(position) = reader.next_position()
            .map_err(|e| PyErr::new::<exceptions::PyValueError, _>(e.to_string()))? {
            
            // Convert position to FEN for storage
            let fen = position.to_fen();
            
            // Load position and generate moves
            self.load_fen(&fen)?;
            let moves = Python::with_gil(|py| self.generate_moves(py));
            
            positions.push((fen, format!("{:?}", moves)));
        }
        
        Ok(positions)
    }
}

#[pymodule]
fn move_generation(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyBoard>()?;
    Ok(())
}

/// Rust-native API

pub mod move_gen;
pub mod rules;
pub mod types;
pub mod pgn;

use crate::types::Move;
// use crate::move_gen::generate_piece_moves; (removed duplicate)
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