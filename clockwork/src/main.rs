//! Castono Chess Engine - Main executable

use chess_core::magic_simple as magic;
use uci::UciEngine;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize magic bitboards
    magic::init();

    // Create and run UCI engine
    let mut engine = UciEngine::new();
    if let Err(e) = engine.run() {
        eprintln!("Error running engine: {}", e);
        std::process::exit(1);
    }

    Ok(())
}