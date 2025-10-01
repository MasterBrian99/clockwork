
use std::fmt;

#[derive(Copy, Clone, PartialEq, Eq, Default)]
pub struct Bitboard(pub u64);

impl Bitboard {
    pub const fn empty() -> Self {
        Bitboard(0)
    }

    pub const fn full() -> Self {
        Bitboard(u64::MAX)
    }

    pub const fn from_square(square: u8) -> Self {
        Bitboard(1u64 << square)
    }

    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }

    pub const fn has_square(self, square: u8) -> bool {
        (self.0 & (1u64 << square)) != 0
    }

    pub fn set_square(&mut self, square: u8) {
        self.0 |= 1u64 << square;
    }

    pub fn clear_square(&mut self, square: u8) {
        self.0 &= !(1u64 << square);
    }

    pub fn toggle_square(&mut self, square: u8) {
        self.0 ^= 1u64 << square;
    }

    pub fn count(self) -> u32 {
        self.0.count_ones()
    }

    pub fn lsb(self) -> Option<u8> {
        if self.0 == 0 {
            None
        } else {
            Some(self.0.trailing_zeros() as u8)
        }
    }

    pub fn msb(self) -> Option<u8> {
        if self.0 == 0 {
            None
        } else {
            Some((63 - self.0.leading_zeros()) as u8)
        }
    }

    pub fn pop_lsb(&mut self) -> Option<u8> {
        let lsb = self.lsb();
        if let Some(_sq) = lsb {
            self.0 &= self.0 - 1; // Clear the LSB
        }
        lsb
    }

    /// Shift the bitboard north (towards rank 8)
    pub fn north(self) -> Self {
        Bitboard(self.0 << 8)
    }

    /// Shift the bitboard south (towards rank 1)
    pub fn south(self) -> Self {
        Bitboard(self.0 >> 8)
    }

    /// Shift the bitboard east (towards file h)
    pub fn east(self) -> Self {
        Bitboard((self.0 << 1) & !FILE_A.0)
    }

    /// Shift the bitboard west (towards file a)
    pub fn west(self) -> Self {
        Bitboard((self.0 >> 1) & !FILE_H.0)
    }

    /// Shift the bitboard northeast (north + east)
    pub fn northeast(self) -> Self {
        self.north().east()
    }

    /// Shift the bitboard northwest (north + west)
    pub fn northwest(self) -> Self {
        self.north().west()
    }

    /// Shift the bitboard southeast (south + east)
    pub fn southeast(self) -> Self {
        self.south().east()
    }

    /// Shift the bitboard southwest (south + west)
    pub fn southwest(self) -> Self {
        self.south().west()
    }

    pub fn squares(self) -> BitboardIterator {
        BitboardIterator(self)
    }
}

impl std::ops::BitOr for Bitboard {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 | rhs.0)
    }
}

impl std::ops::BitOrAssign for Bitboard {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl std::ops::BitAnd for Bitboard {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 & rhs.0)
    }
}

impl std::ops::BitAndAssign for Bitboard {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl std::ops::BitXor for Bitboard {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 ^ rhs.0)
    }
}

impl std::ops::BitXorAssign for Bitboard {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0;
    }
}

impl std::ops::Not for Bitboard {
    type Output = Self;

    fn not(self) -> Self::Output {
        Bitboard(!self.0)
    }
}

impl fmt::Debug for Bitboard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Bitboard({:016x})", self.0)
    }
}

impl fmt::Display for Bitboard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for rank in (0..8).rev() {
            for file in 0..8 {
                let square = rank * 8 + file;
                let ch = if self.has_square(square) { '1' } else { '.' };
                write!(f, "{}", ch)?;
            }
            if rank > 0 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

pub struct BitboardIterator(Bitboard);

impl Iterator for BitboardIterator {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_lsb()
    }
}

pub const FILE_A: Bitboard = Bitboard(0x0101010101010101);
pub const FILE_B: Bitboard = Bitboard(0x0202020202020202);
pub const FILE_C: Bitboard = Bitboard(0x0404040404040404);
pub const FILE_D: Bitboard = Bitboard(0x0808080808080808);
pub const FILE_E: Bitboard = Bitboard(0x1010101010101010);
pub const FILE_F: Bitboard = Bitboard(0x2020202020202020);
pub const FILE_G: Bitboard = Bitboard(0x4040404040404040);
pub const FILE_H: Bitboard = Bitboard(0x8080808080808080);

pub const RANK_1: Bitboard = Bitboard(0x00000000000000FF);
pub const RANK_2: Bitboard = Bitboard(0x000000000000FF00);
pub const RANK_3: Bitboard = Bitboard(0x0000000000FF0000);
pub const RANK_4: Bitboard = Bitboard(0x00000000FF000000);
pub const RANK_5: Bitboard = Bitboard(0x000000FF00000000);
pub const RANK_6: Bitboard = Bitboard(0x0000FF0000000000);
pub const RANK_7: Bitboard = Bitboard(0x00FF000000000000);
pub const RANK_8: Bitboard = Bitboard(0xFF00000000000000);

pub const LIGHT_SQUARES: Bitboard = Bitboard(0x55AA55AA55AA55AA);
pub const DARK_SQUARES: Bitboard = Bitboard(0xAA55AA55AA55AA55);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bitboard_basics() {
        let bb = Bitboard::empty();
        assert!(bb.is_empty());
        assert_eq!(bb.count(), 0);

        let bb = Bitboard::from_square(0); // a1
        assert!(!bb.is_empty());
        assert!(bb.has_square(0));
        assert_eq!(bb.count(), 1);
        assert_eq!(bb.lsb(), Some(0));

        let mut bb = Bitboard::from_square(0);
        bb.set_square(1);
        assert!(bb.has_square(0));
        assert!(bb.has_square(1));
        assert_eq!(bb.count(), 2);

        bb.clear_square(0);
        assert!(!bb.has_square(0));
        assert!(bb.has_square(1));
        assert_eq!(bb.count(), 1);
    }

    #[test]
    fn test_bitboard_shifts() {
        let bb = Bitboard::from_square(0); // a1
        assert_eq!(bb.north(), Bitboard::from_square(8)); // a2
        assert_eq!(bb.east(), Bitboard::from_square(1)); // b1
        assert_eq!(bb.northeast(), Bitboard::from_square(9)); // b2

        let bb = Bitboard::from_square(63); // h8
        assert_eq!(bb.south(), Bitboard::from_square(55)); // h7
        assert_eq!(bb.west(), Bitboard::from_square(62)); // g8
        assert_eq!(bb.southwest(), Bitboard::from_square(54)); // g7
    }

    #[test]
    fn test_bitboard_iterator() {
        let mut bb = Bitboard::empty();
        bb.set_square(0);
        bb.set_square(2);
        bb.set_square(4);

        let squares: Vec<u8> = bb.squares().collect();
        assert_eq!(squares, vec![0, 2, 4]);
    }
}