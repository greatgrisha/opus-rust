import time
import os
import psutil
import multiprocessing
from move_generation import PyBoard

# Helper to measure memory and CPU usage
def resource_usage():
    process = psutil.Process(os.getpid())
    mem = process.memory_info().rss / (1024 * 1024)  # MB
    cpu = process.cpu_percent(interval=0.1)
    return mem, cpu

# Generate a list of (piece, square) tuples for parallel test
def generate_piece_sq_list():
    # Place a piece on every square for stress test
    pieces = ["pawn", "knight", "bishop", "rook", "queen", "king"]
    return [(pieces[i % 6], sq) for i, sq in enumerate(range(64))]

def benchmark_parallel(board, piece_sq_list):
    start_mem, start_cpu = resource_usage()
    start = time.time()
    result = board.generate_moves_for_pieces_parallel(piece_sq_list)
    elapsed = time.time() - start
    end_mem, end_cpu = resource_usage()
    print(f"Parallel: {len(result)} pieces, {elapsed:.4f}s, mem: {end_mem-start_mem:.2f}MB, cpu: {end_cpu-start_cpu:.2f}%")
    return elapsed, end_mem-start_mem, end_cpu-start_cpu

def benchmark_serial(board, piece_sq_list):
    start_mem, start_cpu = resource_usage()
    start = time.time()
    result = [board.generate_moves_for_pieces_parallel([(piece, sq)]) for piece, sq in piece_sq_list]
    elapsed = time.time() - start
    end_mem, end_cpu = resource_usage()
    print(f"Serial: {len(result)} pieces, {elapsed:.4f}s, mem: {end_mem-start_mem:.2f}MB, cpu: {end_cpu-start_cpu:.2f}%")
    return elapsed, end_mem-start_mem, end_cpu-start_cpu

def run_benchmarks():
    board = PyBoard()
    piece_sq_list = generate_piece_sq_list()
    print("\n--- High CPU/Memory (all squares, parallel vs serial) ---")
    benchmark_parallel(board, piece_sq_list)
    benchmark_serial(board, piece_sq_list)

    print("\n--- Low CPU/Memory (few pieces, parallel vs serial) ---")
    small_list = piece_sq_list[:4]
    benchmark_parallel(board, small_list)
    benchmark_serial(board, small_list)

    print("\n--- System Info ---")
    print(f"CPUs: {multiprocessing.cpu_count()} | RAM: {psutil.virtual_memory().total // (1024*1024)} MB")

if __name__ == "__main__":
    run_benchmarks()
