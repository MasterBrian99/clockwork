//! Search algorithms for chess engine

use crate::{
    evaluate,
    moves::Move,
    position::Position,
    Error, Result,
};

/// Search statistics
#[derive(Debug, Default, Clone)]
pub struct SearchStats {
    pub nodes_searched: u64,
    pub qnodes_searched: u64,
    pub cutoffs: u64,
    pub depth: u32,
}

/// Search result
#[derive(Debug, Clone)]
pub struct SearchResult {
    pub best_move: Option<Move>,
    pub score: i32,
    pub depth: u32,
    pub stats: SearchStats,
}

/// Search parameters
#[derive(Debug, Clone)]
pub struct SearchParams {
    pub depth: u32,
    pub time_limit_ms: Option<u64>,
    pub nodes_limit: Option<u64>,
}

impl Default for SearchParams {
    fn default() -> Self {
        Self {
            depth: 4,
            time_limit_ms: None,
            nodes_limit: None,
        }
    }
}

/// Search for the best move in a position
pub fn search(position: &Position, params: &SearchParams) -> Result<SearchResult> {
    let mut stats = SearchStats::default();
    stats.depth = params.depth;

    // Check for immediate game over
    if position.is_game_over() {
        return Ok(SearchResult {
            best_move: None,
            score: evaluate_game_over(position),
            depth: 0,
            stats,
        });
    }

    let mut best_move = None;
    let mut best_score = i32::MIN + 1;

    // Generate all moves
    let moves = position.generate_moves();

    for mv in moves {
        let mut new_pos = position.clone();
        new_pos.make_move(&mv)?;

        let score = -alpha_beta(
            &new_pos,
            params.depth - 1,
            i32::MIN + 1,
            i32::MAX - 1,
            &mut stats,
        );

        if score > best_score {
            best_score = score;
            best_move = Some(mv);
        }
    }

    Ok(SearchResult {
        best_move,
        score: best_score,
        depth: params.depth,
        stats,
    })
}

/// Alpha-beta search algorithm
fn alpha_beta(
    position: &Position,
    depth: u32,
    mut alpha: i32,
    beta: i32,
    stats: &mut SearchStats,
) -> i32 {
    stats.nodes_searched += 1;

    // Check for terminal node
    if depth == 0 {
        return quiescence_search(position, alpha, beta, stats);
    }

    if position.is_game_over() {
        return evaluate_game_over(position);
    }

    let moves = position.generate_moves();

    // Sort moves (basic implementation - could be improved with move ordering)
    let mut scored_moves: Vec<(Move, i32)> = moves
        .into_iter()
        .map(|mv| {
            let score = move_score(position, &mv);
            (mv, score)
        })
        .collect();

    // Sort by score (highest first for maximizing player)
    scored_moves.sort_by(|a, b| b.1.cmp(&a.1));

    for (mv, _) in scored_moves {
        let mut new_pos = position.clone();
        if new_pos.make_move(&mv).is_err() {
            continue; // Skip illegal moves
        }

        let score = -alpha_beta(&new_pos, depth - 1, -beta, -alpha, stats);

        if score >= beta {
            stats.cutoffs += 1;
            return beta; // Beta cutoff
        }

        if score > alpha {
            alpha = score;
        }
    }

    alpha
}

/// Quiescence search to avoid horizon effect
fn quiescence_search(
    position: &Position,
    mut alpha: i32,
    beta: i32,
    stats: &mut SearchStats,
) -> i32 {
    stats.qnodes_searched += 1;

    let stand_pat = evaluate::evaluate(position);

    if stand_pat >= beta {
        return beta;
    }

    if stand_pat > alpha {
        alpha = stand_pat;
    }

    // Only consider capture moves in quiescence search
    let capture_moves: Vec<Move> = position
        .generate_moves()
        .into_iter()
        .filter(|mv| mv.is_capture(&position.board))
        .collect();

    // Sort captures by MVV-LVA (Most Valuable Victim - Least Valuable Attacker)
    let mut scored_captures: Vec<(Move, i32)> = capture_moves
        .into_iter()
        .map(|mv| {
            let score = capture_score(position, &mv);
            (mv, score)
        })
        .collect();

    scored_captures.sort_by(|a, b| b.1.cmp(&a.1));

    for (mv, _) in scored_captures {
        let mut new_pos = position.clone();
        if new_pos.make_move(&mv).is_err() {
            continue;
        }

        let score = -quiescence_search(&new_pos, -beta, -alpha, stats);

        if score >= beta {
            return beta;
        }

        if score > alpha {
            alpha = score;
        }
    }

    alpha
}

