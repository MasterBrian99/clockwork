
use crate::bitboard::Bitboard;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    White,
    Black,
}

impl Color {
    pub fn opposite(self) -> Self {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }

    pub fn pawn_direction(self) -> i8 {
        match self {
            Color::White => 8,
            Color::Black => -8,
        }
    }

    pub fn back_rank(self) -> Bitboard {
        match self {
            Color::White => crate::bitboard::RANK_1,
            Color::Black => crate::bitboard::RANK_8,
        }
    }

    pub fn pawn_start_rank(self) -> Bitboard {
        match self {
            Color::White => crate::bitboard::RANK_2,
            Color::Black => crate::bitboard::RANK_7,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl PieceType {
    pub fn from_char(ch: char) -> Option<Self> {
        match ch {
            'P' | 'p' => Some(PieceType::Pawn),
            'N' | 'n' => Some(PieceType::Knight),
            'B' | 'b' => Some(PieceType::Bishop),
            'R' | 'r' => Some(PieceType::Rook),
            'Q' | 'q' => Some(PieceType::Queen),
            'K' | 'k' => Some(PieceType::King),
            _ => None,
        }
    }

    pub fn to_char(self, color: Color) -> char {
        let ch = match self {
            PieceType::Pawn => 'p',
            PieceType::Knight => 'n',
            PieceType::Bishop => 'b',
            PieceType::Rook => 'r',
            PieceType::Queen => 'q',
            PieceType::King => 'k',
        };
        match color {
            Color::White => ch.to_ascii_uppercase(),
            Color::Black => ch,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Piece {
    pub color: Color,
    pub piece_type: PieceType,
}

impl Piece {
    pub fn new(color: Color, piece_type: PieceType) -> Self {
        Self { color, piece_type }
    }

    pub fn from_char(ch: char) -> Option<Self> {
        let color = if ch.is_uppercase() {
            Color::White
        } else {
            Color::Black
        };
        PieceType::from_char(ch).map(|piece_type| Self::new(color, piece_type))
    }

    pub fn to_char(self) -> char {
        self.piece_type.to_char(self.color)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Square(pub u8);

impl Square {
    pub const fn new(file: u8, rank: u8) -> Self {
        Square(rank * 8 + file)
    }

    pub fn from_algebraic(s: &str) -> Option<Self> {
        if s.len() != 2 {
            return None;
        }
        let mut chars = s.chars();
        let file_char = chars.next()?;
        let rank_char = chars.next()?;

        let file = (file_char as u8).saturating_sub(b'a');
        let rank = (rank_char as u8).saturating_sub(b'1');

        if file < 8 && rank < 8 {
            Some(Square::new(file, rank))
        } else {
            None
        }
    }

    pub fn to_algebraic(self) -> String {
        let file = (b'a' + self.file()) as char;
        let rank = (b'1' + self.rank()) as char;
        format!("{}{}", file, rank)
    }

    /// Get the file (0-7, where 0=a, 7=h)
    pub const fn file(self) -> u8 {
        self.0 & 7
    }

    /// Get the rank (0-7, where 0=1, 7=8)
    pub const fn rank(self) -> u8 {
        self.0 >> 3
    }

    /// Get the square index (0-63)
    pub const fn index(self) -> u8 {
        self.0
    }

    /// Check if the square is on the given rank
    pub const fn is_on_rank(self, rank: u8) -> bool {
        self.rank() == rank
    }

    /// Check if the square is on the given file
    pub const fn is_on_file(self, file: u8) -> bool {
        self.file() == file
    }

    /// Get the bitboard representation of this square
    pub fn bitboard(self) -> Bitboard {
        Bitboard::from_square(self.0)
    }
}

impl From<u8> for Square {
    fn from(value: u8) -> Self {
        Square(value)
    }
}

impl From<Square> for u8 {
    fn from(square: Square) -> Self {
        square.0
    }
}

#[derive(Debug, Clone)]
pub struct Board {
    pub pieces: [[Bitboard; 6]; 2], // [color][piece_type]
    pub occupied: Bitboard,
    pub white: Bitboard,
    pub black: Bitboard,
    pub empty: Bitboard,
}

impl Board {
    pub fn new() -> Self {
        Self {
            pieces: [[Bitboard::empty(); 6]; 2],
            occupied: Bitboard::empty(),
            white: Bitboard::empty(),
            black: Bitboard::empty(),
            empty: Bitboard::full(),
        }
    }

    pub fn starting_position() -> Self {
        let mut board = Self::new();

        board.pieces[Color::White as usize][PieceType::Pawn as usize] = crate::bitboard::RANK_2;
        board.pieces[Color::Black as usize][PieceType::Pawn as usize] = crate::bitboard::RANK_7;

        let back_rank_pieces = [
            PieceType::Rook,
            PieceType::Knight,
            PieceType::Bishop,
            PieceType::Queen,
            PieceType::King,
            PieceType::Bishop,
            PieceType::Knight,
            PieceType::Rook,
        ];

        for (file, &piece_type) in back_rank_pieces.iter().enumerate() {
            let white_square = Square::new(file as u8, 0);
            let black_square = Square::new(file as u8, 7);

            board.pieces[Color::White as usize][piece_type as usize]
                .set_square(white_square.index());
            board.pieces[Color::Black as usize][piece_type as usize]
                .set_square(black_square.index());
        }

        board.update_derived();
        board
    }

    pub fn update_derived(&mut self) {
        self.white = Bitboard::empty();
        self.black = Bitboard::empty();

        for piece_type in 0..6 {
            self.white |= self.pieces[Color::White as usize][piece_type];
            self.black |= self.pieces[Color::Black as usize][piece_type];
        }

        self.occupied = self.white | self.black;
        self.empty = !self.occupied;
    }

    pub fn piece_at(&self, square: Square) -> Option<Piece> {
        let sq_bb = square.bitboard();

        for color in [Color::White, Color::Black] {
            for piece_type in [
                PieceType::Pawn,
                PieceType::Knight,
                PieceType::Bishop,
                PieceType::Rook,
                PieceType::Queen,
                PieceType::King,
            ] {
                if (self.pieces[color as usize][piece_type as usize] & sq_bb).0 != 0 {
                    return Some(Piece::new(color, piece_type));
                }
            }
        }

        None
    }

    pub fn set_piece(&mut self, square: Square, piece: Option<Piece>) {
        for color in [Color::White, Color::Black] {
            for piece_type in [
                PieceType::Pawn,
                PieceType::Knight,
                PieceType::Bishop,
                PieceType::Rook,
                PieceType::Queen,
                PieceType::King,
            ] {
                self.pieces[color as usize][piece_type as usize].clear_square(square.index());
            }
        }

        if let Some(piece) = piece {
            self.pieces[piece.color as usize][piece.piece_type as usize].set_square(square.index());
        }

        self.update_derived();
    }

    pub fn piece_bitboard(&self, color: Color, piece_type: PieceType) -> Bitboard {
        self.pieces[color as usize][piece_type as usize]
    }

    pub fn color_bitboard(&self, color: Color) -> Bitboard {
        match color {
            Color::White => self.white,
            Color::Black => self.black,
        }
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::starting_position()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_square_algebraic() {
        assert_eq!(Square::from_algebraic("a1"), Some(Square::new(0, 0)));
        assert_eq!(Square::from_algebraic("e4"), Some(Square::new(4, 3)));
        assert_eq!(Square::from_algebraic("h8"), Some(Square::new(7, 7)));
        assert_eq!(Square::from_algebraic("i9"), None);

        assert_eq!(Square::new(0, 0).to_algebraic(), "a1");
        assert_eq!(Square::new(4, 3).to_algebraic(), "e4");
        assert_eq!(Square::new(7, 7).to_algebraic(), "h8");
    }

    #[test]
    fn test_board_starting_position() {
        let board = Board::starting_position();

        // Check that all squares are properly set
        assert_eq!(board.piece_at(Square::from_algebraic("e1").unwrap()),
                   Some(Piece::new(Color::White, PieceType::King)));
        assert_eq!(board.piece_at(Square::from_algebraic("e8").unwrap()),
                   Some(Piece::new(Color::Black, PieceType::King)));
        assert_eq!(board.piece_at(Square::from_algebraic("a2").unwrap()),
                   Some(Piece::new(Color::White, PieceType::Pawn)));
        assert_eq!(board.piece_at(Square::from_algebraic("a7").unwrap()),
                   Some(Piece::new(Color::Black, PieceType::Pawn)));

        // Check empty squares
        assert_eq!(board.piece_at(Square::from_algebraic("e4").unwrap()), None);

        // Check bitboard counts
        assert_eq!(board.white.count(), 16);
        assert_eq!(board.black.count(), 16);
        assert_eq!(board.occupied.count(), 32);
        assert_eq!(board.empty.count(), 32);
    }

    #[test]
    fn test_piece_chars() {
        assert_eq!(Piece::from_char('K'), Some(Piece::new(Color::White, PieceType::King)));
        assert_eq!(Piece::from_char('q'), Some(Piece::new(Color::Black, PieceType::Queen)));
        assert_eq!(Piece::from_char('x'), None);

        assert_eq!(Piece::new(Color::White, PieceType::King).to_char(), 'K');
        assert_eq!(Piece::new(Color::Black, PieceType::Queen).to_char(), 'q');
    }
}