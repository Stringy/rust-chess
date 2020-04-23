use crate::bitboard::Bitboard;
use crate::board::Piece;

pub struct BitboardBuilder {
    bitboard: Bitboard,
}

impl BitboardBuilder {
    pub fn new() -> BitboardBuilder {
        BitboardBuilder {
            bitboard: Bitboard(0),
        }
    }

    pub fn rank<'a>(&'a mut self, pieces: &[Piece]) -> &'a mut BitboardBuilder {
        self
    }

    pub fn square<T: Into<Location> + 'a>(
        &'a mut self,
        location: T,
        piece: Piece,
    ) -> &'a mut BitboardBuilder {
        self
    }

    pub fn build(&self) -> Bitboard {
        self.bitboard
    }
}
