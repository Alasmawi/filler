# Filler

Production-oriented Rust robot for the Filler algorithmic game.

The bot reads the game engine stream from `stdin`, parses each Anfield and Piece,
chooses a valid placement, and writes exactly one coordinate pair per turn:

```txt
X Y
```

## Rules Implemented

- Player 1 owns `a` and `@`; Player 2 owns `s` and `$`.
- A placement is valid when active piece cells overlap exactly one own cell.
- Active piece cells may not overlap enemy cells.
- The piece bounding box must remain inside the board.
- If no valid move exists, the bot outputs `0 0`.

## Strategy

The strategy builds a BFS heatmap from all enemy cells and scores every valid
placement. Lower scores are preferred, which pushes the bot toward the enemy.
Small phase-based heuristics adjust scoring for center control, blocking, and
late-game mobility.

## Build

```bash
cargo build --release
```

## Test

```bash
cargo test
```

or:

```bash
make test
```

## Run

```bash
cargo run
```

Example engine invocation:

```bash
./game_engine -f maps/map01 -p1 target/release/filler -p2 robots/bender
```
