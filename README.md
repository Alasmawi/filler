# Filler

A Rust bot for **Filler**, a territory-capture algorithmic game from the 01-edu / Reboot01 piscine curriculum. Two robots take turns dropping pieces onto a shared grid; each drop must touch exactly one of your own cells and none of the opponent's. The bot that ends up with more territory wins. This repo is the player program (not the game engine), built to read the engine's turn-by-turn state over stdin and answer with a placement over stdout.

## About the exercise

The Filler piscine project is about writing a bot that plays correctly and quickly against the provided game engine and reference "robots" (e.g. `bender`), across multiple maps and random seeds, as both player 1 and player 2. The core challenge is parsing an ad hoc text protocol reliably and choosing good placements under time pressure — this implementation adds a simple heuristic strategy on top of a correct, well-tested parser.

## How it works

1. Reads the player identity line, then repeatedly reads an `Anfield` (the board) and a `Piece` block from stdin.
2. Validates candidate placements: the piece's active cells must overlap exactly one of the bot's own cells, must not touch the opponent's cells, and must stay inside the board.
3. Scores valid placements using a BFS distance heatmap from every enemy cell, favoring moves that advance toward the opponent, with tie-breaking by center distance, neighboring empty/enemy cells, and board occupancy phase.
4. Writes the chosen `X Y` coordinate (top-left corner of the piece) to stdout and flushes, or `0 0` if no valid move exists. Keeps looping until stdin closes.

## Tech stack

- **Rust** (edition 2021, `rust-version = 1.63` pinned to match the provided Docker image), no external crates — standard library only
- **Cargo** for building, testing, and running
- **GitHub Actions** (`.github/workflows/ci.yml`) running fmt check, Clippy, and tests against Rust 1.63 and stable

## Project structure

```
Cargo.toml            # crate manifest (lib + bin, no dependencies)
src/
  main.rs              # stdin/stdout game loop
  parser.rs            # parses player id, Anfield, and Piece blocks from engine input
  board.rs             # board model and player symbol ownership (a/@ vs s/$)
  piece.rs             # piece model with sparse active-cell coordinates
  placement.rs         # placement validation and enumeration of valid moves
  heatmap.rs           # BFS distance map from enemy cells
  strategy.rs          # move scoring and deterministic best-move selection
  output.rs            # exact "X Y\n" coordinate formatting expected by the engine
tests/
  integration_tests.rs # parser, placement, strategy, and output tests
Makefile               # build / test / fmt / check / ci targets
```

## Build & run

```bash
cargo build --release
```

Run the full check suite (fmt check, Clippy with warnings as errors, tests):

```bash
make check
```

Playing a match requires the official Filler Docker image and game engine (not included in this repo). From the provided `docker_image` folder:

```bash
docker build -t filler .
docker run -v "$(pwd)/solution":/filler/solution -it filler
```

With this repo mounted as `solution` inside the container:

```bash
cd /filler/solution
cargo build --release
./game_engine -f maps/map01 -p1 solution/target/release/filler -p2 robots/bender
```
