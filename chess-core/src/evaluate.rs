

use crate::{
    board::{Board, Color, PieceType},
    position::Position,
};


const PIECE_VALUES: [i32; 6] = [
    100,   
    300,   
    300,   
    500,   
    900,   
    20000, 
];


const PAWN_TABLE: [i32; 64] = [
    0, 0, 0, 0, 0, 0, 0, 0, 50, 50, 50, 50, 50, 50, 50, 50, 10, 10, 20, 30, 30, 20, 10, 10, 5, 5,
    10, 25, 25, 10, 5, 5, 0, 0, 0, 20, 20, 0, 0, 0, 5, -5, -10, 0, 0, -10, -5, 5, 5, 10, 10, -20,
    -20, 10, 10, 5, 0, 0, 0, 0, 0, 0, 0, 0,
];


const KNIGHT_TABLE: [i32; 64] = [
    -50, -40, -30, -30, -30, -30, -40, -50, -40, -20, 0, 0, 0, 0, -20, -40, -30, 0, 10, 15, 15, 10,
    0, -30, -30, 5, 15, 20, 20, 15, 5, -30, -30, 0, 15, 20, 20, 15, 0, -30, -30, 5, 10, 15, 15, 10,
    5, -30, -40, -20, 0, 5, 5, 0, -20, -40, -50, -40, -30, -30, -30, -30, -40, -50,
];


const BISHOP_TABLE: [i32; 64] = [
    -20, -10, -10, -10, -10, -10, -10, -20, -10, 0, 0, 0, 0, 0, 0, -10, -10, 0, 5, 10, 10, 5, 0,
    -10, -10, 5, 5, 10, 10, 5, 5, -10, -10, 0, 10, 10, 10, 10, 0, -10, -10, 10, 10, 10, 10, 10, 10,
    -10, -10, 5, 0, 0, 0, 0, 5, -10, -20, -10, -10, -10, -10, -10, -10, -20,
];


const ROOK_TABLE: [i32; 64] = [
    0, 0, 0, 0, 0, 0, 0, 0, 5, 10, 10, 10, 10, 10, 10, 5, -5, 0, 0, 0, 0, 0, 0, -5, -5, 0, 0, 0, 0,
    0, 0, -5, -5, 0, 0, 0, 0, 0, 0, -5, -5, 0, 0, 0, 0, 0, 0, -5, -5, 0, 0, 0, 0, 0, 0, -5, 0, 0,
    0, 5, 5, 0, 0, 0,
];


const QUEEN_TABLE: [i32; 64] = [
    -20, -10, -10, -5, -5, -10, -10, -20, -10, 0, 0, 0, 0, 0, 0, -10, -10, 0, 5, 5, 5, 5, 0, -10,
    -5, 0, 5, 5, 5, 5, 0, -5, 0, 0, 5, 5, 5, 5, 0, -5, -10, 5, 5, 5, 5, 5, 0, -10, -10, 0, 5, 0, 0,
    0, 0, -10, -20, -10, -10, -5, -5, -10, -10, -20,
];


const KING_TABLE: [i32; 64] = [
    -30, -40, -40, -50, -50, -40, -40, -30, -30, -40, -40, -50, -50, -40, -40, -30, -30, -40, -40,
    -50, -50, -40, -40, -30, -30, -40, -40, -50, -50, -40, -40, -30, -20, -30, -30, -40, -40, -30,
    -30, -20, -10, -20, -20, -20, -20, -20, -20, -10, 20, 20, 0, 0, 0, 0, 20, 20, 20, 30, 10, 0, 0,
    10, 30, 20,
];


pub fn evaluate(position: &Position) -> i32 {
    let mut score = 0;

    
    score += material_score(&position.board);

    
    score += piece_square_score(&position.board);

    
    if position.side_to_move == Color::Black {
        score = -score;
    }

    score
}


fn material_score(board: &Board) -> i32 {
    let mut score = 0;

    for piece_type in [
        PieceType::Pawn,
        PieceType::Knight,
        PieceType::Bishop,
        PieceType::Rook,
        PieceType::Queen,
        PieceType::King,
    ] {
        let white_count = board.piece_bitboard(Color::White, piece_type).count() as i32;
        let black_count = board.piece_bitboard(Color::Black, piece_type).count() as i32;
        let piece_value = PIECE_VALUES[piece_type as usize];

        score += (white_count - black_count) * piece_value;
    }

    score
}


