use crate::types::{Piece, Color};
use std::str::FromStr;
use std::fmt;
use std::io::{self, BufRead};

#[derive(Debug)]
pub enum ChessError {
    ParseError(String),
    IoError(io::Error),
}

impl From<io::Error> for ChessError {
    fn from(err: io::Error) -> ChessError {
        ChessError::IoError(err)
    }
}

impl fmt::Display for ChessError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ChessError::ParseError(s) => write!(f, "Parse error: {}", s),
            ChessError::IoError(e) => write!(f, "IO error: {}", e),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Position {
    pub pieces: Vec<(Piece, Color, u8)>,  // (piece, color, square)
    pub side_to_move: Color,
    pub castling_rights: String,
    pub en_passant: Option<u8>,
    pub halfmove_clock: u32,
    pub fullmove_number: u32,
}

impl Position {
    pub fn to_fen(&self) -> String {
        let mut board = vec![None; 64];
        for &(piece, color, square) in &self.pieces {
            board[square as usize] = Some((piece, color));
        }

        let mut fen = String::new();
        let mut empty = 0;

        for rank in (0..8).rev() {
            if rank < 7 {
                fen.push('/');
            }
            for file in 0..8 {
                let square = rank * 8 + file;
                match board[square] {
                    None => empty += 1,
                    Some((piece, color)) => {
                        if empty > 0 {
                            fen.push_str(&empty.to_string());
                            empty = 0;
                        }
                        let c = match (piece, color) {
                            (Piece::Pawn, Color::White) => 'P',
                            (Piece::Knight, Color::White) => 'N',
                            (Piece::Bishop, Color::White) => 'B',
                            (Piece::Rook, Color::White) => 'R',
                            (Piece::Queen, Color::White) => 'Q',
                            (Piece::King, Color::White) => 'K',
                            (Piece::Pawn, Color::Black) => 'p',
                            (Piece::Knight, Color::Black) => 'n',
                            (Piece::Bishop, Color::Black) => 'b',
                            (Piece::Rook, Color::Black) => 'r',
                            (Piece::Queen, Color::Black) => 'q',
                            (Piece::King, Color::Black) => 'k',
                        };
                        fen.push(c);
                    }
                }
            }
            if empty > 0 {
                fen.push_str(&empty.to_string());
                empty = 0;
            }
        }

        fen.push(' ');
        fen.push(match self.side_to_move {
            Color::White => 'w',
            Color::Black => 'b',
        });
        fen.push(' ');
        fen.push_str(&self.castling_rights);
        fen.push(' ');
        if let Some(ep) = self.en_passant {
            let file = (b'a' + (ep % 8)) as char;
            let rank = (b'1' + (ep / 8)) as char;
            fen.push(file);
            fen.push(rank);
        } else {
            fen.push_str("-");
        }
        fen.push(' ');
        fen.push_str(&self.halfmove_clock.to_string());
        fen.push(' ');
        fen.push_str(&self.fullmove_number.to_string());
        fen
    }
}

impl FromStr for Position {
    type Err = ChessError;

    fn from_str(fen: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = fen.split_whitespace().collect();
        if parts.len() < 6 {
            return Err(ChessError::ParseError("Invalid FEN: not enough fields".into()));
        }

        let position = parts[0];
        let side = parts[1];
        let castling_rights = parts[2].to_string();
        let en_passant_str = parts[3];
        let halfmove_clock = parts[4].parse::<u32>().unwrap_or(0);
        let fullmove_number = parts[5].parse::<u32>().unwrap_or(1);

        let mut pieces = Vec::new();
        let mut rank = 7;
        let mut file = 0;

        for c in position.chars() {
            match c {
                'P' => { pieces.push((Piece::Pawn, Color::White, rank * 8 + file)); file += 1; }
                'N' => { pieces.push((Piece::Knight, Color::White, rank * 8 + file)); file += 1; }
                'B' => { pieces.push((Piece::Bishop, Color::White, rank * 8 + file)); file += 1; }
                'R' => { pieces.push((Piece::Rook, Color::White, rank * 8 + file)); file += 1; }
                'Q' => { pieces.push((Piece::Queen, Color::White, rank * 8 + file)); file += 1; }
                'K' => { pieces.push((Piece::King, Color::White, rank * 8 + file)); file += 1; }
                'p' => { pieces.push((Piece::Pawn, Color::Black, rank * 8 + file)); file += 1; }
                'n' => { pieces.push((Piece::Knight, Color::Black, rank * 8 + file)); file += 1; }
                'b' => { pieces.push((Piece::Bishop, Color::Black, rank * 8 + file)); file += 1; }
                'r' => { pieces.push((Piece::Rook, Color::Black, rank * 8 + file)); file += 1; }
                'q' => { pieces.push((Piece::Queen, Color::Black, rank * 8 + file)); file += 1; }
                'k' => { pieces.push((Piece::King, Color::Black, rank * 8 + file)); file += 1; }
                '/' => { rank -= 1; file = 0; }
                '1'..='8' => { file += c.to_digit(10).unwrap() as u8; }
                _ => return Err(ChessError::ParseError("Invalid FEN character".into())),
            }
        }

        let side_to_move = match side {
            "w" => Color::White,
            "b" => Color::Black,
            _ => return Err(ChessError::ParseError("Invalid side to move".into())),
        };

        let en_passant = if en_passant_str != "-" {
            let bytes = en_passant_str.as_bytes();
            if bytes.len() == 2 {
                let file = bytes[0] - b'a';
                let rank = bytes[1] - b'1';
                Some(rank * 8 + file)
            } else {
                None
            }
        } else {
            None
        };

        Ok(Position {
            pieces,
            side_to_move,
            castling_rights,
            en_passant,
            halfmove_clock,
            fullmove_number,
        })
    }
}

pub struct PgnReader<R> {
    reader: io::BufReader<R>,
    line_buffer: String,
}

impl<R: io::Read> PgnReader<R> {
    pub fn new(reader: R) -> Self {
        PgnReader {
            reader: io::BufReader::new(reader),
            line_buffer: String::new(),
        }
    }

    pub fn next_position(&mut self) -> Result<Option<Position>, ChessError> {
        let mut in_moves = false;
        let mut fen = None;

        loop {
            self.line_buffer.clear();
            if self.reader.read_line(&mut self.line_buffer)? == 0 {
                break;
            }

            let trimmed = self.line_buffer.trim();
            if trimmed.is_empty() {
                continue;
            }

            // Check for FEN tag
            if trimmed.starts_with("[FEN \"") {
                fen = Some(trimmed[6..trimmed.len()-2].to_string());
                continue;
            }

            // Start of moves section
            if trimmed.starts_with("1.") {
                in_moves = true;
            }

            // If we're in the moves section and have a FEN, we can process it
            if in_moves {
                if let Some(fen) = fen.take() {
                    return Ok(Some(fen.parse()?));
                }
                // If no FEN was found, use the starting position
                return Ok(Some("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".parse()?));
            }
        }

        Ok(None)
    }
}


