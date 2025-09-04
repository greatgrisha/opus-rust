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
_see the, sample.py and the batch_generation.py also, pref-full.py, this would contain every function there is, and it would be just, simple yet efficient representation of the api._
## Performance Benchmark

[_Perfomance benchmark, just use, the algorithim, that play the random move at the increasing or specific depth, to test the, perfomance, of this program._]
Run the provided benchmark script:
```python
python pref_full.py
```
Example RUN:
```
__Light mode__, -> prefered for, just, smoke test
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
__Medium Test_ -> , Medium mode would be suitable for training,  ex. training the neural network on the game would require
atleast millions of games, can be perform on the standard hardware
f) (maturin_env) /content/opus-rust/move-generation# python pref-full.py --mode medium --worker 48 --positions 2000 --depth 20
Perf test starting: mode=medium, workers=48, positions=2000, depth=20, seed=42, parallel=process
Worker 22 done: positions=2000 moves=65302 avg_pos_time=0.000045s
Worker 24 done: positions=2000 moves=65149 avg_pos_time=0.000045s
Worker 20 done: positions=2000 moves=65761 avg_pos_time=0.000046s
Worker 14 done: positions=2000 moves=65250 avg_pos_time=0.000046s
Worker 16 done: positions=2000 moves=65199 avg_pos_time=0.000048s
Worker 34 done: positions=2000 moves=65139 avg_pos_time=0.000051s
Worker 40 done: positions=2000 moves=64848 avg_pos_time=0.000053s
Worker 10 done: positions=2000 moves=65595 avg_pos_time=0.000055s
Worker 25 done: positions=2000 moves=65256 avg_pos_time=0.000060s
Worker 26 done: positions=2000 moves=65248 avg_pos_time=0.000060s
Worker 5 done: positions=2000 moves=65100 avg_pos_time=0.000061s
Worker 6 done: positions=2000 moves=65026 avg_pos_time=0.000060s
Worker 12 done: positions=2000 moves=65152 avg_pos_time=0.000059s
Worker 9 done: positions=2000 moves=65558 avg_pos_time=0.000063s
Worker 18 done: positions=2000 moves=65212 avg_pos_time=0.000063s
Worker 43 done: positions=2000 moves=64999 avg_pos_time=0.000065s
Worker 39 done: positions=2000 moves=65094 avg_pos_time=0.000064s
Worker 42 done: positions=2000 moves=65235 avg_pos_time=0.000065s
Worker 17 done: positions=2000 moves=65475 avg_pos_time=0.000076s
Worker 36 done: positions=2000 moves=65599 avg_pos_time=0.000066s
Worker 35 done: positions=2000 moves=65174 avg_pos_time=0.000068s
Worker 44 done: positions=2000 moves=64883 avg_pos_time=0.000069s
Worker 19 done: positions=2000 moves=65111 avg_pos_time=0.000070s
Worker 31 done: positions=2000 moves=65383 avg_pos_time=0.000072s
Worker 2 done: positions=2000 moves=64866 avg_pos_time=0.000074s
Worker 21 done: positions=2000 moves=65058 avg_pos_time=0.000073s
Worker 45 done: positions=2000 moves=65090 avg_pos_time=0.000076s
Worker 0 done: positions=2000 moves=65315 avg_pos_time=0.000076s
Worker 46 done: positions=2000 moves=64901 avg_pos_time=0.000077s
Worker 27 done: positions=2000 moves=64989 avg_pos_time=0.000078s
Worker 30 done: positions=2000 moves=65051 avg_pos_time=0.000078s
Worker 33 done: positions=2000 moves=64929 avg_pos_time=0.000078s
Worker 7 done: positions=2000 moves=65170 avg_pos_time=0.000079s
Worker 32 done: positions=2000 moves=65253 avg_pos_time=0.000079s
Worker 4 done: positions=2000 moves=65113 avg_pos_time=0.000079s
Worker 15 done: positions=2000 moves=65398 avg_pos_time=0.000080s
Worker 13 done: positions=2000 moves=65417 avg_pos_time=0.000079s
Worker 28 done: positions=2000 moves=64716 avg_pos_time=0.000079s
Worker 41 done: positions=2000 moves=65466 avg_pos_time=0.000081s
Worker 11 done: positions=2000 moves=65032 avg_pos_time=0.000080s
Worker 3 done: positions=2000 moves=65792 avg_pos_time=0.000080s
Worker 1 done: positions=2000 moves=64790 avg_pos_time=0.000080s
Worker 23 done: positions=2000 moves=65344 avg_pos_time=0.000080s
Worker 29 done: positions=2000 moves=65330 avg_pos_time=0.000081s
Worker 47 done: positions=2000 moves=65641 avg_pos_time=0.000081s
Worker 38 done: positions=2000 moves=65169 avg_pos_time=0.000081s
Worker 37 done: positions=2000 moves=64980 avg_pos_time=0.000083s
Worker 8 done: positions=2000 moves=64559 avg_pos_time=0.000082s

--- Performance Summary ---
Mode: medium
Workers: 48
Total positions: 96000
Total moves generated: 3.13M
Avg moves per position: 32.59
Total wall time: 7.70s
Measured total time (workers span): 7.56s
Mean time per position: 0.069229 ms
Median time per position: 0.061035 ms
P95 time per position: 0.108242 ms
Moves/sec (measured): 470832
Memory delta (main process): 5.94 MB
---------------------------

(maturin_env) /content/opus-rust/move-generation# 

__Heavy Test_ -> This would be, full, training grade, test on perfomance, would be perform. expect that if the medium test used 2000 position then expect the 10000 position. indeed, 100x increase in all the generation.

(maturin_env) /content/opus-rust/move-generation# python pref-full.py --mode heavy
Perf test starting: mode=heavy, workers=48, positions=100000, depth=40, seed=42, parallel=process



Worker 13 done: positions=100000 moves=3562655 avg_pos_time=0.000057s
Worker 39 done: positions=100000 moves=3561882 avg_pos_time=0.000058s
Worker 19 done: positions=100000 moves=3556644 avg_pos_time=0.000057s
Worker 8 done: positions=100000 moves=3561427 avg_pos_time=0.000057s
Worker 20 done: positions=100000 moves=3557918 avg_pos_time=0.000057s
Worker 42 done: positions=100000 moves=3558467 avg_pos_time=0.000057s
Worker 3 done: positions=100000 moves=3555355 avg_pos_time=0.000057s
Worker 25 done: positions=100000 moves=3560976 avg_pos_time=0.000057s
Worker 2 done: positions=100000 moves=3558883 avg_pos_time=0.000058s
Worker 0 done: positions=100000 moves=3559577 avg_pos_time=0.000057s
Worker 43 done: positions=100000 moves=3559259 avg_pos_time=0.000058s
Worker 29 done: positions=100000 moves=3560019 avg_pos_time=0.000057s
Worker 4 done: positions=100000 moves=3560494 avg_pos_time=0.000058s
Worker 32 done: positions=100000 moves=3559071 avg_pos_time=0.000057s
Worker 5 done: positions=100000 moves=3557824 avg_pos_time=0.000058s
Worker 9 done: positions=100000 moves=3558135 avg_pos_time=0.000058s
Worker 33 done: positions=100000 moves=3561889 avg_pos_time=0.000058s
Worker 1 done: positions=100000 moves=3560623 avg_pos_time=0.000058s
Worker 14 done: positions=100000 moves=3563053 avg_pos_time=0.000057s
Worker 37 done: positions=100000 moves=3556952 avg_pos_time=0.000057s
Worker 34 done: positions=100000 moves=3559905 avg_pos_time=0.000058s
Worker 40 done: positions=100000 moves=3561493 avg_pos_time=0.000057s
Worker 15 done: positions=100000 moves=3560082 avg_pos_time=0.000058s
Worker 11 done: positions=100000 moves=3560688 avg_pos_time=0.000058s
Worker 21 done: positions=100000 moves=3560013 avg_pos_time=0.000061s
Worker 26 done: positions=100000 moves=3560983 avg_pos_time=0.000061s
Worker 27 done: positions=100000 moves=3557881 avg_pos_time=0.000062s
Worker 22 done: positions=100000 moves=3560174 avg_pos_time=0.000062s
Worker 44 done: positions=100000 moves=3558028 avg_pos_time=0.000062s
Worker 35 done: positions=100000 moves=3560044 avg_pos_time=0.000062s
Worker 28 done: positions=100000 moves=3558992 avg_pos_time=0.000062s
Worker 45 done: positions=100000 moves=3560652 avg_pos_time=0.000061s
Worker 30 done: positions=100000 moves=3562268 avg_pos_time=0.000062s
Worker 10 done: positions=100000 moves=3557509 avg_pos_time=0.000062s
Worker 12 done: positions=100000 moves=3559452 avg_pos_time=0.000062s
Worker 6 done: positions=100000 moves=3563697 avg_pos_time=0.000062s
Worker 31 done: positions=100000 moves=3560770 avg_pos_time=0.000062s
Worker 16 done: positions=100000 moves=3558783 avg_pos_time=0.000061s
Worker 7 done: positions=100000 moves=3561783 avg_pos_time=0.000062s
Worker 36 done: positions=100000 moves=3561608 avg_pos_time=0.000062s
Worker 38 done: positions=100000 moves=3561446 avg_pos_time=0.000061s
Worker 24 done: positions=100000 moves=3559228 avg_pos_time=0.000062s
Worker 47 done: positions=100000 moves=3558363 avg_pos_time=0.000062s
Worker 23 done: positions=100000 moves=3555187 avg_pos_time=0.000062s
Worker 18 done: positions=100000 moves=3560851 avg_pos_time=0.000062s
Worker 46 done: positions=100000 moves=3556162 avg_pos_time=0.000062s
Worker 17 done: positions=100000 moves=3561021 avg_pos_time=0.000063s
Worker 41 done: positions=100000 moves=3561557 avg_pos_time=0.000070s

--- Performance Summary ---
Mode: heavy
Workers: 48
Total positions: 4800000
Total moves generated: 170.87M
Avg moves per position: 35.60
Total wall time: 515.09s
Measured total time (workers span): 510.92s
Mean time per position: 0.059756 ms
Median time per position: 0.057697 ms
P95 time per position: 0.077724 ms
Moves/sec (measured): 595720
Memory delta (main process): 208.19 MB
---------------------------



```

## Comparison to Other Libraries
- **python-chess:** This Rust engine is significantly faster for bulk move generation, especially when using parallelism. python-chess is pure Python and not parallelized.
- **Stockfish:** Stockfish is a full chess engine; this library is focused on move generation only, with a much simpler API and easier integration for custom program.
- **Other Rust libraries:** This project is optimized for Python FFI and parallel workloads, making it ideal for AI/ science pipelines.

## API Reference
- `PyBoard()` — Create a new board
- `generate_moves()` — Get all legal moves for the current board
- `generate_moves_for_pieces_parallel(piece_sq_list)` — Parallel move generation for a list of (piece, square) tuples

## Development
- Rust code in `src/`
- Python bindings via PyO3
- Benchmarks in `pref_full.py`

## License
MIT
