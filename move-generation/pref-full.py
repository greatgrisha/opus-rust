#!/usr/bin/env python3
"""
Comprehensive performance test for move-generation Rust extension.

Modes:
  - light  : small, quick (smoke tests, local profiling)
  - medium : training-grade, many positions (default for validation/bench)
  - heavy  : very large workloads (for long-running cluster jobs)

What it does:
  - spawns N worker processes
  - each worker repeatedly generates random playout positions (using python-chess)
    and loads the FEN into the Rust-backed `PyBoard` then calls `generate_moves`
    to measure move-generation throughput.
  - collects timings, memory and cpu usage and prints a detailed report.

Usage examples:
  python perf_full.py --mode light --workers 2 --positions 1000
  python perf_full.py --mode medium --workers 8 --positions 100000

Notes:
  - medium/heavy modes can be very CPU / memory intensive.
  - heavy defaults are intentionally conservative; increase only on beefy machines.
"""

import argparse
import time
import os
import statistics
import multiprocessing
from concurrent.futures import ProcessPoolExecutor, as_completed
import psutil
import random
from typing import Dict, Any, Tuple, List

# Local imports (PyO3 extension)
from move_generation import PyBoard
import chess


def worker_loop(worker_id: int, positions: int, depth: int, seed: int) -> Dict[str, Any]:
    random.seed(seed)
    py_board = PyBoard()
    stats = {
        'worker_id': worker_id,
        'positions': 0,
        'moves_generated': 0,
        'position_times': [],
        'start_time': time.time(),
    }

    # We'll create random playouts using python-chess for diversity
    for i in range(positions):
        # Generate a random playout up to `depth` moves from start
        b = chess.Board()
        moves_made = 0
        while moves_made < depth and not b.is_game_over():
            legal = list(b.legal_moves)
            if not legal:
                break
            mv = random.choice(legal)
            b.push(mv)
            moves_made += 1

        fen = b.fen()
        t0 = time.time()
        py_board.load_fen(fen)
        moves = py_board.generate_moves()  # list of UCI strings
        t1 = time.time()
        elapsed = t1 - t0

        stats['positions'] += 1
        stats['moves_generated'] += len(moves)
        stats['position_times'].append(elapsed)

        # Lightweight backpressure to avoid locking the system fully in very heavy tests
        if i % 1000 == 0:
            time.sleep(0)

    stats['end_time'] = time.time()
    return stats


def aggregate_results(worker_results: List[Dict[str, Any]]) -> Dict[str, Any]:
    total_positions = sum(w['positions'] for w in worker_results)
    total_moves = sum(w['moves_generated'] for w in worker_results)
    all_times = [t for w in worker_results for t in w['position_times']]

    aggregated = {
        'total_positions': total_positions,
        'total_moves': total_moves,
        'avg_moves_per_position': total_moves / total_positions if total_positions else 0,
        'total_time': max((w['end_time'] for w in worker_results)) - min((w['start_time'] for w in worker_results)),
        'mean_time_per_position': statistics.mean(all_times) if all_times else 0,
        'median_time_per_position': statistics.median(all_times) if all_times else 0,
        'p95_time_per_position': statistics.quantiles(all_times, n=100)[94] if len(all_times) >= 100 else max(all_times) if all_times else 0,
        'moves_per_second': (total_moves / sum(all_times)) if sum(all_times) > 0 else 0,
    }
    return aggregated


def human(n: float) -> str:
    if n > 1e9:
        return f"{n/1e9:.2f}B"
    if n > 1e6:
        return f"{n/1e6:.2f}M"
    if n > 1e3:
        return f"{n/1e3:.2f}k"
    return f"{n:.0f}"


def run(mode: str, workers: int, positions: int, depth: int, seed: int, parallel_kind: str):
    print(f"Perf test starting: mode={mode}, workers={workers}, positions={positions}, depth={depth}, seed={seed}, parallel={parallel_kind}")
    start = time.time()
    sys_start_mem = psutil.Process(os.getpid()).memory_info().rss / (1024*1024)

    worker_args = []
    for wid in range(workers):
        worker_args.append((wid, positions, depth, seed + wid))

    results = []
    # Run workers in separate processes to get real parallelism
    with ProcessPoolExecutor(max_workers=workers) as ex:
        futures = [ex.submit(worker_loop, *args) for args in worker_args]
        for f in as_completed(futures):
            try:
                res = f.result()
                results.append(res)
                # lightweight per-worker progress
                print(f"Worker {res['worker_id']} done: positions={res['positions']} moves={res['moves_generated']} avg_pos_time={(sum(res['position_times'])/len(res['position_times'])) if res['position_times'] else 0:.6f}s")
            except Exception as e:
                print(f"Worker failed: {e}")

    aggregated = aggregate_results(results)
    end = time.time()
    sys_end_mem = psutil.Process(os.getpid()).memory_info().rss / (1024*1024)

    print("\n--- Performance Summary ---")
    print(f"Mode: {mode}")
    print(f"Workers: {workers}")
    print(f"Total positions: {aggregated['total_positions']}")
    print(f"Total moves generated: {human(aggregated['total_moves'])}")
    print(f"Avg moves per position: {aggregated['avg_moves_per_position']:.2f}")
    print(f"Total wall time: {end - start:.2f}s")
    print(f"Measured total time (workers span): {aggregated['total_time']:.2f}s")
    print(f"Mean time per position: {aggregated['mean_time_per_position']*1000:.6f} ms")
    print(f"Median time per position: {aggregated['median_time_per_position']*1000:.6f} ms")
    print(f"P95 time per position: {aggregated['p95_time_per_position']*1000:.6f} ms")
    print(f"Moves/sec (measured): {aggregated['moves_per_second']:.0f}")
    print(f"Memory delta (main process): {sys_end_mem - sys_start_mem:.2f} MB")
    print("---------------------------\n")


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description="Full performance test for move generation")
    parser.add_argument('--mode', choices=['light', 'medium', 'heavy'], default='medium')
    parser.add_argument('--workers', type=int, default=max(1, multiprocessing.cpu_count() // 2))
    parser.add_argument('--positions', type=int, default=None, help='positions per worker')
    parser.add_argument('--depth', type=int, default=None, help='random playout depth')
    parser.add_argument('--seed', type=int, default=42)
    parser.add_argument('--parallel-kind', choices=['process', 'thread'], default='process')
    return parser.parse_args()


def main():
    args = parse_args()
    if args.mode == 'light':
        positions = args.positions or 100
        depth = args.depth or 6
        workers = args.workers or 1
    elif args.mode == 'medium':
        positions = args.positions or 2000
        depth = args.depth or 20
        workers = args.workers or max(1, multiprocessing.cpu_count() // 2)
    else:  # heavy
        positions = args.positions or 100000
        depth = args.depth or 40
        workers = args.workers or multiprocessing.cpu_count()

    run(args.mode, workers, positions, depth, args.seed, args.parallel_kind)


if __name__ == '__main__':
    main()
