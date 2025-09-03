//! Types for fast chess move generation

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Color {
    White,
    Black,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Piece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Move {
    pub from: u8, // 0..63
    pub to: u8,   // 0..63
    pub promotion: Option<Piece>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Board {
    // 0..63 squares, None if empty, Some((Piece, Color)) if occupied
    pub squares: [Option<(Piece, Color)>; 64],
    pub side_to_move: Color,
    pub castling_rights: String, // e.g. "KQkq"
    pub en_passant: Option<u8>,  // Square index or None
    pub halfmove_clock: u32,
    pub fullmove_number: u32,
}