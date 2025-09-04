# Move Generation: Fast Chess Move Generator (Rust + Python)

## Overview
This project provides an extremely fast chess move generation and rules engine written in Rust, with Python bindings via [PyO3](https://github.com/PyO3/pyo3). It is designed for high efficiency, parallel move generation, and easy integration into Python projects.

- **Language:** Rust (core), Python (bindings)
- **Parallelism:** Uses [rayon](https://github.com/rayon-rs/rayon) for multi-core move generation
- **Validation:** Move generation validated against [PyChess](https://github.com/niklasf/python-chess)
- **API:** Exposes a simple Python class `PyBoard` for move generation

## Features
- Full legal move generation for all pieces
- Parallel move generation for multiple pieces
- Fast bitboard-based implementation
- Python API for easy use in, AI, and chess tools, and especially AI, training

## Installation
1. **Build the Python extension:**
   ```bash
   cd move-generation
   maturin develop --release
   ```
2. **Install Python dependencies for benchmarking:**
   ```bash
   pip install psutil
   ```

## Usage Example
```python
from move_generation import PyBoard

board = PyBoard()
# Generate all legal moves for the current board
moves = board.generate_moves()
print("All moves:", moves)

# Parallel move generation for specific pieces/squares
piece_sq_list = [("rook", 0), ("knight", 1), ("bishop", 2), ("queen", 3)]
parallel_moves = board.generate_moves_for_pieces_parallel(piece_sq_list)
print("Parallel moves for pieces:", parallel_moves)
```

## Performance Benchmark

[perfomance benchmark, just use, the algorithim, that play the random move at the increasing or specific depth, to test the, perfomance, of this program.]
Run the provided benchmark script:
```bash
python pref_full.py
```
Example RUN:
```
** light mode, -> prefered for, just, smoke test
(f) @greatgrisha ➜ /workspaces/kaali/move-generation (master) $ python perf_full.py --mode light --workers 2 --depth 6
Perf test starting: mode=light, workers=2, positions=100, depth=6, seed=42, parallel=process
Worker 1 done: positions=100 moves=2586 avg_pos_time=0.000008s
Worker 0 done: positions=100 moves=2673 avg_pos_time=0.000009s

--- Performance Summary ---
Mode: light
Workers: 2
Total positions: 200
Total moves generated: 5.26k
Avg moves per position: 26.30
Total wall time: 0.05s
Measured total time (workers span): 0.04s
Mean time per position: 0.008438 ms
Median time per position: 0.006437 ms
P95 time per position: 0.015211 ms
Moves/sec (measured): 3116395
Memory delta (main process): 0.12 MB
---------------------------

(f) @greatgrisha ➜ /workspaces/kaali/move-generation (master) $

** medium test, - medium mode would be suitable for training, ex. training the neural network on the game would require, atleast millions of games, medium mode can test this optimally on the standard hardware

(f) @greatgrisha ➜ /workspaces/opus-rust/move-generation (master) $ python perf_full.py  --mode medium --workers 8 --positions 50 --depth 6
Perf test starting: mode=medium, workers=8, positions=50, depth=6, seed=42, parallel=process
Worker 0 done: positions=50 moves=1319 avg_pos_time=0.000016s
Worker 7 done: positions=50 moves=1361 avg_pos_time=0.000016s
Worker 1 done: positions=50 moves=1256 avg_pos_time=0.000014s
Worker 3 done: positions=50 moves=1279 avg_pos_time=0.000015s
Worker 6 done: positions=50 moves=1263 avg_pos_time=0.000015s
Worker 5 done: positions=50 moves=1293 avg_pos_time=0.000016s
Worker 2 done: positions=50 moves=1328 avg_pos_time=0.000016s
Worker 4 done: positions=50 moves=1314 avg_pos_time=0.000015s

--- Performance Summary ---
Mode: medium
Workers: 8
Total positions: 400
Total moves generated: 10.41k
Avg moves per position: 26.03
Total wall time: 0.08s
Measured total time (workers span): 0.07s
Mean time per position: 0.015447 ms
Median time per position: 0.011206 ms
P95 time per position: 0.031126 ms
Moves/sec (measured): 1685328
Memory delta (main process): 0.25 MB
---------------------------

(f) @greatgrisha ➜ /workspaces/opus-rust/move-generation (master) $

**heavy test** test atleast the billion or trillion of game generation and speed require to truly form the neural network and here it can surpass other generation engine[ this isn't engine but validator and move generation for the training of model and for the integration to the specific other engine]
(f) @greatgrisha ➜ /workspaces/kaali/move-generation (master) $ echo ""machine is just same, so the perfomance would be same""
machine is just same, so the perfomance would be same
(f) @greatgrisha ➜ /workspaces/kaali/move-generation (master) $ lscpu
Architecture:             x86_64
  CPU op-mode(s):         32-bit, 64-bit
  Address sizes:          48 bits physical, 48 bits virtual
  Byte Order:             Little Endian
CPU(s):                   4
  On-line CPU(s) list:    0-3
Vendor ID:                AuthenticAMD
  Model name:             AMD EPYC 7763 64-Core Processor
    CPU family:           25
    Model:                1
    Thread(s) per core:   2
    Core(s) per socket:   2
    Socket(s):            1
    Stepping:             1
.......

```

## Comparison to Other Libraries
- **python-chess:** This Rust engine is significantly faster for bulk move generation, especially when using parallelism. python-chess is pure Python and not parallelized.
- **Stockfish:** Stockfish is a full chess engine; this library is focused on move generation only, with a much simpler API and easier integration for custom tools.
- **Other Rust libraries:** This project is optimized for Python FFI and parallel workloads, making it ideal for AI/ science pipelines.

## API Reference
- `PyBoard()` — Create a new board
- `generate_moves()` — Get all legal moves for the current board
- `generate_moves_for_pieces_parallel(piece_sq_list)` — Parallel move generation for a list of (piece, square) tuples

## Development
- Rust code in `src/`
- Python bindings via PyO3
- Benchmarks in `perf_test.py`

## License
MIT
