use move_generation::types::{Board, Color, Piece};
use move_generation::move_gen::{generate_moves};

#[test]
fn test_pawn_moves() {
    let mut board = Board {
        squares: [None; 64],
        side_to_move: Color::White,
    };

    // Place a white pawn at e2 (square 12)
    board.squares[12] = Some((Piece::Pawn, Color::White));

    let moves = generate_moves(&board, Color::White);

    // Expect the pawn to move to e3 (20) and e4 (28)
    let expected_moves = vec![
        (12, 20),
        (12, 28),
    ];

    for &(from, to) in &expected_moves {
        assert!(moves.iter().any(|m| m.from == from && m.to == to));
    }
}

#[test]
fn test_knight_moves() {
    let mut board = Board {
        squares: [None; 64],
        side_to_move: Color::White,
    };

    // Place a white knight at b1 (square 1)
    board.squares[1] = Some((Piece::Knight, Color::White));

    let moves = generate_moves(&board, Color::White);

    // Expect the knight to move to a3 (16), c3 (18), and other valid squares
    let expected_moves = vec![
        (1, 16),
        (1, 18),
    ];

    for &(from, to) in &expected_moves {
        assert!(moves.iter().any(|m| m.from == from && m.to == to));
    }
}

#[test]
fn test_rook_moves() {
    let mut board = Board {
        squares: [None; 64],
        side_to_move: Color::White,
    };

    // Place a white rook at a1 (square 0)
    board.squares[0] = Some((Piece::Rook, Color::White));

    let moves = generate_moves(&board, Color::White);

    // Expect the rook to move along the first rank and first file
    let expected_moves = vec![
        (0, 8),
        (0, 16),
        (0, 24),
        (0, 32),
        (0, 40),
        (0, 48),
        (0, 56),
        (0, 1),
        (0, 2),
        (0, 3),
        (0, 4),
        (0, 5),
        (0, 6),
        (0, 7),
    ];

    for &(from, to) in &expected_moves {
        assert!(moves.iter().any(|m| m.from == from && m.to == to));
    }
}
