use crate::board::Piece;
use derive_more::{BitAnd, BitAndAssign, BitOr, BitOrAssign};
use std::fmt;
use std::fmt::{Debug, Formatter};

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
#[derive(Copy, Clone, BitAnd, BitAndAssign, BitOr, BitOrAssign)]
pub struct Move(pub u32);

impl Debug for Move {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "(piece: {:?}, to: {}, from: {}, cap: {:?}, promo: {:?})",
            self.piece(),
            self.to(),
            self.from(),
            self.capture(),
            self.promotion()
        )
    }
}

impl Move {
    pub fn new(
        piece: Piece,
        from: usize,
        to: usize,
        capture: Option<Piece>,
        promo: Option<Piece>,
    ) -> Self {
        MoveBuilder::new()
            .piece(piece)
            .from(from)
            .to(to)
            .capture(capture.unwrap_or(Piece::Empty))
            .promote(promo.unwrap_or(Piece::Empty))
            .build()
    }
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
        ((self.0 & 0x00000fc0) >> 6) as usize
    }

    ///
    /// Retrieves the moving piece from the move
    ///
    pub fn piece(&self) -> Piece {
        (((self.0 & 0x0000f000) >> 12) as u8).into()
    }

    ///
    /// Retrieves the captured piece from the move
    ///
    pub fn capture(&self) -> Option<Piece> {
        let n = ((self.0 & 0x000f0000) >> 16) as u8;
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
        let n = ((self.0 & 0x00f00000) >> 20) as u8;
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
    pub fn new() -> Self {
        MoveBuilder(Move(0))
    }
    ///
    /// Returns the built move
    ///
    pub fn build(&self) -> Move {
        self.0
    }

    ///
    /// Sets the from index in the move
    ///
    pub fn from(&mut self, f: usize) -> &mut MoveBuilder {
        self.0 &= Move(0xffffffc0);
        self.0 |= Move(f as u32 & 0x0000003f);
        self
    }

    ///
    /// Sets the to index in the move
    ///
    pub fn to(&mut self, t: usize) -> &mut MoveBuilder {
        self.0 &= Move(0xfffff03f);
        self.0 |= Move(((t as u32) << 6) & 0x00000fc0);
        self
    }

    ///
    /// Sets the moving piece in the move
    ///
    pub fn piece(&mut self, p: Piece) -> &mut MoveBuilder {
        self.0 &= Move(0xffff0fff);
        self.0 |= Move(((p as u32) << 12) & 0x0000f000);
        self
    }

    ///
    /// Sets the captured piece in the move
    ///
    pub fn capture(&mut self, p: Piece) -> &mut MoveBuilder {
        self.0 &= Move(0xfff0ffff);
        self.0 |= Move(((p as u32) << 16) & 0x000f0000);
        self
    }

    ///
    /// Sets the promotion piece in the move
    ///
    pub fn promote(&mut self, p: Piece) -> &mut MoveBuilder {
        self.0 &= Move(0xff0fffff);
        self.0 |= Move(((p as u32) << 20) & 0x00f00000);
        self
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_move_construction() {
        let mov = Move::new(
            Piece::WhiteKnight,
            10,
            20,
            Some(Piece::BlackBishop),
            Some(Piece::WhiteQueen),
        );

        assert_eq!(mov.piece(), Piece::WhiteKnight);
        assert_eq!(mov.from(), 10);
        assert_eq!(mov.to(), 20);
        assert_eq!(mov.capture().unwrap_or(Piece::Empty), Piece::BlackBishop);
        assert_eq!(mov.promotion().unwrap_or(Piece::Empty), Piece::WhiteQueen);
    }

    #[test]
    fn test_move_builder_promotion() {
        let m = MoveBuilder::new().promote(Piece::WhiteKnight).build();
        assert!(m.is_promotion());
        assert_eq!(m.promotion().unwrap_or(Piece::Empty), Piece::WhiteKnight);
    }

    #[test]
    fn test_move_builder_capture() {
        let m = MoveBuilder::new().capture(Piece::WhiteKnight).build();
        assert!(m.is_capture());
        assert_eq!(m.capture().unwrap_or(Piece::Empty), Piece::WhiteKnight);
    }
}
