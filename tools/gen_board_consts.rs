extern crate chess;

use chess::bitboard::Bitboard;

fn main() {
    let white_pawns = Bitboard(0x00FF000000000000);
    let black_pawns = Bitboard(0x000000000000FF00);
}
