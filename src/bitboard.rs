use std::fmt;
use std::fmt::{Debug, Formatter};

use derive_more::{Add, AddAssign, BitAnd, BitOr, BitXor, BitXorAssign, Sub, SubAssign};

use lazy_static::lazy_static;

lazy_static! {
    static ref BIT_TABLE: [Bitboard; 64] = calculate_bit_table();
}
const UNIVERSE: u64 = std::u64::MAX;
const EMPTY: u64 = 0;

#[derive(
    Copy,
    Clone,
    PartialEq,
    Add,
    Sub,
    BitAnd,
    BitXor,
    BitOr,
    AddAssign,
    SubAssign,
    BitXorAssign,
    Default
)]
pub struct Bitboard(pub u64);

impl Debug for Bitboard {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Bitboard(0x{:X?})", self.0)
    }
}

impl Bitboard {
    pub fn lsb(&self) -> usize {
        #[allow(non_upper_case_globals)]
        const debruijn: u64 = 0x07edd5e59a4e28c2_u64;

        #[allow(non_upper_case_globals)]
        #[rustfmt::skip]
        const indexes: [usize; 64] = [
            63,  0, 58,  1, 59, 47, 53,  2,
            60, 39, 48, 27, 54, 33, 42,  3,
            61, 51, 37, 40, 49, 18, 28, 20,
            55, 30, 34, 11, 43, 14, 22,  4,
            62, 57, 46, 52, 38, 26, 32, 41,
            50, 36, 17, 19, 29, 10, 13, 21,
            56, 45, 25, 31, 35, 16,  9, 12,
            44, 24, 15,  8, 23,  7,  6,  5,
        ];

        let n = self.0;
        let m = (n & (!n + 1)).wrapping_mul(debruijn);
        indexes[(m >> 58) as usize]
    }

    pub fn msb(&self) -> usize {
        0
    }

    pub fn iter(&self) -> BitboardIter {
        BitboardIter::new(self)
    }
}

impl Into<Bitboard> for u64 {
    fn into(self) -> Bitboard {
        Bitboard(self)
    }
}

///
/// A wrapper around two bitboards to allow us to iterate
/// over each of the bits inside the first whilst maintaining
/// current iteration state in the second
///
/// # Examples
///
/// ```
/// use chess::bitboard::Bitboard;
/// let bb = Bitboard(0xFFFF);
/// let expected = (0..16).collect::<Vec<usize>>();
/// assert_eq!(expected, bb.iter().collect::<Vec<usize>>());
/// ```
///
pub struct BitboardIter<'a>(&'a Bitboard, Bitboard);

impl<'a> BitboardIter<'a> {
    ///
    /// Simple function that constructs a new `BitboardIter`
    /// with a reference to a `Bitboard` to iterate over
    ///
    /// # Arguments
    ///
    /// * `bb` - a reference to a bitboard to iterate over
    ///
    pub fn new(bb: &'a Bitboard) -> Self {
        BitboardIter(bb, EMPTY.into())
    }
}

impl Iterator for BitboardIter<'_> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        // copy of self.0 with bits that haven't been processed yet
        // TODO: investigate whether we can do away with this copy
        let tmp = *self.0 ^ self.1;
        if tmp != 0.into() {
            // get the next bit to process
            let lsb = tmp.lsb();

            // set that bit within the stored state
            self.1 ^= BIT_TABLE[lsb];
            Some(lsb)
        } else {
            None
        }
    }
}

#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum Rank {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
}

#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum File {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

///
/// A type to represent a board position in human-readable terms
///
#[derive(Debug, Copy, Clone)]
pub struct Location {
    pub rank: Rank,
    pub file: File,
}

impl Into<Bitboard> for Location {
    fn into(self) -> Bitboard {
        BIT_TABLE[((self.rank as u8 * 8) + self.file as u8) as usize]
    }
}

impl Into<Location> for (Rank, File) {
    fn into(self) -> Location {
        Location {
            rank: self.0.into(),
            file: self.1.into(),
        }
    }
}

///
/// Lazy static evaluation to create a table of all bitboards
/// that contain a single 'on' bit.
///
/// The resultant array can be quickly indexed for operations
/// that require activating specific bits on the bitboard
///
fn calculate_bit_table() -> [Bitboard; 64] {
    let mut bits: [Bitboard; 64] = [Bitboard(0); 64];
    for i in 0..64u64 {
        bits[i as usize] = Bitboard(1 << i);
    }
    bits
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add_bitboards() {
        let (a, b) = (Bitboard(1), Bitboard(2));
        assert_eq!(Bitboard(3), a + b);
    }

    #[test]
    fn test_xor_bitboards() {
        let (a, b) = (Bitboard(1), Bitboard(2));
        assert_eq!(Bitboard(3), a ^ b);

        let (a, b) = (Bitboard(123), Bitboard(456));
        assert_eq!(Bitboard(435), a ^ b);
    }

    #[test]
    fn test_lsb_bitboard() {
        assert_eq!(0, Bitboard(1).lsb());
        assert_eq!(1, Bitboard(2).lsb());
        assert_eq!(2, Bitboard(4).lsb());
        assert_eq!(3, Bitboard(8).lsb());
        assert_eq!(4, Bitboard(16).lsb());
        assert_eq!(5, Bitboard(32).lsb());
    }

    #[test]
    fn test_bitboard_iter() {
        let bb = Bitboard(0xFFFF);
        let expected = (0..16).collect::<Vec<usize>>();
        assert_eq!(expected, bb.iter().collect::<Vec<usize>>());

        let bb = Bitboard(0b10101010);
        let expected: [usize; 4] = [1, 3, 5, 7];
        let expected = Vec::from(expected);
        assert_eq!(expected, bb.iter().collect::<Vec<usize>>());
    }
}
