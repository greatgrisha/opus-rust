//! Move generation for fast chess library

use crate::types::{Board, Color, Move, Piece};
use std::ops::BitOr;

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

    /// Check if the bitboard contains a specific square
    pub fn contains(&self, sq: u8) -> bool {
        self.0 & (1 << sq) != 0
    }
}
impl BitOr for Bitboard {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 | rhs.0)
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
// pub fn generate_piece_moves(board: &Board, piece: Piece, sq: u8) -> Vec<Move> {
//     match piece {
//         Piece::Pawn => generate_pawn_moves(board, sq),
//         Piece::Knight => generate_knight_moves(sq),
//         Piece::Bishop => generate_bishop_moves(board, sq),
//         Piece::Rook => generate_rook_moves(board, sq),
//         Piece::Queen => generate_queen_moves(board, sq),
//         Piece::King => generate_king_moves(board, sq),
//     }
// }

/// Generate pawn moves
fn generate_pawn_moves(board: &Board, sq: u8) -> Vec<Move> {
    let mut moves = vec![];
    let direction = match board.side_to_move {
        Color::White => 8,  // White pawns move up the board
        Color::Black => -8, // Black pawns move down the board
    };

    // Single forward move
    let forward_sq = sq as i8 + direction;
    if forward_sq >= 0 && forward_sq < 64 && board.squares[forward_sq as usize].is_none() {
        moves.push(Move {
            from: sq,
            to: forward_sq as u8,
            promotion: None,
        });

        // Double forward move (only from starting rank)
        let starting_rank = match board.side_to_move {
            Color::White => 1,
            Color::Black => 6,
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
        Color::White => [7, 9],  // Diagonal captures for white
        Color::Black => [-7, -9], // Diagonal captures for black
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

    // En Passant
    if let Some(en_passant_sq) = get_en_passant_square(board) {
        let en_passant_directions = match board.side_to_move {
            Color::White => [7, 9],
            Color::Black => [-7, -9],
        };
        for &ep_dir in &en_passant_directions {
            let target_sq = sq as i8 + ep_dir;
            if target_sq == en_passant_sq as i8 {
                moves.push(Move {
                    from: sq,
                    to: en_passant_sq,
                    promotion: None,
                });
            }
        }
    }

    moves
}

/// Get the en passant square, if available
fn get_en_passant_square(board: &Board) -> Option<u8> {
    board.en_passant
}

/// Generate knight moves
fn generate_knight_moves(board: &Board, sq: u8) -> Vec<Move> {
    let mut moves = vec![];
    let knight_offsets = [-17, -15, -10, -6, 6, 10, 15, 17];
    let from_rank = (sq / 8) as i8;
    let from_file = (sq % 8) as i8;
    for &offset in &knight_offsets {
        let target_sq = sq as i8 + offset;
        if target_sq < 0 || target_sq >= 64 {
            continue;
        }
        let to_rank = (target_sq / 8) as i8;
        let to_file = (target_sq % 8) as i8;
        let dr = (from_rank - to_rank).abs();
        let df = (from_file - to_file).abs();
        // Must be a knight move (2,1) or (1,2)
        if !((dr == 2 && df == 1) || (dr == 1 && df == 2)) {
            continue;
        }
        // Can't land on own piece
        if let Some((_, color)) = board.squares[target_sq as usize] {
            if color == board.side_to_move {
                continue;
            }
        }
        moves.push(Move {
            from: sq,
            to: target_sq as u8,
            promotion: None,
        });
    }
        fn generate_bishop_moves(board: &Board, sq: u8) -> Vec<Move> {
                let mut moves = vec![];
                let directions = [-9, -7, 7, 9];
                for &dir in &directions {
                    let mut current_sq = sq as i8;
                    loop {
                        let next_sq = current_sq + dir;
                        if next_sq < 0 || next_sq >= 64 {
                            break;
                        }
                        let from_rank = current_sq / 8;
                        let from_file = current_sq % 8;
                        let to_rank = next_sq / 8;
                        let to_file = next_sq % 8;
                        // Prevent wrapping
                        if (from_rank - to_rank).abs() != 1 || (from_file - to_file).abs() != 1 {
                            break;
                        }
                        if let Some((_, color)) = board.squares[next_sq as usize] {
                            if color != board.side_to_move {
                                moves.push(Move {
                                    from: sq,
                                    to: next_sq as u8,
                                    promotion: None,
                                });
                            }
                            break;
                        } else {
                            moves.push(Move {
                                from: sq,
                                to: next_sq as u8,
                                promotion: None,
                            });
                        }
                        current_sq = next_sq;
                    }
                }
                moves
            }

            fn generate_rook_moves(board: &Board, sq: u8) -> Vec<Move> {
                let mut moves = vec![];
                let directions = [-8, -1, 1, 8];
                for &dir in &directions {
                    let mut current_sq = sq as i8;
                    loop {
                        let next_sq = current_sq + dir;
                        if next_sq < 0 || next_sq >= 64 {
                            break;
                        }
                        let from_rank = current_sq / 8;
                        let from_file = current_sq % 8;
                        let to_rank = next_sq / 8;
                        let to_file = next_sq % 8;
                        // Prevent wrapping
                        if dir == -1 && to_file > from_file { break; }
                        if dir == 1 && to_file < from_file { break; }
                        if dir == -8 && to_rank > from_rank { break; }
                        if dir == 8 && to_rank < from_rank { break; }
                        if let Some((_, color)) = board.squares[next_sq as usize] {
                            if color != board.side_to_move {
                                moves.push(Move {
                                    from: sq,
                                    to: next_sq as u8,
                                    promotion: None,
                                });
                            }
                            break;
                        } else {
                            moves.push(Move {
                                from: sq,
                                to: next_sq as u8,
                                promotion: None,
                            });
                        }
                        current_sq = next_sq;
                    }
                }
                moves
            }

    moves
}

/// Generate queen moves
// fn generate_queen_moves(board: &Board, sq: u8) -> Vec<Move> {
//     let mut moves = vec![];
//     moves.extend(generate_rook_moves(board, sq));
//     moves.extend(generate_bishop_moves(board, sq));
//     moves
// }
    pub fn generate_piece_moves(board: &Board, piece: Piece, sq: u8) -> Vec<Move> {
    match piece {
        Piece::Pawn => generate_pawn_moves(board, sq),
    Piece::Knight => generate_knight_moves(board, sq),
        Piece::Bishop => generate_bishop_moves(board, sq),
        Piece::Rook => generate_rook_moves(board, sq),
        Piece::Queen => {
            let mut moves = generate_rook_moves(board, sq);
            moves.extend(generate_bishop_moves(board, sq));
            moves
        },
        Piece::King => generate_king_moves(board, sq),
    }
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

    // Castling
    if can_castle_kingside(board) {
        moves.push(Move {
            from: sq,
            to: sq + 2, // Kingside castling
            promotion: None,
        });
    }
    if can_castle_queenside(board) {
        moves.push(Move {
            from: sq,
            to: sq - 2, // Queenside castling
            promotion: None,
        });
    }

    moves
}

/// Check if kingside castling is possible
fn can_castle_kingside(board: &Board) -> bool {
    let (king_sq, rook_sq, right_char) = match board.side_to_move {
        Color::White => (4, 7, 'K'), // e1, h1
        Color::Black => (60, 63, 'k'), // e8, h8
    };
    // Check castling rights from FEN
    if !board.castling_rights.contains(right_char) {
        return false;
    }
    // Ensure the squares between the king and rook are empty
    let between_squares = match board.side_to_move {
        Color::White => [5, 6], // f1, g1
        Color::Black => [61, 62], // f8, g8
    };
    if !are_squares_empty(board, &between_squares) {
        return false;
    }
    // Ensure the king does not move through or into check
    let king_path = match board.side_to_move {
        Color::White => [4, 5, 6], // e1, f1, g1
        Color::Black => [60, 61, 62], // e8, f8, g8
    };
    if is_any_square_attacked(board, &king_path, board.side_to_move) {
        return false;
    }
    true
}

/// Check if queenside castling is possible
fn can_castle_queenside(board: &Board) -> bool {
    let (king_sq, rook_sq, right_char) = match board.side_to_move {
        Color::White => (4, 0, 'Q'), // e1, a1
        Color::Black => (60, 56, 'q'), // e8, a8
    };
    // Check castling rights from FEN
    if !board.castling_rights.contains(right_char) {
        return false;
    }
    // Ensure the squares between the king and rook are empty
    let between_squares = match board.side_to_move {
        Color::White => [1, 2, 3], // b1, c1, d1
        Color::Black => [57, 58, 59], // b8, c8, d8
    };
    if !are_squares_empty(board, &between_squares) {
        return false;
    }
    // Ensure the king does not move through or into check
    let king_path = match board.side_to_move {
        Color::White => [4, 3, 2], // e1, d1, c1
        Color::Black => [60, 59, 58], // e8, d8, c8
    };
    if is_any_square_attacked(board, &king_path, board.side_to_move) {
        return false;
    }
    true
}

/// Check if the king and rook have not moved
fn has_king_and_rook_not_moved(_board: &Board, _king_sq: u8, _rook_sq: u8) -> bool {
    // FEN castling rights now checked in can_castle_kingside/queenside
    true
}

/// Check if all squares in a given list are empty
fn are_squares_empty(board: &Board, squares: &[u8]) -> bool {
    squares.iter().all(|&sq| board.squares[sq as usize].is_none())
}

/// Check if any square in a given list is attacked
fn is_any_square_attacked(board: &Board, squares: &[u8], color: Color) -> bool {
    squares.iter().any(|&sq| is_square_attacked(board, sq, color))
}

/// Check if a square is attacked by the opponent
fn is_square_attacked(board: &Board, sq: u8, color: Color) -> bool {
    for (i, piece) in board.squares.iter().enumerate() {
        if let Some((p, c)) = piece {
            if *c != color {
                match p {
                    Piece::Pawn => {
                        let attack_offsets = match c {
                            Color::White => [-7, -9],
                            Color::Black => [7, 9],
                        };
                        for &offset in &attack_offsets {
                            let target_sq = i as i8 + offset;
                            if target_sq >= 0 && target_sq < 64 && target_sq as u8 == sq {
                                return true;
                            }
                        }
                    }
                    Piece::Knight => {
                        let knight_offsets = [-17, -15, -10, -6, 6, 10, 15, 17];
                        for &offset in &knight_offsets {
                            let target_sq = i as i8 + offset;
                            if target_sq >= 0 && target_sq < 64 && target_sq as u8 == sq {
                                return true;
                            }
                        }
                    }
                    Piece::Bishop | Piece::Rook | Piece::Queen => {
                        let attacks = match p {
                            Piece::Bishop => compute_bishop_attacks(i as u8),
                            Piece::Rook => compute_rook_attacks(i as u8),
                            Piece::Queen => compute_bishop_attacks(i as u8) | compute_rook_attacks(i as u8),
                            _ => unreachable!(),
                        };
                        if attacks.contains(sq) {
                            return true;
                        }
                    }
                    Piece::King => {
                        let king_offsets = [-9, -8, -7, -1, 1, 7, 8, 9];
                        for &offset in &king_offsets {
                            let target_sq = i as i8 + offset;
                            if target_sq >= 0 && target_sq < 64 && target_sq as u8 == sq {
                                return true;
                            }
                        }
                    }
                }
            }
        }
    }
    false
}

/// Generate bishop moves
fn generate_bishop_moves(board: &Board, sq: u8) -> Vec<Move> {
    let mut moves = vec![];
    let directions = [-9i8, -7i8, 7i8, 9i8];
    for &dir in &directions {
        let mut current_sq = sq as i8;
        loop {
            let next_sq = current_sq + dir;
            if next_sq < 0 || next_sq >= 64 {
                break;
            }
            let from_rank = (current_sq / 8) as i8;
            let from_file = (current_sq % 8) as i8;
            let to_rank = (next_sq / 8) as i8;
            let to_file = (next_sq % 8) as i8;
            // Prevent wrapping: diagonal steps must change both rank and file by 1
            if (from_rank - to_rank).abs() != 1 || (from_file - to_file).abs() != 1 {
                break;
            }
            if let Some((_, color)) = board.squares[next_sq as usize] {
                if color != board.side_to_move {
                    moves.push(Move { from: sq, to: next_sq as u8, promotion: None });
                }
                break; // blocked
            } else {
                moves.push(Move { from: sq, to: next_sq as u8, promotion: None });
            }
            current_sq = next_sq;
        }
    }
    moves
}

/// Generate rook moves
fn generate_rook_moves(board: &Board, sq: u8) -> Vec<Move> {
    let mut moves = vec![];
    let directions = [-8i8, -1i8, 1i8, 8i8];
    for &dir in &directions {
        let mut current_sq = sq as i8;
        loop {
            let next_sq = current_sq + dir;
            if next_sq < 0 || next_sq >= 64 {
                break;
            }
            let from_rank = (current_sq / 8) as i8;
            let from_file = (current_sq % 8) as i8;
            let to_rank = (next_sq / 8) as i8;
            let to_file = (next_sq % 8) as i8;
            // Prevent wrapping for horizontal moves
            if dir == -1 && to_file > from_file { break; }
            if dir == 1 && to_file < from_file { break; }
            if dir == -8 && to_rank > from_rank { break; }
            if dir == 8 && to_rank < from_rank { break; }
            if let Some((_, color)) = board.squares[next_sq as usize] {
                if color != board.side_to_move {
                    moves.push(Move { from: sq, to: next_sq as u8, promotion: None });
                }
                break; // blocked
            } else {
                moves.push(Move { from: sq, to: next_sq as u8, promotion: None });
            }
            current_sq = next_sq;
        }
    }
    moves
}