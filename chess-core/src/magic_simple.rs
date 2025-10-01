
use crate::bitboard::Bitboard;

pub fn get_rook_attacks(square: u8, occupancy: Bitboard) -> Bitboard {
    let mut attacks = Bitboard::empty();
    let rank = square / 8;
    let file = square % 8;

    // North
    for r in (rank + 1)..8 {
        let sq = r * 8 + file;
        attacks.set_square(sq);
        if occupancy.has_square(sq) {
            break;
        }
    }

    // South
    for r in (0..rank).rev() {
        let sq = r * 8 + file;
        attacks.set_square(sq);
        if occupancy.has_square(sq) {
            break;
        }
    }

    // East
    for f in (file + 1)..8 {
        let sq = rank * 8 + f;
        attacks.set_square(sq);
        if occupancy.has_square(sq) {
            break;
        }
    }

    // West
    for f in (0..file).rev() {
        let sq = rank * 8 + f;
        attacks.set_square(sq);
        if occupancy.has_square(sq) {
            break;
        }
    }

    attacks
}

pub fn get_bishop_attacks(square: u8, occupancy: Bitboard) -> Bitboard {
    let mut attacks = Bitboard::empty();
    let rank = square / 8;
    let file = square % 8;

    // Northeast
    for (r, f) in ((rank + 1)..8).zip((file + 1)..8) {
        let sq = r * 8 + f;
        attacks.set_square(sq);
        if occupancy.has_square(sq) {
            break;
        }
    }

    // Northwest
    for (r, f) in ((rank + 1)..8).zip((0..file).rev()) {
        let sq = r * 8 + f;
        attacks.set_square(sq);
        if occupancy.has_square(sq) {
            break;
        }
    }

    // Southeast
    for (r, f) in ((0..rank).rev()).zip((file + 1)..8) {
        let sq = r * 8 + f;
        attacks.set_square(sq);
        if occupancy.has_square(sq) {
            break;
        }
    }

    // Southwest
    for (r, f) in ((0..rank).rev()).zip((0..file).rev()) {
        let sq = r * 8 + f;
        attacks.set_square(sq);
        if occupancy.has_square(sq) {
            break;
        }
    }
    attacks
}

pub fn get_queen_attacks(square: u8, occupancy: Bitboard) -> Bitboard {
    get_rook_attacks(square, occupancy) | get_bishop_attacks(square, occupancy)
}

pub fn init() {
    // Nothing to initialize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rook_attacks() {
        // Test rook on empty board
        let attacks = get_rook_attacks(0, Bitboard::empty()); // a1
        assert!(attacks.has_square(1)); // b1
        assert!(attacks.has_square(8)); // a2
        assert!(!attacks.has_square(9)); // b2

        // Test rook with blocking piece
        let mut occupancy = Bitboard::empty();
        occupancy.set_square(1); // Block b1
        let attacks = get_rook_attacks(0, occupancy);
        assert!(!attacks.has_square(2)); // c1 should be blocked
        assert!(attacks.has_square(1)); // b1 should be included (capture)
    }

    #[test]
    fn test_bishop_attacks() {
        // Test bishop on empty board
        let attacks = get_bishop_attacks(0, Bitboard::empty()); // a1
        assert!(attacks.has_square(9)); // b2
        assert!(!attacks.has_square(1)); // b1 (not diagonal)

        // Test bishop with blocking piece
        let mut occupancy = Bitboard::empty();
        occupancy.set_square(9); // Block b2
        let attacks = get_bishop_attacks(0, occupancy);
        assert!(!attacks.has_square(18)); // c3 should be blocked
        assert!(attacks.has_square(9)); // b2 should be included (capture)
    }
}