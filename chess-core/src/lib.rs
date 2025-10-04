pub  mod  bitboard;
pub  mod board;
pub  mod magic;
pub mod magic_simple;
pub  mod moves;
pub  mod position;
pub mod movegen;
pub  mod evaluate;
pub  mod search;

/// Result type for chess operations
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Invalid move: {0}")]
    InvalidMove(String),
    #[error("Invalid position: {0}")]
    InvalidPosition(String),
    #[error("Invalid FEN: {0}")]
    InvalidFen(String),
}