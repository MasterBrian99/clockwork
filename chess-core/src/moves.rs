
use crate::board::{Color, PieceType, Square};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Move {
    data: u32,
}

impl Move {
    pub fn new(from: Square, to: Square, piece_type: PieceType) -> Self {
        let data = (from.index() as u32)
            | ((to.index() as u32) << 6)
            | ((piece_type as u32) << 12);
        Self { data }
    }

    pub fn new_promotion(from: Square, to: Square, piece_type: PieceType, promotion: PieceType) -> Self {
        let data = (from.index() as u32)
            | ((to.index() as u32) << 6)
            | ((piece_type as u32) << 12)
            | ((promotion as u32) << 16)
            | (1 << 20); // Promotion flag
        Self { data }
    }

    pub fn new_en_passant(from: Square, to: Square) -> Self {
        let data = (from.index() as u32)
            | ((to.index() as u32) << 6)
            | ((PieceType::Pawn as u32) << 12)
            | (1 << 21); // En passant flag
        Self { data }
    }

    pub fn new_castling(from: Square, to: Square, _color: Color) -> Self {
        let data = (from.index() as u32)
            | ((to.index() as u32) << 6)
            | ((PieceType::King as u32) << 12)
            | (1 << 22); // Castling flag
        Self { data }
    }

    /// Get the source square
    pub fn from(self) -> Square {
        Square((self.data & 0x3F) as u8)
    }

    /// Get the destination square
    pub fn to(self) -> Square {
        Square(((self.data >> 6) & 0x3F) as u8)
    }

    pub fn piece_type(self) -> PieceType {
        match (self.data >> 12) & 0x7 {
            0 => PieceType::Pawn,
            1 => PieceType::Knight,
            2 => PieceType::Bishop,
            3 => PieceType::Rook,
            4 => PieceType::Queen,
            5 => PieceType::King,
            _ => unreachable!(),
        }
    }

    /// Check if this is a promotion move
    pub fn is_promotion(self) -> bool {
        (self.data & (1 << 20)) != 0
    }

    pub fn promotion_piece(self) -> Option<PieceType> {
        if self.is_promotion() {
            match (self.data >> 16) & 0x7 {
                0 => Some(PieceType::Pawn),
                1 => Some(PieceType::Knight),
                2 => Some(PieceType::Bishop),
                3 => Some(PieceType::Rook),
                4 => Some(PieceType::Queen),
                5 => Some(PieceType::King),
                _ => None,
            }
        } else {
            None
        }
    }

    /// Check if this is an en passant move
    pub fn is_en_passant(self) -> bool {
        (self.data & (1 << 21)) != 0
    }

    /// Check if this is a castling move
    pub fn is_castling(self) -> bool {
        (self.data & (1 << 22)) != 0
    }

    /// Check if this is a capture (requires board context)
    pub fn is_capture(self, board: &crate::board::Board) -> bool {
        let to_square = self.to();
        board.piece_at(to_square).is_some() || self.is_en_passant()
    }

    /// Convert to algebraic notation (simplified)
    pub fn to_algebraic(self) -> String {
        let from = self.from().to_algebraic();
        let to = self.to().to_algebraic();

        if self.is_promotion() {
            if let Some(promotion) = self.promotion_piece() {
                let promotion_char = match promotion {
                    PieceType::Knight => 'n',
                    PieceType::Bishop => 'b',
                    PieceType::Rook => 'r',
                    PieceType::Queen => 'q',
                    _ => '?',
                };
                return format!("{}{}{}", from, to, promotion_char);
            }
        }

        format!("{}{}", from, to)
    }

    pub fn from_algebraic(s: &str, piece_type: PieceType) -> Option<Self> {
        if s.len() < 4 {
            return None;
        }

        let from_str = &s[0..2];
        let to_str = &s[2..4];

        let from = Square::from_algebraic(from_str)?;
        let to = Square::from_algebraic(to_str)?;

        if s.len() == 5 {
            // Promotion
            let promotion_char = s.chars().nth(4)?;
            let promotion = match promotion_char {
                'n' | 'N' => PieceType::Knight,
                'b' | 'B' => PieceType::Bishop,
                'r' | 'R' => PieceType::Rook,
                'q' | 'Q' => PieceType::Queen,
                _ => return None,
            };
            Some(Self::new_promotion(from, to, piece_type, promotion))
        } else {
            Some(Self::new(from, to, piece_type))
        }
    }
}

impl std::fmt::Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_algebraic())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_creation() {
        let from = Square::from_algebraic("e2").unwrap();
        let to = Square::from_algebraic("e4").unwrap();
        let mv = Move::new(from, to, PieceType::Pawn);

        assert_eq!(mv.from(), from);
        assert_eq!(mv.to(), to);
        assert_eq!(mv.piece_type(), PieceType::Pawn);
        assert!(!mv.is_promotion());
        assert!(!mv.is_en_passant());
        assert!(!mv.is_castling());
    }

    #[test]
    fn test_promotion_move() {
        let from = Square::from_algebraic("e7").unwrap();
        let to = Square::from_algebraic("e8").unwrap();
        let mv = Move::new_promotion(from, to, PieceType::Pawn, PieceType::Queen);

        assert_eq!(mv.from(), from);
        assert_eq!(mv.to(), to);
        assert_eq!(mv.piece_type(), PieceType::Pawn);
        assert!(mv.is_promotion());
        assert_eq!(mv.promotion_piece(), Some(PieceType::Queen));
    }

    #[test]
    fn test_en_passant_move() {
        let from = Square::from_algebraic("e5").unwrap();
        let to = Square::from_algebraic("d6").unwrap();
        let mv = Move::new_en_passant(from, to);

        assert_eq!(mv.from(), from);
        assert_eq!(mv.to(), to);
        assert_eq!(mv.piece_type(), PieceType::Pawn);
        assert!(mv.is_en_passant());
    }

    #[test]
    fn test_castling_move() {
        let from = Square::from_algebraic("e1").unwrap();
        let to = Square::from_algebraic("g1").unwrap();
        let mv = Move::new_castling(from, to, Color::White);

        assert_eq!(mv.from(), from);
        assert_eq!(mv.to(), to);
        assert_eq!(mv.piece_type(), PieceType::King);
        assert!(mv.is_castling());
    }

    #[test]
    fn test_move_algebraic() {
        let from = Square::from_algebraic("e2").unwrap();
        let to = Square::from_algebraic("e4").unwrap();
        let mv = Move::new(from, to, PieceType::Pawn);

        assert_eq!(mv.to_algebraic(), "e2e4");

        let promotion = Move::new_promotion(from, to, PieceType::Pawn, PieceType::Queen);
        assert_eq!(promotion.to_algebraic(), "e2e4q");
    }
}