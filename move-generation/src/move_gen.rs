//! Move generation for fast chess library

use crate::types::{Board, Color, Move, Piece};

/// Bitboard representation for fast move generation
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Bitboard(u64);

impl Bitboard {
    /// Create an empty bitboard
    pub fn empty() -> Self {
        Bitboard(0)
    }

    /// Set a bit at a specific square
    pub fn set(&mut self, sq: u8) {
        self.0 |= 1 << sq;
    }

    /// Clear a bit at a specific square
    pub fn clear(&mut self, sq: u8) {
        self.0 &= !(1 << sq);
    }

    /// Check if a bit is set at a specific square
    pub fn is_set(&self, sq: u8) -> bool {
        (self.0 & (1 << sq)) != 0
    }

    /// Get all set bits as a vector of square indices
    pub fn bits(&self) -> Vec<u8> {
        let mut bits = vec![];
        let mut bb = self.0;
        while bb != 0 {
            let sq = bb.trailing_zeros() as u8;
            bits.push(sq);
            bb &= bb - 1; // Clear the least significant bit
        }
        bits
    }
}

/// Precomputed attack tables for sliding pieces
pub struct AttackTables {
    pub rook_attacks: Vec<Bitboard>,
    pub bishop_attacks: Vec<Bitboard>,
}

impl AttackTables {
    /// Initialize attack tables
    pub fn new() -> Self {
        let mut rook_attacks = vec![Bitboard::empty(); 64];
        let mut bishop_attacks = vec![Bitboard::empty(); 64];

        for sq in 0..64 {
            rook_attacks[sq] = compute_rook_attacks(sq.try_into().unwrap());
            bishop_attacks[sq] = compute_bishop_attacks(sq.try_into().unwrap());
        }

        AttackTables {
            rook_attacks,
            bishop_attacks,
        }
    }
}

/// Compute rook attack mask for a given square
fn compute_rook_attacks(sq: u8) -> Bitboard {
    let mut attacks = Bitboard::empty();

    // Horizontal and vertical directions
    let directions = [-8, -1, 1, 8];

    for &dir in &directions {
        let mut current_sq = sq as i8;
        while let Some(next_sq) = step_in_direction(current_sq, dir) {
            attacks.set(next_sq as u8);
            current_sq = next_sq;
        }
    }

    attacks
}

/// Compute bishop attack mask for a given square
fn compute_bishop_attacks(sq: u8) -> Bitboard {
    let mut attacks = Bitboard::empty();

    // Diagonal directions
    let directions = [-9, -7, 7, 9];

    for &dir in &directions {
        let mut current_sq = sq as i8;
        while let Some(next_sq) = step_in_direction(current_sq, dir) {
            attacks.set(next_sq as u8);
            current_sq = next_sq;
        }
    }

    attacks
}

/// Step in a direction and return the next square, or None if out of bounds
fn step_in_direction(sq: i8, dir: i8) -> Option<i8> {
    let next_sq = sq + dir;

    if next_sq < 0 || next_sq >= 64 {
        return None;
    }

    let same_row = (sq / 8) == (next_sq / 8);
    let same_col = (sq % 8) == (next_sq % 8);

    if dir.abs() == 1 && !same_row {
        return None;
    }

    if dir.abs() == 8 && !same_col {
        return None;
    }

    Some(next_sq)
}

/// Generate all legal moves for a given color
pub fn generate_moves(board: &Board, color: Color) -> Vec<Move> {
    let mut moves = vec![];

    // Iterate over all squares
    for (sq, piece) in board.squares.iter().enumerate() {
        if let Some((p, c)) = piece {
            if *c == color {
                moves.extend(generate_piece_moves(board, *p, sq as u8));
            }
        }
    }

    moves
}

/// Generate all valid moves for a specific piece at a square
pub fn generate_piece_moves(board: &Board, piece: Piece, sq: u8) -> Vec<Move> {
    match piece {
        Piece::Pawn => generate_pawn_moves(board, sq),
        Piece::Knight => generate_knight_moves(sq),
        Piece::Bishop => generate_bishop_moves(board, sq),
        Piece::Rook => generate_rook_moves(board, sq),
        Piece::Queen => generate_queen_moves(board, sq),
        Piece::King => generate_king_moves(board, sq),
    }
}

