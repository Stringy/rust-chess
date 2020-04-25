use crate::board::Piece;
use derive_more::{BitAnd, BitAndAssign, BitOr, BitOrAssign};

///
/// A move structure encapsulates information about a given move
/// on the chess board.
///
/// From least to most significant bits:
/// * 6 bits from index (0..63)
/// * 6 bits to index (0..63)
/// * 4 bits moving piece
/// * 4 bits captured piece
/// * 4 bits promotion piece
///
#[derive(Debug, Copy, Clone, BitAnd, BitAndAssign, BitOr, BitOrAssign)]
pub struct Move(u32);

impl Move {
    ///
    /// Retrieves the from index from the move.
    /// Returned as a usize to allow for easier indexing
    ///
    pub fn from(&self) -> usize {
        (self.0 & 0x0000003f) as usize
    }

    ///
    /// Retrieves the to index from the move.
    /// Returned as a usize to allow for easier indexing
    ///
    pub fn to(&self) -> usize {
        (self.0 & 0x00000fc0) as usize
    }

    ///
    /// Retrieves the moving piece from the move
    ///
    pub fn piece(&self) -> Piece {
        ((self.0 & 0x0000f000) as u8).into()
    }

    ///
    /// Retrieves the captured piece from the move
    ///
    pub fn capture(&self) -> Option<Piece> {
        let n = (self.0 & 0x000f0000) as u8;
        if n == 0 {
            None
        } else {
            Some(n.into())
        }
    }

    ///
    /// Retrieves the promoted piece from the move
    ///
    pub fn promotion(&self) -> Option<Piece> {
        let n = (self.0 & 0x00f00000) as u8;
        if n == 0 {
            None
        } else {
            Some(n.into())
        }
    }

    ///
    /// Checks whether the moving piece is white
    ///
    pub fn is_white(&self) -> bool {
        (self.0 & 0x0000e000) == 0
    }

    ///
    /// Checks whether the moving piece is black
    ///
    pub fn is_black(&self) -> bool {
        (self.0 & 0x0000e000) != 0
    }

    ///
    /// Checks whether there is a capturing piece
    ///
    pub fn is_capture(&self) -> bool {
        self.capture().is_some()
    }

    ///
    /// Checks whether there is a promotion piece
    ///
    pub fn is_promotion(&self) -> bool {
        self.promotion().is_some()
    }
}

#[derive(Debug)]
pub struct MoveBuilder(Move);

impl MoveBuilder {
    ///
    /// Returns the built move
    ///
    pub fn build(&self) -> Move {
        self.0
    }

    ///
    /// Sets the from index in the move
    ///
    pub fn from(&mut self, f: u8) -> &mut MoveBuilder {
        self.0 &= Move(0xffffffc0);
        self.0 |= Move(f as u32 & 0x0000003f);
        self
    }

    ///
    /// Sets the to index in the move
    ///
    pub fn to(&mut self, t: u8) -> &mut MoveBuilder {
        self.0 &= Move(0xfffff03f);
        self.0 |= Move((t as u32 & 0x00000fc0) << 6);
        self
    }

    ///
    /// Sets the moving piece in the move
    ///
    pub fn piece(&mut self, p: Piece) -> &mut MoveBuilder {
        self.0 &= Move(0xffff0fff);
        self.0 |= Move(p as u32 & 0x0000f000);
        self
    }

    ///
    /// Sets the captured piece in the move
    ///
    pub fn capture(&mut self, p: Piece) -> &mut MoveBuilder {
        self.0 &= Move(0xfff0ffff);
        self.0 |= Move(p as u32 & 0x000f0000);
        self
    }

    ///
    /// Sets the promotion piece in the move
    ///
    pub fn promote(&mut self, p: Piece) -> &mut MoveBuilder {
        self.0 &= Move(0xff0fffff);
        self.0 |= Move(p as u32 & 0x00f00000);
        self
    }
}
