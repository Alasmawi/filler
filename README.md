# Filler

Rust robot for the Filler algorithmic game.

The bot reads the game engine stream from standard input, parses each Anfield
and Piece, chooses a placement, and writes exactly one coordinate pair per turn:

```txt
X Y
```

The output includes the final newline required by the engine.

## Game Rules

- Player 1 is represented by `a` and `@`.
- Player 2 is represented by `s` and `$`.
- A valid placement must overlap exactly one cell of the bot's own territory.
- Active piece cells must not overlap opponent territory.
- The full piece rectangle must stay inside the Anfield.
- If no valid move exists, the bot still returns `0 0`.
- The bot keeps reading turns until the engine closes standard input.

Piece cells from the engine are parsed with `.` as empty and `O` or `#` as
active cells, matching the examples used by the subject and engine variants.

## Protocol Example

Input:

```txt
$$$ exec p1 : [robots/bender]
Anfield 20 15:
    01234567890123456789
000 ....................
001 ....................
002 .........@..........
003 ....................
004 ....................
005 ....................
006 ....................
007 ....................
008 ....................
009 ....................
010 ....................
011 ....................
012 .........$..........
013 ....................
014 ....................
Piece 4 1:
.OO.
```

Output:

```txt
7 2
```

Coordinates are printed as `X Y`, where `X` is the column and `Y` is the row of
the piece's top-left corner.

## Project Layout

- `src/parser.rs`: player, board, and piece parsing from engine input.
- `src/board.rs`: board model and player symbol ownership.
- `src/piece.rs`: piece model with sparse active-cell coordinates.
- `src/placement.rs`: placement validation and valid-placement enumeration.
- `src/heatmap.rs`: distance map from enemy cells.
- `src/strategy.rs`: move scoring and deterministic best-move selection.
- `src/output.rs`: exact engine coordinate formatting.
- `src/main.rs`: stdin/stdout loop.
- `tests/integration_tests.rs`: parser, placement, strategy, and output tests.

## Strategy

The bot builds a BFS heatmap from all enemy cells and scores every valid
placement. Lower scores are preferred, which generally moves toward the enemy.
The score also considers center distance, neighboring empty cells, neighboring
enemy cells, and the current board occupancy phase.

Ties keep the first coordinate found during row-major scanning, so repeated
inputs produce deterministic moves.

## Build

```bash
cargo build --release
```

The project declares `rust-version = "1.63"` in `Cargo.toml` to match the
provided Docker image.

## Test And Checks

```bash
make check
```

This runs:

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets
```

For a release build as well:

```bash
make ci
```

## Docker Game Engine

The official subject requires running inside the provided Docker image.

From the provided `docker_image` folder:

```bash
docker build -t filler .
docker run -v "$(pwd)/solution":/filler/solution -it filler
```

Place this repository inside the mounted `solution` directory, then build the
robot in the container:

```bash
cd /filler/solution
cargo build --release
```

Example game-engine command from inside the container:

```bash
./game_engine -f maps/map01 -p1 solution/target/release/filler -p2 robots/bender
```

Run the bot as both players across the provided maps, robots, and seeds:

```bash
./game_engine -f maps/map01 -p1 solution/target/release/filler -p2 robots/bender -s 1 -q
./game_engine -f maps/map01 -p1 robots/bender -p2 solution/target/release/filler -s 1 -q
```

## Known Validation Status

Local formatting, Clippy, tests, and release build pass. The repository also
contains a GitHub Actions workflow that checks Rust `1.63.0` and stable.

Official Docker tournament results are not committed here. Before submission,
run the release binary in the official container against every provided map and
robot, as both player 1 and player 2, with several deterministic seeds.