/// Generate pawn moves
fn generate_pawn_moves(board: &Board, sq: u8) -> Vec<Move> {
    let mut moves = vec![];
    let direction = match board.side_to_move {
        Color::White => -8,
        Color::Black => 8,
    };

    let forward_sq = sq as i8 + direction;
    if forward_sq >= 0 && forward_sq < 64 && board.squares[forward_sq as usize].is_none() {
        moves.push(Move {
            from: sq,
            to: forward_sq as u8,
            promotion: None,
        });

        // Double move for pawns on their starting rank
        let starting_rank = match board.side_to_move {
            Color::White => 6,
            Color::Black => 1,
        };
        if sq / 8 == starting_rank {
            let double_forward_sq = forward_sq + direction;
            if double_forward_sq >= 0 && double_forward_sq < 64 && board.squares[double_forward_sq as usize].is_none() {
                moves.push(Move {
                    from: sq,
                    to: double_forward_sq as u8,
                    promotion: None,
                });
            }
        }
    }

    // Captures
    let capture_directions = match board.side_to_move {
        Color::White => [-9, -7],
        Color::Black => [7, 9],
    };
    for &cap_dir in &capture_directions {
        let capture_sq = sq as i8 + cap_dir;
        if capture_sq >= 0 && capture_sq < 64 {
            if let Some((_, color)) = board.squares[capture_sq as usize] {
                if color != board.side_to_move {
                    moves.push(Move {
                        from: sq,
                        to: capture_sq as u8,
                        promotion: None,
                    });
                }
            }
        }
    }

    moves
}

/// Generate knight moves
fn generate_knight_moves(sq: u8) -> Vec<Move> {
    let mut moves = vec![];
    let knight_offsets = [-17, -15, -10, -6, 6, 10, 15, 17];

    for &offset in &knight_offsets {
        let target_sq = sq as i8 + offset;
        if target_sq >= 0 && target_sq < 64 {
            moves.push(Move {
                from: sq,
                to: target_sq as u8,
                promotion: None,
            });
        }
    }

    moves
}

/// Generate queen moves
fn generate_queen_moves(board: &Board, sq: u8) -> Vec<Move> {
    let mut moves = vec![];
    moves.extend(generate_rook_moves(board, sq));
    moves.extend(generate_bishop_moves(board, sq));
    moves
}

/// Generate king moves
fn generate_king_moves(board: &Board, sq: u8) -> Vec<Move> {
    let mut moves = vec![];
    let king_offsets = [-9, -8, -7, -1, 1, 7, 8, 9];

    for &offset in &king_offsets {
        let target_sq = sq as i8 + offset;
        if target_sq >= 0 && target_sq < 64 {
            if let Some((_, color)) = board.squares[target_sq as usize] {
                if color != board.side_to_move {
                    moves.push(Move {
                        from: sq,
                        to: target_sq as u8,
                        promotion: None,
                    });
                }
            } else {
                moves.push(Move {
                    from: sq,
                    to: target_sq as u8,
                    promotion: None,
                });
            }
        }
    }

    moves
}

/// Generate bishop moves
fn generate_bishop_moves(board: &Board, sq: u8) -> Vec<Move> {
    let mut moves = vec![];
    let attacks = compute_bishop_attacks(sq);

    for target_sq in attacks.bits() {
        if let Some((_, color)) = board.squares[target_sq as usize] {
            if color != board.side_to_move {
                moves.push(Move {
                    from: sq,
                    to: target_sq,
                    promotion: None,
                });
            }
        } else {
            moves.push(Move {
                from: sq,
                to: target_sq,
                promotion: None,
            });
        }
    }

    moves
}

/// Generate rook moves
fn generate_rook_moves(board: &Board, sq: u8) -> Vec<Move> {
    let mut moves = vec![];
    let attacks = compute_rook_attacks(sq);

    for target_sq in attacks.bits() {
        if let Some((_, color)) = board.squares[target_sq as usize] {
            if color != board.side_to_move {
                moves.push(Move {
                    from: sq,
                    to: target_sq,
                    promotion: None,
                });
            }
        } else {
            moves.push(Move {
                from: sq,
                to: target_sq,
                promotion: None,
            });
        }
    }

    moves
}