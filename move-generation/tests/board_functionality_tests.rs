use move_generation::types::{Board, Color, Piece};
use move_generation::move_gen::generate_moves;

#[test]
fn test_random_board_position() {
    let board = Board {
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
    };

    let moves = generate_moves(&board, Color::White);

    // Debugging output
    println!("Generated moves: {:?}", moves);

    // TODO: Validate moves against Stockfish or PyChess
    assert!(!moves.is_empty(), "No moves generated for the given board position.");
}
