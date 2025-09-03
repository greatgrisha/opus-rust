import chess
import chess.pgn
import io
import time
import random
from move_generation import PyBoard

def load_position_from_fen(fen):
    """Convert a FEN position into a format our move generator can use."""
    board = chess.Board(fen)
    pieces = []
    
    # Map python-chess piece symbols to our format
    piece_map = {
        'P': 'pawn', 'N': 'knight', 'B': 'bishop',
        'R': 'rook', 'Q': 'queen', 'K': 'king'
    }
    
    # Go through each square
    for sq in chess.SQUARES:
        piece = board.piece_at(sq)
        if piece:
            piece_type = piece_map[piece.symbol().upper()]
            color = "white" if piece.color else "black"
            pieces.append((piece_type, color, sq))
    
    return {
        "pieces": pieces,
        "side_to_move": "white" if board.turn else "black"
    }

def load_pgn_positions(pgn_string):
    """Extract positions from a PGN game."""
    game = chess.pgn.read_game(io.StringIO(pgn_string))
    positions = []
    board = game.board()
    
    # Get starting position
    positions.append(load_position_from_fen(board.fen()))
    
    # Go through each move and get the resulting positions
    for move in game.mainline_moves():
        board.push(move)
        positions.append(load_position_from_fen(board.fen()))
    
    return positions

def random_move_stress_test(num_moves=70):
    """Do a stress test by making random legal moves for a specified number of moves."""
    board = PyBoard()
    chess_board = chess.Board()  # For validation
    start_time = time.time()
    moves_made = 0
    total_moves_generated = 0
    move_gen_times = []

    while moves_made < num_moves and not chess_board.is_game_over():
        # Convert current position
        position = load_position_from_fen(chess_board.fen())
        
        # Set up our board
        board.set_pieces(position["pieces"])
        board.set_side_to_move(position["side_to_move"])
        
        # Generate and time moves
        t0 = time.time()
        legal_moves = board.generate_moves()
        t1 = time.time()
        move_gen_times.append(t1 - t0)
        total_moves_generated += len(legal_moves)
        
        # Convert moves to python-chess format and make a random move
        move_list = []
        for move_str in legal_moves:
            try:
                move = chess.Move.from_uci(move_str)
                if move in chess_board.legal_moves:
                    move_list.append(move)
            except ValueError:
                continue
        
        if not move_list:
            break
            
        # Make a random move
        random_move = random.choice(move_list)
        chess_board.push(random_move)
        moves_made += 1
        
        # Print progress
        if moves_made % 10 == 0:
            print(f"Made {moves_made} moves...")

    total_time = time.time() - start_time
    avg_gen_time = sum(move_gen_times) / len(move_gen_times) if move_gen_times else 0
    
    print(f"\nStress Test Results:")
    print(f"Total moves made: {moves_made}")
    print(f"Total legal moves generated: {total_moves_generated}")
    print(f"Average moves per position: {total_moves_generated/moves_made:.1f}")
    print(f"Average move generation time: {avg_gen_time*1000:.2f}ms")
    print(f"Total test time: {total_time:.2f}s")
    print(f"Final position:")
    print(chess_board)

# Example usage
if __name__ == "__main__":
    print("Running 70-move stress test...")
    random_move_stress_test(70)
    
    # Example with a PGN game
    sample_pgn = '''
    [Event "Example Game"]
    [Site "?"]
    [Date "2025.09.03"]
    [Round "1"]
    [White "Test"]
    [Black "Test"]
    [Result "*"]

    1. e4 e5 2. Nf3 Nc6 3. Bb5 a6 4. Ba4 Nf6 5. O-O Be7 *
    '''
    
    print("\nTesting PGN position extraction...")
    positions = load_pgn_positions(sample_pgn)
    print(f"Extracted {len(positions)} positions from PGN")
    
    # Generate moves for the last position
    board = PyBoard()
    last_pos = positions[-1]
    board.set_pieces(last_pos["pieces"])
    board.set_side_to_move(last_pos["side_to_move"])
    moves = board.generate_moves()
    print(f"Moves in final position: {len(moves)}")
