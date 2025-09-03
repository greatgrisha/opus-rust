import time
from move_generation import PyBoard

# Example: Generate all moves for the current board
board = PyBoard()
moves = board.generate_moves()
print("All moves:", moves)

# Example: Parallel move generation for multiple pieces
piece_sq_list = [("rook", 0), ("knight", 1), ("bishop", 2), ("queen", 3)]
parallel_moves = board.generate_moves_for_pieces_parallel(piece_sq_list)
print("Parallel moves for pieces:", parallel_moves)

# Timing efficiency demo
start = time.time()
for _ in range(1000):
    board.generate_moves()
print(f"1000x generate_moves: {time.time() - start:.4f}s")

start = time.time()
for _ in range(1000):
    board.generate_moves_for_pieces_parallel(piece_sq_list)
print(f"1000x generate_moves_for_pieces_parallel: {time.time() - start:.4f}s")