/// Evaluate game over positions
fn evaluate_game_over(position: &Position) -> i32 {
    if position.is_checkmate() {
        // Very negative score for checkmate (but not the absolute minimum)
        -20000 + (position.fullmove_number as i32) // Prefer later checkmates
    } else if position.is_stalemate() {
        // Draw
        0
    } else {
        // Other draws (50-move rule, repetition, insufficient material)
        0
    }
}

/// Score a move for move ordering
fn move_score(position: &Position, mv: &Move) -> i32 {
    let mut score = 0;

    // Captures get high priority
    if mv.is_capture(&position.board) {
        score += capture_score(position, mv);
    }

    // Promotions get high priority
    if mv.is_promotion() {
        if let Some(promotion) = mv.promotion_piece() {
            score += match promotion {
                crate::board::PieceType::Queen => 900,
                crate::board::PieceType::Rook => 500,
                crate::board::PieceType::Bishop => 300,
                crate::board::PieceType::Knight => 300,
                _ => 0,
            };
        }
    }

    // Killer moves and history heuristic could be added here

    score
}

/// Score a capture move using MVV-LVA
fn capture_score(position: &Position, mv: &Move) -> i32 {
    let from_piece = position.board.piece_at(mv.from());
    let to_piece = position.board.piece_at(mv.to());

    if let (Some(attacker), Some(victim)) = (from_piece, to_piece) {
        // MVV-LVA: Most Valuable Victim - Least Valuable Attacker
        let victim_value = piece_value(victim.piece_type);
        let attacker_value = piece_value(attacker.piece_type);

        // Higher score for capturing valuable pieces with less valuable pieces
        victim_value * 10 - attacker_value
    } else if mv.is_en_passant() {
        // En passant captures a pawn
        100 // Pawn value
    } else {
        0
    }
}

/// Get the value of a piece type
fn piece_value(piece_type: crate::board::PieceType) -> i32 {
    match piece_type {
        crate::board::PieceType::Pawn => 100,
        crate::board::PieceType::Knight => 300,
        crate::board::PieceType::Bishop => 300,
        crate::board::PieceType::Rook => 500,
        crate::board::PieceType::Queen => 900,
        crate::board::PieceType::King => 20000,
    }
}

/// Iterative deepening search
pub fn iterative_deepening(
    position: &Position,
    max_depth: u32,
    time_limit_ms: Option<u64>,
) -> Result<SearchResult> {
    let mut best_result = None;

    for depth in 1..=max_depth {
        let params = SearchParams {
            depth,
            time_limit_ms,
            nodes_limit: None,
        };

        let result = search(position, &params)?;

        // Update best result
        best_result = Some(result.clone());

        // Check time limit (basic implementation)
        // In a real engine, you'd check elapsed time here

        // If we found a checkmate, we can stop early
        if result.score.abs() > 10000 {
            break;
        }
    }

    best_result.ok_or_else(|| Error::InvalidMove("No moves found".to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_starting_position() {
        let pos = Position::new();
        let params = SearchParams {
            depth: 3,
            time_limit_ms: None,
            nodes_limit: None,
        };

        let result = search(&pos, &params).unwrap();
        assert!(result.best_move.is_some());
        assert!(result.score.abs() < 1000); // Should be a reasonable score
        assert!(result.stats.nodes_searched > 0);
    }

    #[test]
    fn test_checkmate_search() {
        // Fool's mate position - black to move and deliver checkmate
        let fen = "rnb1kbnr/pppp1ppp/8/4p3/6Pq/5P2/PPPPP2P/RNBQKBNR w KQkq - 0 1";
        let pos = Position::from_fen(fen).unwrap();

        let params = SearchParams {
            depth: 1,
            time_limit_ms: None,
            nodes_limit: None,
        };

        let result = search(&pos, &params).unwrap();
        assert!(result.score < -10000); // Very negative score for checkmate
    }

    #[test]
    fn test_iterative_deepening() {
        let pos = Position::new();
        let result = iterative_deepening(&pos, 3, None).unwrap();

        assert!(result.best_move.is_some());
        assert_eq!(result.depth, 3);
        assert!(result.stats.nodes_searched > 0);
    }
}