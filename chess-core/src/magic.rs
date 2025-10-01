
use crate::bitboard::Bitboard;

/// Magic numbers - copied from known good sources
const ROOK_MAGICS: [u64; 64] = [
    0x0080001020400080, 0x0040001000200040, 0x0080081000200080, 0x0080040800100080,
    0x0080020400080080, 0x0080010200040080, 0x0080008001000200, 0x0080002040800100,
    0x0000800020400080, 0x0000400020005000, 0x0000801000200080, 0x0000800800100080,
    0x0000800400080080, 0x0000800200040080, 0x0000800100020080, 0x0000800040800100,
    0x0000208000400080, 0x0000404000201000, 0x0000808010002000, 0x0000808008001000,
    0x0000808004000800, 0x0000808002000400, 0x0000010100020004, 0x0000020000408104,
    0x0000208080004000, 0x0000200040005000, 0x0000100080200080, 0x0000080080100080,
    0x0000040080080080, 0x0000020080040080, 0x0000010080800200, 0x0000800080004100,
    0x0000204000800080, 0x0000200040401000, 0x0000100080802000, 0x0000080080801000,
    0x0000040080800800, 0x0000020080800400, 0x0000020001010004, 0x0000800040800100,
    0x0000204000808000, 0x0000200040008080, 0x0000100020008080, 0x0000080010008080,
    0x0000040008008080, 0x0000020004008080, 0x0000010002008080, 0x0000004081020004,
    0x0000204000800080, 0x0000200040008080, 0x0000100020008080, 0x0000080010008080,
    0x0000040008008080, 0x0000020004008080, 0x0000800100020080, 0x0000800041000080,
    0x00FFFCDDFCED714A, 0x007FFCDDFCED714A, 0x003FFFCDFFD88096, 0x0000040810002101,
    0x0001000204080011, 0x0001000204000801, 0x0001000082000401, 0x0001FFFAABFAD1A2,
];

const BISHOP_MAGICS: [u64; 64] = [
    0x0002020202020200, 0x0002020202020000, 0x0004010202000000, 0x0004040080000000,
    0x0001104000000000, 0x0000821040000000, 0x0000410410400000, 0x0000104104104000,
    0x0000040404040400, 0x0000020202020200, 0x0000040102020000, 0x0000040400800000,
    0x0000011040000000, 0x0000008210400000, 0x0000004104104000, 0x0000002082082000,
    0x0004000808080800, 0x0002000404040400, 0x0001000202020200, 0x0000800802004000,
    0x0000800400A00000, 0x0000200100884000, 0x0000400082082000, 0x0000200041041000,
    0x0002080010101000, 0x0001040008080800, 0x0000208004010400, 0x0000404004010200,
    0x0000840000802000, 0x0000404002011000, 0x0000808001041000, 0x0000404000820800,
    0x0001041000202000, 0x0000820800101000, 0x0000104400080800, 0x0000020080080080,
    0x0000404040040100, 0x0000808100020100, 0x0001010100020800, 0x0000808080010400,
    0x0000820820004000, 0x0000410410002000, 0x0000082088001000, 0x0000002011000800,
    0x0000080100400400, 0x0001010101000200, 0x0000608090A00C00, 0x0001010101000200,
    0x0000804040800100, 0x0001002020200100, 0x000080809000A100, 0x0000808080050100,
    0x0000808080080100, 0x0000808080100100, 0x0000404040004280, 0x0000404040002140,
    0x0000208104000080, 0x0000404040002120, 0x0000208104000040, 0x0000208104000020,
    0x0000208104000010, 0x0000081040000400, 0x0000041040000200, 0x0000021040000100,
];

static mut ROOK_ATTACKS: [[Bitboard; 4096]; 64] = [[Bitboard::empty(); 4096]; 64];
static mut BISHOP_ATTACKS: [[Bitboard; 512]; 64] = [[Bitboard::empty(); 512]; 64];

pub fn init() {
    unsafe {
        for square in 0..64 {
            init_rook_attacks(square);
            init_bishop_attacks(square);
        }
    }
}

