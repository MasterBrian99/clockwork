

use chess_core::{moves::Move, position::Position, search};
use std::io::{self, BufRead, Write};


pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;


pub struct UciEngine {
    position: Position,
    search_params: search::SearchParams,
}

impl UciEngine {
    
    pub fn new() -> Self {
        Self {
            position: Position::new(),
            search_params: search::SearchParams::default(),
        }
    }

    
    pub fn run(&mut self) -> Result<()> {
        let stdin = io::stdin();
        let mut stdout = io::stdout();

        for line in stdin.lock().lines() {
            let line = line?;
            let response = self.handle_command(&line)?;

            if let Some(response) = response {
                writeln!(stdout, "{}", response)?;
                stdout.flush()?;
            }

            if line == "quit" {
                break;
            }
        }

        Ok(())
    }

    
    pub fn handle_command(&mut self, command: &str) -> Result<Option<String>> {
        let parts: Vec<&str> = command.split_whitespace().collect();
        if parts.is_empty() {
            return Ok(None);
        }

        match parts[0] {
            "uci" => self.handle_uci(),
            "isready" => self.handle_isready(),
            "ucinewgame" => self.handle_ucinewgame(),
            "position" => self.handle_position(&parts[1..]),
            "go" => self.handle_go(&parts[1..]),
            "stop" => self.handle_stop(),
            "quit" => Ok(None), 
            "debug" => self.handle_debug(&parts[1..]),
            "setoption" => self.handle_setoption(&parts[1..]),
            "register" => self.handle_register(),
            _ => Ok(Some("Unknown command".to_string())),
        }
    }

    
    fn handle_uci(&self) -> Result<Option<String>> {
        let mut response = String::new();
        response.push_str("id name Castono Chess Engine\n");
        response.push_str("id author Claude Code\n");
        response.push_str("uciok");
        Ok(Some(response))
    }

    
    fn handle_isready(&self) -> Result<Option<String>> {
        Ok(Some("readyok".to_string()))
    }

    
    fn handle_ucinewgame(&mut self) -> Result<Option<String>> {
        self.position = Position::new();
        Ok(None)
    }

    
    fn handle_position(&mut self, args: &[&str]) -> Result<Option<String>> {
        if args.is_empty() {
            return Err("Invalid position command".into());
        }

        match args[0] {
            "startpos" => {
                self.position = Position::new();
                if args.len() > 1 && args[1] == "moves" {
                    self.apply_moves(&args[2..])?;
                }
            }
            "fen" => {
                if args.len() < 2 {
                    return Err("Missing FEN string".into());
                }
                let fen_parts: Vec<&str> = args[1..].iter().take(6).copied().collect();
                let fen = fen_parts.join(" ");
                self.position = Position::from_fen(&fen)
                    .map_err(|e| format!("Invalid FEN: {}", e))?;

                
                let moves_start = 1 + fen_parts.len();
                if args.len() > moves_start && args[moves_start] == "moves" {
                    self.apply_moves(&args[moves_start + 1..])?;
                }
            }
            _ => return Err("Invalid position type".into()),
        }

        Ok(None)
    }

    
    fn handle_go(&mut self, args: &[&str]) -> Result<Option<String>> {
        let mut params = search::SearchParams::default();

        let mut i = 0;
        while i < args.len() {
            match args[i] {
                "depth" => {
                    if i + 1 < args.len() {
                        params.depth = args[i + 1].parse().unwrap_or(4);
                        i += 1;
                    }
                }
                "movetime" => {
                    if i + 1 < args.len() {
                        params.time_limit_ms = Some(args[i + 1].parse().unwrap_or(1000));
                        i += 1;
                    }
                }
                "nodes" => {
                    if i + 1 < args.len() {
                        params.nodes_limit = Some(args[i + 1].parse().unwrap_or(1000000));
                        i += 1;
                    }
                }
                "infinite" => {
                    params.time_limit_ms = None;
                    params.nodes_limit = None;
                }
                _ => {}
            }
            i += 1;
        }

        self.search_params = params;

        
        let result = search::search(&self.position, &self.search_params)?;

        if let Some(best_move) = result.best_move {
            let response = format!(
                "bestmove {}\ninfo depth {} score cp {} nodes {}",
                best_move.to_algebraic(),
                result.depth,
                result.score,
                result.stats.nodes_searched
            );
            Ok(Some(response))
        } else {
            Ok(Some("bestmove 0000".to_string())) 
        }
    }

    
    fn handle_stop(&self) -> Result<Option<String>> {
        
        Ok(None)
    }

    
    fn handle_debug(&self, args: &[&str]) -> Result<Option<String>> {
        if !args.is_empty() && args[0] == "on" {
            
            Ok(Some("Debug mode enabled".to_string()))
        } else {
            
            Ok(Some("Debug mode disabled".to_string()))
        }
    }

    
    fn handle_setoption(&self, _args: &[&str]) -> Result<Option<String>> {
        
        Ok(None)
    }

    
    fn handle_register(&self) -> Result<Option<String>> {
        Ok(None) 
    }

    
    fn apply_moves(&mut self, moves: &[&str]) -> Result<()> {
        for move_str in moves {
            let mv = self.parse_move(move_str)?;
            self.position.make_move(&mv)?;
        }
        Ok(())
    }

    
    fn parse_move(&self, move_str: &str) -> Result<Move> {
        
        if move_str.len() < 4 {
            return Err(format!("Invalid move format: {}", move_str).into());
        }

        let from_str = &move_str[0..2];
        let to_str = &move_str[2..4];

        let from_square = chess_core::board::Square::from_algebraic(from_str)
            .ok_or_else(|| format!("Invalid from square: {}", from_str))?;
        let to_square = chess_core::board::Square::from_algebraic(to_str)
            .ok_or_else(|| format!("Invalid to square: {}", to_str))?;

        
        let piece = self.position.board.piece_at(from_square)
            .ok_or_else(|| format!("No piece at {}", from_str))?;

        
        if move_str.len() == 5 {
            let promotion_char = move_str.chars().nth(4).unwrap();
            let promotion = match promotion_char {
                'n' | 'N' => chess_core::board::PieceType::Knight,
                'b' | 'B' => chess_core::board::PieceType::Bishop,
                'r' | 'R' => chess_core::board::PieceType::Rook,
                'q' | 'Q' => chess_core::board::PieceType::Queen,
                _ => return Err(format!("Invalid promotion: {}", promotion_char).into()),
            };
            Ok(Move::new_promotion(from_square, to_square, piece.piece_type, promotion))
        } else {
            Ok(Move::new(from_square, to_square, piece.piece_type))
        }
    }
}

impl Default for UciEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uci_commands() {
        let mut engine = UciEngine::new();

        
        let response = engine.handle_command("uci").unwrap();
        assert!(response.unwrap().contains("Castono Chess Engine"));

        
        let response = engine.handle_command("isready").unwrap();
        assert_eq!(response, Some("readyok".to_string()));

        
        let response = engine.handle_command("ucinewgame").unwrap();
        assert_eq!(response, None);
    }

    #[test]
    fn test_position_commands() {
        let mut engine = UciEngine::new();

        
        engine.handle_command("position startpos").unwrap();

        
        engine.handle_command("position startpos moves e2e4 e7e5").unwrap();

        
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        engine.handle_command(&format!("position fen {}", fen)).unwrap();
    }
}