fn piece_square_score(board: &Board) -> i32 {
    let mut score = 0;

    
    for piece_type in [
        PieceType::Pawn,
        PieceType::Knight,
        PieceType::Bishop,
        PieceType::Rook,
        PieceType::Queen,
        PieceType::King,
    ] {
        let pieces = board.piece_bitboard(Color::White, piece_type);
        for square in pieces.squares() {
            score += get_piece_square_value(piece_type, square, Color::White);
        }
    }

    
    for piece_type in [
        PieceType::Pawn,
        PieceType::Knight,
        PieceType::Bishop,
        PieceType::Rook,
        PieceType::Queen,
        PieceType::King,
    ] {
        let pieces = board.piece_bitboard(Color::Black, piece_type);
        for square in pieces.squares() {
            score -= get_piece_square_value(piece_type, square, Color::Black);
        }
    }

    score
}


fn get_piece_square_value(piece_type: PieceType, square: u8, color: Color) -> i32 {
    let table_index = match color {
        Color::White => square as usize,
        Color::Black => 63 - square as usize, 
    };

    match piece_type {
        PieceType::Pawn => PAWN_TABLE[table_index],
        PieceType::Knight => KNIGHT_TABLE[table_index],
        PieceType::Bishop => BISHOP_TABLE[table_index],
        PieceType::Rook => ROOK_TABLE[table_index],
        PieceType::Queen => QUEEN_TABLE[table_index],
        PieceType::King => KING_TABLE[table_index],
    }
}


pub fn is_insufficient_material(board: &Board) -> bool {
    let total_pieces = board.occupied.count();

    if total_pieces == 2 {
        
        return true;
    }

    if total_pieces == 3 {
        
        let white_pieces = board.white.count();
        let black_pieces = board.black.count();

        if white_pieces == 2 && black_pieces == 1 {
            let minor_piece = if board
                .piece_bitboard(Color::White, PieceType::Bishop)
                .count()
                == 1
            {
                PieceType::Bishop
            } else if board
                .piece_bitboard(Color::White, PieceType::Knight)
                .count()
                == 1
            {
                PieceType::Knight
            } else {
                return false;
            };
            return minor_piece == PieceType::Bishop || minor_piece == PieceType::Knight;
        }

        if black_pieces == 2 && white_pieces == 1 {
            let minor_piece = if board
                .piece_bitboard(Color::Black, PieceType::Bishop)
                .count()
                == 1
            {
                PieceType::Bishop
            } else if board
                .piece_bitboard(Color::Black, PieceType::Knight)
                .count()
                == 1
            {
                PieceType::Knight
            } else {
                return false;
            };
            return minor_piece == PieceType::Bishop || minor_piece == PieceType::Knight;
        }
    }

    false
}


pub fn is_threefold_repetition(positions: &[String]) -> bool {
    if positions.len() < 6 {
        return false;
    }

    let mut counts = std::collections::HashMap::new();
    for fen in positions {
        *counts.entry(fen).or_insert(0) += 1;
        if counts[fen] >= 3 {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::Square;
    

    #[test]
    fn test_evaluate_starting_position() {
        let pos = Position::new();
        let score = evaluate(&pos);

        
        assert!(score.abs() < 50);
    }

    #[test]
    fn test_material_score() {
        let mut board = Board::new();

        
        board.set_piece(
            Square::from_algebraic("d1").unwrap(),
            Some(crate::board::Piece::new(Color::White, PieceType::Queen)),
        );
        board.update_derived();

        let score = material_score(&board);
        assert_eq!(score, 900); 
    }

    #[test]
    fn test_insufficient_material() {
        let mut board = Board::new();

        
        for square in 0..64 {
            board.set_piece(Square::from(square), None);
        }

        
        board.set_piece(
            Square::from_algebraic("e1").unwrap(),
            Some(crate::board::Piece::new(Color::White, PieceType::King)),
        );
        board.set_piece(
            Square::from_algebraic("e8").unwrap(),
            Some(crate::board::Piece::new(Color::Black, PieceType::King)),
        );
        board.update_derived();

        assert!(is_insufficient_material(&board));

        
        board.set_piece(
            Square::from_algebraic("c1").unwrap(),
            Some(crate::board::Piece::new(Color::White, PieceType::Bishop)),
        );
        board.update_derived();

        assert!(is_insufficient_material(&board));

        
        board.set_piece(
            Square::from_algebraic("a2").unwrap(),
            Some(crate::board::Piece::new(Color::White, PieceType::Pawn)),
        );
        board.update_derived();

        assert!(!is_insufficient_material(&board));
    }
}