pub fn get_rook_attacks(square: u8, occupancy: Bitboard) -> Bitboard {
    unsafe {
        let mask = rook_mask(square);
        let magic = ROOK_MAGICS[square as usize];
        let shift = rook_shift(square);
        let index = ((occupancy.0 & mask.0).wrapping_mul(magic) >> shift) as usize;
        ROOK_ATTACKS[square as usize][index]
    }
}

pub fn get_bishop_attacks(square: u8, occupancy: Bitboard) -> Bitboard {
    unsafe {
        let mask = bishop_mask(square);
        let magic = BISHOP_MAGICS[square as usize];
        let shift = bishop_shift(square);
        let index = ((occupancy.0 & mask.0).wrapping_mul(magic) >> shift) as usize;
        BISHOP_ATTACKS[square as usize][index]
    }
}

pub fn get_queen_attacks(square: u8, occupancy: Bitboard) -> Bitboard {
    get_rook_attacks(square, occupancy) | get_bishop_attacks(square, occupancy)
}

unsafe fn init_rook_attacks(square: u8) {
    let mask = rook_mask(square);
    let magic = ROOK_MAGICS[square as usize];
    let shift = rook_shift(square);

    let mut occupancy = Bitboard::empty();
    loop {
        let index = ((occupancy.0 & mask.0).wrapping_mul(magic) >> shift) as usize;
        ROOK_ATTACKS[square as usize][index] = rook_attacks(square, occupancy);

        // Next occupancy pattern
        occupancy.0 = occupancy.0.wrapping_sub(mask.0) & mask.0;
        if occupancy.0 == 0 {
            break;
        }
    }
}

unsafe fn init_bishop_attacks(square: u8) {
    let mask = bishop_mask(square);
    let magic = BISHOP_MAGICS[square as usize];
    let shift = bishop_shift(square);

 
    let mut occupancy = Bitboard::empty();
    loop {
        let index = ((occupancy.0 & mask.0).wrapping_mul(magic) >> shift) as usize;
        BISHOP_ATTACKS[square as usize][index] = bishop_attacks(square, occupancy);

        // Next occupancy pattern
        occupancy.0 = occupancy.0.wrapping_sub(mask.0) & mask.0;
        if occupancy.0 == 0 {
            break;
        }
    }
}

fn rook_mask(square: u8) -> Bitboard {
    let mut attacks = Bitboard::empty();
    let rank = square / 8;
    let file = square % 8;

    for f in 1..7 {
        if f != file {
            attacks.set_square(rank * 8 + f);
        }
    }

    for r in 1..7 {
        if r != rank {
            attacks.set_square(r * 8 + file);
        }
    }

    attacks
}

fn bishop_mask(square: u8) -> Bitboard {
    let mut attacks = Bitboard::empty();
    let rank = square / 8;
    let file = square % 8;

    for (r, f) in (1..7).zip(1..7) {
        let r_diff = r as i8 - rank as i8;
        let f_diff = f as i8 - file as i8;

        if r_diff.abs() == f_diff.abs() && r_diff != 0 {
            attacks.set_square(r * 8 + f);
        }
    }

    attacks
}

fn rook_shift(square: u8) -> u32 {
    let mask_count = rook_mask(square).count();
    if mask_count == 0 || mask_count >= 64 { 0 } else { 64 - mask_count }
}

fn bishop_shift(square: u8) -> u32 {
    let mask_count = bishop_mask(square).count();
    if mask_count == 0 || mask_count >= 64 { 0 } else { 64 - mask_count }
}

fn rook_attacks(square: u8, occupancy: Bitboard) -> Bitboard {
    let mut attacks = Bitboard::empty();
    let rank = square / 8;
    let file = square % 8;

    for r in (rank + 1)..8 {
        let sq = r * 8 + file;
        attacks.set_square(sq);
        if occupancy.has_square(sq) {
            break;
        }
    }

    for r in (0..rank).rev() {
        let sq = r * 8 + file;
        attacks.set_square(sq);
        if occupancy.has_square(sq) {
            break;
        }
    }

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

fn bishop_attacks(square: u8, occupancy: Bitboard) -> Bitboard {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_magic_init() {
        init(); // Should not panic - if panic, initialization failed or whatever
    }

    #[test]
    fn test_rook_attacks() {
        init();

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
        init();

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