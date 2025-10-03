
use crate::{
    bitboard::Bitboard,
    board::{Board, Color, PieceType, Square},
    magic_simple as magic,
    moves::Move,
};

lazy_static::lazy_static! {
    
    static ref KNIGHT_ATTACKS: [Bitboard; 64] = {
        let mut attacks = [Bitboard::empty(); 64];
        for square in 0..64 {
            attacks[square as usize] = compute_knight_attacks(square);
        }
        attacks
    };

    
    static ref KING_ATTACKS: [Bitboard; 64] = {
        let mut attacks = [Bitboard::empty(); 64];
        for square in 0..64 {
            attacks[square as usize] = compute_king_attacks(square);
        }
        attacks
    };
}


pub fn generate_moves(board: &Board, color: Color) -> Vec<Move> {
    let mut moves = Vec::new();

    
    for piece_type in [
        PieceType::Pawn,
        PieceType::Knight,
        PieceType::Bishop,
        PieceType::Rook,
        PieceType::Queen,
        PieceType::King,
    ] {
        let pieces = board.piece_bitboard(color, piece_type);
        for from_square in pieces.squares() {
            let targets = generate_piece_moves(board, color, piece_type, from_square);
            for to_square in targets.squares() {
                
                if piece_type == PieceType::Pawn && is_promotion_rank(to_square, color) {
                    for promotion in [
                        PieceType::Knight,
                        PieceType::Bishop,
                        PieceType::Rook,
                        PieceType::Queen,
                    ] {
                        moves.push(Move::new_promotion(
                            Square::from(from_square),
                            Square::from(to_square),
                            piece_type,
                            promotion,
                        ));
                    }
                } else {
                    moves.push(Move::new(
                        Square::from(from_square),
                        Square::from(to_square),
                        piece_type,
                    ));
                }
            }
        }
    }

    
    

    moves
}


pub fn generate_piece_moves(
    board: &Board,
    color: Color,
    piece_type: PieceType,
    from_square: u8,
) -> Bitboard {
    let from = Square::from(from_square);
    let mut targets = match piece_type {
        PieceType::Pawn => generate_pawn_moves(board, color, from),
        PieceType::Knight => generate_knight_moves(board, color, from),
        PieceType::Bishop => generate_bishop_moves(board, color, from),
        PieceType::Rook => generate_rook_moves(board, color, from),
        PieceType::Queen => generate_queen_moves(board, color, from),
        PieceType::King => generate_king_moves(board, color, from),
    };

    
    targets &= !board.color_bitboard(color);

    targets
}


fn generate_pawn_moves(board: &Board, color: Color, from: Square) -> Bitboard {
    let mut moves = Bitboard::empty();
    let from_idx = from.index();

    match color {
        Color::White => {
            
            let single_push = from_idx + 8;
            if single_push < 64 && board.empty.has_square(single_push) {
                moves.set_square(single_push);

                
                if from.is_on_rank(1) {
                    let double_push = from_idx + 16;
                    if double_push < 64 && board.empty.has_square(double_push) {
                        moves.set_square(double_push);
                    }
                }
            }

            
            let capture_east = from_idx + 9;
            if capture_east < 64 && from.file() < 7 && board.black.has_square(capture_east) {
                moves.set_square(capture_east);
            }

            let capture_west = from_idx + 7;
            if capture_west < 64 && from.file() > 0 && board.black.has_square(capture_west) {
                moves.set_square(capture_west);
            }
        }
        Color::Black => {
            
            let single_push = from_idx as i8 - 8;
            if single_push >= 0 && board.empty.has_square(single_push as u8) {
                moves.set_square(single_push as u8);

                
                if from.is_on_rank(6) {
                    let double_push = from_idx as i8 - 16;
                    if double_push >= 0 && board.empty.has_square(double_push as u8) {
                        moves.set_square(double_push as u8);
                    }
                }
            }

            
            let capture_east = from_idx as i8 - 7;
            if capture_east >= 0 && from.file() < 7 && board.white.has_square(capture_east as u8) {
                moves.set_square(capture_east as u8);
            }

            let capture_west = from_idx as i8 - 9;
            if capture_west >= 0 && from.file() > 0 && board.white.has_square(capture_west as u8) {
                moves.set_square(capture_west as u8);
            }
        }
    }

    moves
}


pub fn generate_knight_moves(board: &Board, color: Color, from: Square) -> Bitboard {
    KNIGHT_ATTACKS[from.index() as usize] & !board.color_bitboard(color)
}


pub fn generate_bishop_moves(board: &Board, color: Color, from: Square) -> Bitboard {
    magic::get_bishop_attacks(from.index(), board.occupied) & !board.color_bitboard(color)
}


