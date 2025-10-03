
use crate::{
    board::{Board, Color, Piece, PieceType, Square},
    moves::Move,
    movegen,
    Error, Result,
};


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CastlingRights {
    pub white_kingside: bool,
    pub white_queenside: bool,
    pub black_kingside: bool,
    pub black_queenside: bool,
}

impl CastlingRights {
    
    pub fn all() -> Self {
        Self {
            white_kingside: true,
            white_queenside: true,
            black_kingside: true,
            black_queenside: true,
        }
    }

    
    pub fn none() -> Self {
        Self {
            white_kingside: false,
            white_queenside: false,
            black_kingside: false,
            black_queenside: false,
        }
    }

    
    pub fn update(&mut self, mv: &Move, board: &Board) {
        let from = mv.from();
        let piece = board.piece_at(from);

        if let Some(piece) = piece {
            match piece.piece_type {
                PieceType::King => {
                    match piece.color {
                        Color::White => {
                            self.white_kingside = false;
                            self.white_queenside = false;
                        }
                        Color::Black => {
                            self.black_kingside = false;
                            self.black_queenside = false;
                        }
                    }
                }
                PieceType::Rook => {
                    match piece.color {
                        Color::White => {
                            if from == Square::from_algebraic("a1").unwrap() {
                                self.white_queenside = false;
                            } else if from == Square::from_algebraic("h1").unwrap() {
                                self.white_kingside = false;
                            }
                        }
                        Color::Black => {
                            if from == Square::from_algebraic("a8").unwrap() {
                                self.black_queenside = false;
                            } else if from == Square::from_algebraic("h8").unwrap() {
                                self.black_kingside = false;
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }
}


#[derive(Debug, Clone)]
pub struct Position {
    
    pub board: Board,
    
    pub side_to_move: Color,
    
    pub castling_rights: CastlingRights,
    
    pub en_passant: Option<Square>,
    
    pub halfmove_clock: u32,
    
    pub fullmove_number: u32,
    
    pub history: Vec<PositionState>,
}


#[derive(Debug, Clone)]
pub struct PositionState {
    pub board: Board,
    pub castling_rights: CastlingRights,
    pub en_passant: Option<Square>,
    pub halfmove_clock: u32,
}

impl Position {
    
    pub fn new() -> Self {
        Self {
            board: Board::starting_position(),
            side_to_move: Color::White,
            castling_rights: CastlingRights::all(),
            en_passant: None,
            halfmove_clock: 0,
            fullmove_number: 1,
            history: Vec::new(),
        }
    }

    
    pub fn from_fen(fen: &str) -> Result<Self> {
        let parts: Vec<&str> = fen.split_whitespace().collect();
        if parts.len() < 4 {
            return Err(Error::InvalidFen("Not enough parts".to_string()));
        }

        let board = parse_fen_board(parts[0])?;
        let side_to_move = match parts[1] {
            "w" => Color::White,
            "b" => Color::Black,
            _ => return Err(Error::InvalidFen("Invalid side to move".to_string())),
        };

        let castling_rights = parse_fen_castling(parts[2])?;
        let en_passant = if parts[3] == "-" {
            None
        } else {
            Some(
                Square::from_algebraic(parts[3])
                    .ok_or_else(|| Error::InvalidFen("Invalid en passant square".to_string()))?,
            )
        };

        let halfmove_clock = parts
            .get(4)
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);
        let fullmove_number = parts
            .get(5)
            .and_then(|s| s.parse().ok())
            .unwrap_or(1);

        Ok(Self {
            board,
            side_to_move,
            castling_rights,
            en_passant,
            halfmove_clock,
            fullmove_number,
            history: Vec::new(),
        })
    }

    
    pub fn to_fen(&self) -> String {
        let mut fen = String::new();

        
        for rank in (0..8).rev() {
            let mut empty_count = 0;
            for file in 0..8 {
                let square = Square::new(file, rank);
                if let Some(piece) = self.board.piece_at(square) {
                    if empty_count > 0 {
                        fen.push_str(&empty_count.to_string());
                        empty_count = 0;
                    }
                    fen.push(piece.to_char());
                } else {
                    empty_count += 1;
                }
            }
            if empty_count > 0 {
                fen.push_str(&empty_count.to_string());
            }
            if rank > 0 {
                fen.push('/');
            }
        }

        
        fen.push(' ');
        fen.push(match self.side_to_move {
            Color::White => 'w',
            Color::Black => 'b',
        });

        
        fen.push(' ');
        let mut castling = String::new();
        if self.castling_rights.white_kingside {
            castling.push('K');
        }
        if self.castling_rights.white_queenside {
            castling.push('Q');
        }
        if self.castling_rights.black_kingside {
            castling.push('k');
        }
        if self.castling_rights.black_queenside {
            castling.push('q');
        }
        if castling.is_empty() {
            castling.push('-');
        }
        fen.push_str(&castling);

        
        fen.push(' ');
        if let Some(sq) = self.en_passant {
            fen.push_str(&sq.to_algebraic());
        } else {
            fen.push('-');
        }

        
        fen.push(' ');
        fen.push_str(&self.halfmove_clock.to_string());
        fen.push(' ');
        fen.push_str(&self.fullmove_number.to_string());

        fen
    }

    
    pub fn make_move(&mut self, mv: &Move) -> Result<()> {
        
        let state = PositionState {
            board: self.board.clone(),
            castling_rights: self.castling_rights,
            en_passant: self.en_passant,
            halfmove_clock: self.halfmove_clock,
        };
        self.history.push(state);

        
        self.castling_rights.update(mv, &self.board);

        
        self.en_passant = None;

        
        if mv.is_en_passant() {
            self.make_en_passant_move(mv)?;
        } else if mv.is_castling() {
            self.make_castling_move(mv)?;
        } else if mv.is_promotion() {
            self.make_promotion_move(mv)?;
        } else {
            self.make_normal_move(mv);
        }

        
        if mv.piece_type() == PieceType::Pawn || self.board.piece_at(mv.to()).is_some() {
            self.halfmove_clock = 0;
        } else {
            self.halfmove_clock += 1;
        }

        
        if self.side_to_move == Color::Black {
            self.fullmove_number += 1;
        }
        self.side_to_move = self.side_to_move.opposite();

        Ok(())
    }

    
    pub fn undo_move(&mut self) -> Result<()> {
        if let Some(state) = self.history.pop() {
            self.board = state.board;
            self.castling_rights = state.castling_rights;
            self.en_passant = state.en_passant;
            self.halfmove_clock = state.halfmove_clock;

            
            self.side_to_move = self.side_to_move.opposite();
            if self.side_to_move == Color::Black {
                self.fullmove_number -= 1;
            }

            Ok(())
        } else {
            Err(Error::InvalidMove("No moves to undo".to_string()))
        }
    }

    
    pub fn generate_moves(&self) -> Vec<Move> {
        movegen::generate_moves(&self.board, self.side_to_move)
    }

    
    pub fn in_check(&self) -> bool {
        let king_square = self.find_king(self.side_to_move);
        self.is_square_attacked(king_square, self.side_to_move.opposite())
    }

    
    pub fn is_game_over(&self) -> bool {
        self.generate_moves().is_empty()
    }

    
    pub fn is_checkmate(&self) -> bool {
        self.in_check() && self.generate_moves().is_empty()
    }

    
    pub fn is_stalemate(&self) -> bool {
        !self.in_check() && self.generate_moves().is_empty()
    }

    
    fn find_king(&self, color: Color) -> Square {
        let king_bb = self.board.piece_bitboard(color, PieceType::King);
        Square::from(king_bb.lsb().expect("King not found"))
    }

    
    fn is_square_attacked(&self, square: Square, by_color: Color) -> bool {
        let sq_bb = square.bitboard();

        
        

        
        let pawn_attacks = match by_color {
            Color::White => sq_bb.southwest() | sq_bb.southeast(),
            Color::Black => sq_bb.northwest() | sq_bb.northeast(),
        };
        if (pawn_attacks & self.board.piece_bitboard(by_color, PieceType::Pawn)).0 != 0 {
            return true;
        }

        
        let knight_attacks = movegen::generate_knight_moves(&self.board, by_color, square);
        if (knight_attacks & self.board.piece_bitboard(by_color, PieceType::Knight)).0 != 0 {
            return true;
        }

        
        let king_attacks = movegen::generate_king_moves(&self.board, by_color, square);
        if (king_attacks & self.board.piece_bitboard(by_color, PieceType::King)).0 != 0 {
            return true;
        }

        
        let bishop_attacks = movegen::generate_bishop_attacks(&self.board, square);
        if (bishop_attacks & (self.board.piece_bitboard(by_color, PieceType::Bishop)
            | self.board.piece_bitboard(by_color, PieceType::Queen))).0 != 0
        {
            return true;
        }

        let rook_attacks = movegen::generate_rook_attacks(&self.board, square);
        if (rook_attacks & (self.board.piece_bitboard(by_color, PieceType::Rook)
            | self.board.piece_bitboard(by_color, PieceType::Queen))).0 != 0
        {
            return true;
        }

        false
    }

    
    fn make_normal_move(&mut self, mv: &Move) {
        let from = mv.from();
        let to = mv.to();
        let piece = self.board.piece_at(from).expect("No piece at from square");

        
        if piece.piece_type == PieceType::Pawn {
            let rank_diff = (to.rank() as i8 - from.rank() as i8).abs();
            if rank_diff == 2 {
                
                let ep_rank = if piece.color == Color::White { from.rank() + 1 } else { from.rank() - 1 };
                self.en_passant = Some(Square::new(from.file(), ep_rank));
            }
        }

        
        self.board.set_piece(from, None);
        self.board.set_piece(to, Some(piece));
    }

    
    fn make_en_passant_move(&mut self, mv: &Move) -> Result<()> {
        let from = mv.from();
        let to = mv.to();
        let piece = self.board.piece_at(from).expect("No piece at from square");

        
        self.board.set_piece(from, None);
        self.board.set_piece(to, Some(piece));

        
        let captured_pawn_square = match piece.color {
            Color::White => Square::new(to.file(), to.rank() - 1),
            Color::Black => Square::new(to.file(), to.rank() + 1),
        };
        self.board.set_piece(captured_pawn_square, None);

        Ok(())
    }

    
    fn make_castling_move(&mut self, mv: &Move) -> Result<()> {
        let from = mv.from();
        let to = mv.to();
        let piece = self.board.piece_at(from).expect("No piece at from square");

        
        self.board.set_piece(from, None);
        self.board.set_piece(to, Some(piece));

        
        let (rook_from, rook_to) = match to {
            
            sq if sq == Square::from_algebraic("g1").unwrap() => {
                (Square::from_algebraic("h1").unwrap(), Square::from_algebraic("f1").unwrap())
            }
            sq if sq == Square::from_algebraic("c1").unwrap() => {
                (Square::from_algebraic("a1").unwrap(), Square::from_algebraic("d1").unwrap())
            }
            sq if sq == Square::from_algebraic("g8").unwrap() => {
                (Square::from_algebraic("h8").unwrap(), Square::from_algebraic("f8").unwrap())
            }
            sq if sq == Square::from_algebraic("c8").unwrap() => {
                (Square::from_algebraic("a8").unwrap(), Square::from_algebraic("d8").unwrap())
            }
            _ => return Err(Error::InvalidMove("Invalid castling move".to_string())),
        };

        let rook = self.board.piece_at(rook_from).expect("No rook at castling square");
        self.board.set_piece(rook_from, None);
        self.board.set_piece(rook_to, Some(rook));

        Ok(())
    }

    
    fn make_promotion_move(&mut self, mv: &Move) -> Result<()> {
        let from = mv.from();
        let to = mv.to();
        let piece = self.board.piece_at(from).expect("No piece at from square");
        let promotion_piece = mv.promotion_piece().expect("No promotion piece specified");

        
        self.board.set_piece(from, None);
        self.board.set_piece(to, Some(Piece::new(piece.color, promotion_piece)));

        Ok(())
    }
}

impl Default for Position {
    fn default() -> Self {
        Self::new()
    }
}


fn parse_fen_board(fen: &str) -> Result<Board> {
    let mut board = Board::new();
    let ranks: Vec<&str> = fen.split('/').collect();

    if ranks.len() != 8 {
        return Err(Error::InvalidFen("Invalid number of ranks".to_string()));
    }

    for (rank_idx, rank_str) in ranks.iter().enumerate() {
        let rank = 7 - rank_idx; 
        let mut file = 0;

        for ch in rank_str.chars() {
            if file >= 8 {
                return Err(Error::InvalidFen("Too many files in rank".to_string()));
            }

            if let Some(digit) = ch.to_digit(10) {
                file += digit as u8;
            } else if let Some(piece) = Piece::from_char(ch) {
                let square = Square::new(file, rank as u8);
                board.set_piece(square, Some(piece));
                file += 1;
            } else {
                return Err(Error::InvalidFen(format!("Invalid character: {}", ch)));
            }
        }

        if file != 8 {
            return Err(Error::InvalidFen("Not enough files in rank".to_string()));
        }
    }

    board.update_derived();
    Ok(board)
}


fn parse_fen_castling(fen: &str) -> Result<CastlingRights> {
    let mut rights = CastlingRights::none();

    if fen == "-" {
        return Ok(rights);
    }

    for ch in fen.chars() {
        match ch {
            'K' => rights.white_kingside = true,
            'Q' => rights.white_queenside = true,
            'k' => rights.black_kingside = true,
            'q' => rights.black_queenside = true,
            _ => return Err(Error::InvalidFen("Invalid castling character".to_string())),
        }
    }

    Ok(rights)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position_new() {
        let pos = Position::new();
        assert_eq!(pos.side_to_move, Color::White);
        assert_eq!(pos.fullmove_number, 1);
        assert_eq!(pos.halfmove_clock, 0);
        assert!(pos.castling_rights.white_kingside);
        assert!(pos.castling_rights.white_queenside);
        assert!(pos.castling_rights.black_kingside);
        assert!(pos.castling_rights.black_queenside);
    }

    #[test]
    fn test_fen_roundtrip() {
        let start_pos = Position::new();
        let fen = start_pos.to_fen();
        let parsed_pos = Position::from_fen(&fen).unwrap();

        assert_eq!(start_pos.side_to_move, parsed_pos.side_to_move);
        assert_eq!(start_pos.castling_rights, parsed_pos.castling_rights);
        assert_eq!(start_pos.en_passant, parsed_pos.en_passant);
    }

    #[test]
    fn test_make_move() {
        let mut pos = Position::new();
        let moves = pos.generate_moves();
        assert_eq!(moves.len(), 20);

        
        let e2e4 = moves.iter().find(|m| m.to_algebraic() == "e2e4").unwrap();
        pos.make_move(e2e4).unwrap();

        assert_eq!(pos.side_to_move, Color::Black);
        assert_eq!(pos.fullmove_number, 1);
        assert_eq!(pos.halfmove_clock, 0);

        
        pos.undo_move().unwrap();
        assert_eq!(pos.side_to_move, Color::White);
    }

}