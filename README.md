# Clockwork Chess Engine

A Rust-based chess engine implementing the Universal Chess Interface (UCI) protocol with advanced search algorithms and bitboard-based performance optimizations.

## Overview

Clockwork is a modular chess engine written in Rust that features:

- **Bitboard-based board representation** for efficient move generation and evaluation
- **Alpha-beta pruning** with quiescence search for effective tree pruning
- **UCI protocol implementation** for compatibility with popular chess GUIs
- **Magic bitboard move generation** for fast sliding piece attack calculations
- **Iterative deepening search** with configurable depth and time limits
- **Position evaluation** using material and piece-square tables

## Architecture

The project is organized into three main crates:

### `chess-core` 
Core chess logic and algorithms:
- Board representation using bitboards
- Move generation for all piece types
- Position evaluation with piece-square tables
- Search algorithms (alpha-beta, iterative deepening)
- FEN notation parsing and generation
- Game state management (check, checkmate, stalemate)

### `uci`
Universal Chess Interface implementation:
- UCI command parsing and response generation
- Engine configuration and parameter management
- Integration with chess GUIs and analysis tools

### `clockwork`
Main executable that ties everything together:
- Initializes magic bitboards
- Creates and runs the UCI engine
- Entry point for the chess engine

## Features

### Board Representation
- 64-bit bitboards for efficient piece placement tracking
- Separate bitboards for each piece type and color
- Fast attack generation using magic bitboards
- Optimized move generation algorithms

### Search Algorithm
- Alpha-beta pruning with move ordering
- Quiescence search to avoid horizon effect
- MVV-LVA (Most Valuable Victim - Least Valuable Attacker) capture ordering
- Configurable search depth and time limits
- Iterative deepening for better time management

### Position Evaluation
- Material evaluation using standard piece values
- Piece-square tables for positional evaluation
- Separate tables for each piece type
- Endgame detection for insufficient material

### UCI Protocol Support
- Standard UCI commands: `uci`, `isready`, `ucinewgame`, `position`, `go`, `stop`, `quit`
- Position setup from FEN notation or starting position
- Move sequence application
- Search parameter configuration (depth, time, nodes)

## Getting Started

### Prerequisites

- Rust 1.70+ (recommended to use [rustup](https://rustup.rs/))
- A chess GUI that supports UCI (optional, for playing against the engine)

### Building

Clone the repository and build the project:

```bash
git clone https://github.com/MasterBrian99/clockwork.git
cd clockwork
cargo build --release
```

### Running the Engine

After building, you can run the engine directly:

```bash
cargo run --release
```

Or run the release binary:

```bash
./target/release/clockwork
```

### Testing

Run the test suite:

```bash
cargo test
```

### Using with Chess GUIs

1. Configure your chess GUI (Arena, Fritz, SCID, etc.) to use UCI engines
2. Point it to the `clockwork` executable
3. Start a new game or analysis session

## UCI Commands

The engine supports the following UCI commands:

- `uci` - Engine identification
- `isready` - Check if engine is ready
- `ucinewgame` - Start a new game
- `position [startpos|fen] [moves ...]` - Set board position
- `go [parameters]` - Start searching
  - `depth <n>` - Search to specific depth
  - `movetime <ms>` - Search for specific time
  - `nodes <n>` - Search specific number of nodes
  - `infinite` - Search indefinitely
- `stop` - Stop current search
- `quit` - Exit engine

### Example Session

```
uci
id name Castono Chess Engine
id author Claude Code
uciok

isready
readyok

position startpos moves e2e4 e7e5
go depth 4
bestmove c7c5 info depth 4 score cp 20 nodes 12345

quit
```

## Code Structure

### Core Components

#### Board (`chess-core/src/board.rs`)
- `Board` struct with bitboard representation
- `Piece`, `PieceType`, `Color`, and `Square` enums/structs
- Methods for piece placement and retrieval

#### Position (`chess-core/src/position.rs`)
- `Position` struct maintaining full game state
- FEN notation parsing and generation
- Move execution and undo functionality
- Game state detection (check, checkmate, stalemate)

#### Move Generation (`chess-core/src/movegen.rs`)
- Efficient move generation for all piece types
- Magic bitboard attack calculations
- Legal move validation

#### Search (`chess-core/src/search.rs`)
- Alpha-beta pruning implementation
- Quiescence search
- Iterative deepening
- Move ordering heuristics

#### Evaluation (`chess-core/src/evaluate.rs`)
- Material and positional evaluation
- Piece-square tables
- Endgame detection

#### UCI Interface (`uci/src/lib.rs`)
- UCI protocol implementation
- Command parsing and response generation
- Engine state management

## Performance Considerations

### Bitboard Optimizations
- 64-bit operations for efficient piece tracking
- Magic bitboards for fast sliding piece attacks
- Precomputed attack tables for knights and kings

### Search Optimizations
- Alpha-beta pruning reduces search tree size
- Move ordering improves pruning effectiveness
- Quiescence search prevents tactical blunders
- Iterative deepening provides better time management

### Memory Efficiency
- Compact move representation (32-bit)
- Efficient board state storage
- Minimal allocation during search

## Development

### Adding Features

The modular structure makes it easy to extend the engine:

1. **New evaluation terms**: Add to `chess-core/src/evaluate.rs`
2. **Search improvements**: Modify `chess-core/src/search.rs`
3. **UCI extensions**: Update `uci/src/lib.rs`
4. **Performance optimizations**: Focus on `chess-core/src/movegen.rs`

### Testing Strategy

The project includes comprehensive unit tests for:
- Move generation correctness
- Position manipulation
- Search algorithm behavior
- UCI command handling

Run tests with `cargo test` and add new tests when implementing features.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

## Acknowledgments

- The Rust chess engine community for inspiration and best practices
- UCI protocol specification for standardization
- Magic bitboard algorithms for efficient move generation