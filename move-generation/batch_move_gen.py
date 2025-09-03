import json
from move_generation import PyBoard

def load_positions(filename):
    """Load chess positions from a JSON dataset file.
    Expected format:
    [
        {
            "pieces": [["piece_type", "color", square], ...],
            "side_to_move": "white"|"black"
        },
        ...
    ]
    """
    with open(filename) as f:
        return json.load(f)

def batch_generate_moves(positions):
    """Generate moves for multiple positions efficiently."""
    board = PyBoard()
    results = []
    
    for pos in positions:
        # Convert pieces to tuples (Rust expects tuples, not lists)
        pieces = [tuple(piece) for piece in pos["pieces"]]
        
        # Set up the position
        board.set_pieces(pieces)
        board.set_side_to_move(pos["side_to_move"])
        
        # Generate moves for this position
        moves = board.generate_moves()
        results.append(moves)
    
    return results

# Example usage
if __name__ == "__main__":
    # Example position format
    example_positions = [
        {
            "pieces": [
                ["rook", "white", 0],   # a1
                ["king", "white", 4],   # e1
                ["rook", "white", 7],   # h1
                ["rook", "black", 56],  # a8
                ["king", "black", 60],  # e8
                ["rook", "black", 63],  # h8
            ],
            "side_to_move": "white"
        },
        {
            "pieces": [
                ["queen", "white", 0],
                ["king", "white", 4],
                ["bishop", "black", 63],
                ["king", "black", 60],
            ],
            "side_to_move": "black"
        }
    ]
    
    # Save example to file
    with open("example_positions.json", "w") as f:
        json.dump(example_positions, f, indent=2)
    
    # Process positions
    print("Loading positions from example_positions.json...")
    positions = load_positions("example_positions.json")
    
    print("\nGenerating moves for each position...")
    all_moves = batch_generate_moves(positions)
    
    # Print results
    for i, moves in enumerate(all_moves):
        print(f"\nPosition {i+1} moves:")
        print(moves)
