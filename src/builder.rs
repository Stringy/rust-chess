use crate::bitboard::Bitboard;
use crate::board::{Location, Rank};

#[derive(Default)]
pub struct BitboardBuilder {
    bitboard: Bitboard,
}

impl BitboardBuilder {
    pub fn new() -> BitboardBuilder {
        BitboardBuilder::default()
    }

    ///
    /// Sets a particular Rank in the bitboard, replacing it with the specific
    /// pieces
    ///
    /// # Arguments
    ///
    /// * `rank` the rank to set within the bitboard
    /// * `bits` a bit mask that represents the bits to turn on for the rank
    ///
    /// # Example
    ///
    /// ```
    /// use chess::bitboard::Bitboard;
    /// use chess::board::Rank;
    /// use chess::builder::BitboardBuilder;
    ///
    /// let bitboard = BitboardBuilder::new()
    ///                 .rank(Rank::One, 0b11110011)
    ///                 .rank(Rank::Two, 0b11111111)
    ///                 .board();
    ///
    /// assert_eq!(bitboard, Bitboard(0xFFF3));
    /// ```
    ///
    pub fn rank(&mut self, rank: Rank, bits: u8) -> &mut BitboardBuilder {
        // calculate mask at the correct bit offset for the given rank
        // (least significant bits => lower rank)
        let mask = (bits as u64) << ((rank as u64) * 8);

        // zero the bits of that rank, whilst retaining all other bits
        self.bitboard.0 &= (0xFFu64 << ((rank as u64) * 8)) | self.bitboard.0;

        // apply mask
        self.bitboard.0 |= mask;
        self
    }

    pub fn file<'a>(&'a mut self, _bits: u8) -> &'a mut BitboardBuilder {
        // TODO: is this even useful?
        unimplemented!();
    }

    ///
    /// Toggles a specific square on the bitboard
    ///
    /// # Arguments
    ///
    /// * `location` the location on the board
    ///
    /// # Examples
    ///
    /// ```
    /// use chess::bitboard::Bitboard;
    /// use chess::builder::BitboardBuilder;
    /// use chess::board::{Rank, File};
    ///
    /// let bitboard = BitboardBuilder::new()
    ///                 .square((Rank::One, File::A))
    ///                 .square((Rank::One, File::H))
    ///                 .board();
    ///
    /// assert_eq!(bitboard, Bitboard(0x81));
    /// ```
    ///
    pub fn square<T: Into<Location>>(&mut self, location: T) -> &mut BitboardBuilder {
        self.bitboard ^= location.into().into();
        self
    }

    ///
    /// Simply retrieves the internal board representation
    ///
    pub fn board(&self) -> Bitboard {
        self.bitboard
    }
}
