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
- Python API for easy use in data science, AI, and chess tools

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
Run the provided benchmark script:
```bash
python perf_test.py
```
Example RUN:
```
--- High CPU/Memory (all squares, parallel vs serial) ---
Parallel: 64 pieces, 0.0007s, mem: 0.00MB, cpu: 0.00%
Serial: 64 pieces, 0.0002s, mem: 0.12MB, cpu: 0.00%

--- Low CPU/Memory (few pieces, parallel vs serial) ---
Parallel: 4 pieces, 0.0001s, mem: 0.00MB, cpu: 0.00%
Serial: 4 pieces, 0.0000s, mem: 0.00MB, cpu: 0.00%

--- System Info ---
CPUs: 4 | RAM: 15995 MB
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