pub fn generate_bishop_attacks(board: &Board, from: Square) -> Bitboard {
    magic::get_bishop_attacks(from.index(), board.occupied)
}


pub fn generate_rook_moves(board: &Board, color: Color, from: Square) -> Bitboard {
    magic::get_rook_attacks(from.index(), board.occupied) & !board.color_bitboard(color)
}


pub fn generate_rook_attacks(board: &Board, from: Square) -> Bitboard {
    magic::get_rook_attacks(from.index(), board.occupied)
}


pub fn generate_queen_moves(board: &Board, color: Color, from: Square) -> Bitboard {
    magic::get_queen_attacks(from.index(), board.occupied) & !board.color_bitboard(color)
}


pub fn generate_king_moves(board: &Board, color: Color, from: Square) -> Bitboard {
    KING_ATTACKS[from.index() as usize] & !board.color_bitboard(color)
}


fn compute_knight_attacks(square: u8) -> Bitboard {
    let mut attacks = Bitboard::empty();
    let rank = square / 8;
    let file = square % 8;

    
    let offsets = [
        (2, 1),
        (2, -1),
        (-2, 1),
        (-2, -1),
        (1, 2),
        (1, -2),
        (-1, 2),
        (-1, -2),
    ];

    for &(dr, df) in &offsets {
        let new_rank = rank as i8 + dr;
        let new_file = file as i8 + df;

        if new_rank >= 0 && new_rank < 8 && new_file >= 0 && new_file < 8 {
            attacks.0 |= 1u64 << (new_rank * 8 + new_file);
        }
    }

    attacks
}


fn compute_king_attacks(square: u8) -> Bitboard {
    let mut attacks = Bitboard::empty();
    let rank = square / 8;
    let file = square % 8;

    
    for dr in -1..=1 {
        for df in -1..=1 {
            if dr == 0 && df == 0 {
                continue;
            }

            let new_rank = rank as i8 + dr;
            let new_file = file as i8 + df;

            if new_rank >= 0 && new_rank < 8 && new_file >= 0 && new_file < 8 {
                attacks.0 |= 1u64 << (new_rank * 8 + new_file);
            }
        }
    }

    attacks
}


fn is_promotion_rank(square: u8, color: Color) -> bool {
    let rank = square / 8;
    match color {
        Color::White => rank == 7,
        Color::Black => rank == 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_knight_attacks() {
        
        let attacks = KNIGHT_ATTACKS[1]; 
        assert!(attacks.has_square(11)); 
        assert!(attacks.has_square(16)); 
        assert!(attacks.has_square(18)); 
        assert!(!attacks.has_square(0)); 
    }

    #[test]
    fn test_king_attacks() {
        
        let attacks = KING_ATTACKS[4]; 
        assert!(attacks.has_square(3)); 
        assert!(attacks.has_square(5)); 
        assert!(attacks.has_square(11)); 
        assert!(attacks.has_square(12)); 
        assert!(attacks.has_square(13)); 
        assert!(!attacks.has_square(0)); 
    }

    #[test]
    fn test_pawn_moves() {
        let board = Board::starting_position();

        
        let moves = generate_pawn_moves(&board, Color::White, Square::from_algebraic("e2").unwrap());
        assert!(moves.has_square(Square::from_algebraic("e3").unwrap().index()));
        assert!(moves.has_square(Square::from_algebraic("e4").unwrap().index()));

        
        let moves = generate_pawn_moves(&board, Color::Black, Square::from_algebraic("e7").unwrap());
        assert!(moves.has_square(Square::from_algebraic("e6").unwrap().index()));
        assert!(moves.has_square(Square::from_algebraic("e5").unwrap().index()));
    }

    #[test]
    fn test_knight_moves() {
        let board = Board::starting_position();

        
        let moves = generate_knight_moves(&board, Color::White, Square::from_algebraic("b1").unwrap());
        assert!(moves.has_square(Square::from_algebraic("a3").unwrap().index()));
        assert!(moves.has_square(Square::from_algebraic("c3").unwrap().index()));
        assert!(!moves.has_square(Square::from_algebraic("d2").unwrap().index())); 
    }

    #[test]
    fn test_move_generation() {
        magic::init();
        let board = Board::starting_position();

        let moves = generate_moves(&board, Color::White);
        assert_eq!(moves.len(), 20); 

        
        for mv in moves {
            assert!(mv.from().index() < 64);
            assert!(mv.to().index() < 64);
            assert_ne!(mv.from(), mv.to());
        }
    }
}