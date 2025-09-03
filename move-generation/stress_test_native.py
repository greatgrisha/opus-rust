import time
import random
import chess
import chess.pgn
from move_generation import PyBoard

def generate_random_game(num_moves=70):
    """Generate a random game in PGN format for testing."""
    game = chess.pgn.Game()
    board = chess.Board()
    node = game
    moves_made = 0
    
    while moves_made < num_moves and not board.is_game_over():
        legal_moves = list(board.legal_moves)
        if not legal_moves:
            break
        move = random.choice(legal_moves)
        board.push(move)
        node = node.add_variation(move)
        moves_made += 1
    
    return str(game)

def stress_test(num_moves=70, num_games=10):
    """Stress test the move generator with random positions."""
    board = PyBoard()
    
    print(f"Generating {num_games} random games with up to {num_moves} moves each...")
    test_pgn = ""
    for i in range(num_games):
        test_pgn += generate_random_game(num_moves)
        test_pgn += "\n\n"
    
    # Save the test games
    with open("stress_test.pgn", "w") as f:
        f.write(test_pgn)
    
    print("\nTesting native PGN parser and move generation...")
    start_time = time.time()
    total_positions = 0
    total_moves = 0
    move_times = []
    
    try:
        # Load and process all positions using our native parser
        positions = board.load_pgn("stress_test.pgn")
        total_positions = len(positions)
        
        for fen, moves_str in positions:
            # Load position
            t0 = time.time()
            board.load_fen(fen)
            moves = board.generate_moves()
            t1 = time.time()
            move_times.append(t1 - t0)
            total_moves += len(moves)
            
        total_time = time.time() - start_time
        avg_gen_time = sum(move_times) / len(move_times) if move_times else 0
        
        print(f"\nStress Test Results:")
        print(f"Total positions processed: {total_positions}")
        print(f"Total moves generated: {total_moves}")
        print(f"Average moves per position: {total_moves/total_positions:.1f}")
        print(f"Average move generation time: {avg_gen_time*1000:.2f}ms")
        print(f"Total processing time: {total_time:.2f}s")
        
        # Validate some random moves
        print("\nValidating random moves against python-chess...")
        validation_count = min(10, total_positions)
        validated = 0
        
        for i, (fen, _) in enumerate(random.sample(positions, validation_count)):
            board.load_fen(fen)
            rust_moves = set(board.generate_moves())

            # Validate against python-chess
            py_board = chess.Board(fen)
            py_moves = {move.uci() for move in py_board.legal_moves}

            if i == 0:
                print("\n--- DEBUG: First validation position ---")
                print("FEN:", fen)
                print("Rust moves:", sorted(rust_moves))
                print("python-chess moves:", sorted(py_moves))
                print(f"Moves only in Rust: {sorted(rust_moves - py_moves)}")
                print(f"Moves only in python-chess: {sorted(py_moves - rust_moves)}")
                print("--------------------------------------\n")

            if rust_moves == py_moves:
                validated += 1
        print(f"Successfully validated {validated}/{validation_count} positions")
        
    except Exception as e:
        print(f"Error during test: {e}")
        raise

# Example with sample PGN data
if __name__ == "__main__":
    stress_test(num_moves=70, num_games=10)
