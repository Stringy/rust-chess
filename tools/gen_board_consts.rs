extern crate chess;

use chess::bitboard::{Rank};
use chess::builder::BitboardBuilder;

fn main() {
    let white_pawns = BitboardBuilder::new().rank(Rank::Two, 0xFF).board();
    let white_rooks = BitboardBuilder::new().rank(Rank::One, 0x81).board();
    let white_knights = BitboardBuilder::new().rank(Rank::One, 0x42).board();
    let white_bishops = BitboardBuilder::new().rank(Rank::One, 0x24).board();
    let white_queen = BitboardBuilder::new().rank(Rank::One, 0x10).board();
    let white_king = BitboardBuilder::new().rank(Rank::One, 0x08).board();

    let black_pawns = BitboardBuilder::new().rank(Rank::Seven, 0xFF).board();
    let black_rooks = BitboardBuilder::new().rank(Rank::Eight, 0x81).board();
    let black_knights = BitboardBuilder::new().rank(Rank::Eight, 0x42).board();
    let black_bishops = BitboardBuilder::new().rank(Rank::Eight, 0x24).board();
    let black_queen = BitboardBuilder::new().rank(Rank::Eight, 0x08).board();
    let black_king = BitboardBuilder::new().rank(Rank::Eight, 0x10).board();

    println!("const CLASSIC_WHITE_PAWNS: Bitboard   = {:X?};", white_pawns);
    println!("const CLASSIC_WHITE_ROOKS: Bitboard   = {:X?};", white_rooks);
    println!("const CLASSIC_WHITE_KNIGHTS: Bitboard = {:X?};", white_knights);
    println!("const CLASSIC_WHITE_BISHOPS: Bitboard = {:X?};", white_bishops);
    println!("const CLASSIC_WHITE_QUEEN: Bitboard   = {:X?};", white_queen);
    println!("const CLASSIC_WHITE_KING: Bitboard    = {:X?};", white_king);

    println!("const CLASSIC_BLACK_PAWNS: Bitboard   = {:X?};", black_pawns);
    println!("const CLASSIC_BLACK_ROOKS: Bitboard   = {:X?};", black_rooks);
    println!("const CLASSIC_BLACK_KNIGHTS: Bitboard = {:X?};", black_knights);
    println!("const CLASSIC_BLACK_BISHOPS: Bitboard = {:X?};", black_bishops);
    println!("const CLASSIC_BLACK_QUEEN: Bitboard   = {:X?};", black_queen);
    println!("const CLASSIC_BLACK_KING: Bitboard    = {:X?};", black_king);
}
