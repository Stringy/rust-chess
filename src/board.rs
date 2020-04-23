use crate::bitboard::Bitboard;

use derive_more::BitOr;

#[derive(Debug)]
pub enum Colour {
    Black,
    White,
}

#[derive(Debug, BitOr)]
pub enum Castle {
    OO = 1,
    OOO = 2,
}

///
/// Representation of each piece on the chess board,
/// including an empty square.
///
/// The format is as follows:
///
/// `[ 1 bit colour ][ 1 bit sliding ][ 2 bit designation ]`
///
#[repr(u8)]
pub enum Piece {
    Empty = 0,
    WhitePawn = 0b0001,
    WhiteKing = 0b0010,
    WhiteKnight = 0b0011,
    WhiteBishop = 0b0101,
    WhiteRook = 0b0110,
    WhiteQueen = 0b0111,

    BlackPawn = 0b1001,
    BlackKing = 0b1010,
    BlackKnight = 0b1011,
    BlackBishop = 0b1101,
    BlackRook = 0b1110,
    BlackQueen = 0b1111,
}

impl Piece {
    // TODO: add conversion between Piece and string repr
}

#[derive(Default, Debug)]
pub struct Board {
    pub white_pieces: Bitboard,
    pub black_pieces: Bitboard,

    pub occupied: Bitboard,

    pub white_pawns: Bitboard,
    pub white_rooks: Bitboard,
    pub white_knights: Bitboard,
    pub white_bishops: Bitboard,
    pub white_queens: Bitboard,
    pub white_king: Bitboard,

    pub black_pawns: Bitboard,
    pub black_rooks: Bitboard,
    pub black_knights: Bitboard,
    pub black_bishops: Bitboard,
    pub black_queens: Bitboard,
    pub black_king: Bitboard,

    pub next_move: Colour,
    pub white_castle: Castle,
    pub black_castle: Castle,

    pub move_count: u16,

    pub white_pawns_count: u8,
    pub white_rooks_count: u8,
    pub white_knights_count: u8,
    pub white_bishops_count: u8,
    pub white_queens_count: u8,
    pub white_king_count: u8,

    pub black_pawns_count: u8,
    pub black_rooks_count: u8,
    pub black_knights_count: u8,
    pub black_bishops_count: u8,
    pub black_queens_count: u8,
    pub black_king_count: u8,

    pub white_material: i32,
    pub black_material: i32,
}

#[derive(Debug)]
pub enum Layout {
    Classic,
}

impl Board {
    // pub fn new(layout: Layout) -> Self {
    //     match layout {
    //         Layout::Classic => new_classic(),
    //     }
    // }

    // fn new_classic() -> Self {
    //     Board {
    //         white_pieces: Bitboard(),
    //         black_pieces: Bitboard(),
    //         occupied: Bitboard(),
    //         white_pawns: Bitboard(),
    //         white_rooks: Bitboard(),
    //         white_knights: Bitboard(),
    //         white_bishops: Bitboard(),
    //         white_queens: Bitboard(),
    //         white_king: Bitboard(),
    //         black_pawns: Bitboard(),
    //         black_rooks: Bitboard(),
    //         black_knights: Bitboard(),
    //         black_bishops: Bitboard(),
    //         black_queens: Bitboard(),
    //         black_king: Bitboard(),
    //
    //         next_move: Colour::White,
    //
    //         white_castle: Castle::OO | Castle::OOO,
    //         black_castle: Castle::OO | Castle::OOO,
    //
    //         move_count: 0,
    //         white_pawns_count: 0,
    //         white_rooks_count: 0,
    //         white_knights_count: 0,
    //         white_bishops_count: 0,
    //         white_queens_count: 0,
    //         white_king_count: 0,
    //         black_pawns_count: 0,
    //         black_rooks_count: 0,
    //         black_knights_count: 0,
    //         black_bishops_count: 0,
    //         black_queens_count: 0,
    //         black_king_count: 0,
    //         white_material: 0,
    //         black_material: 0,
    //     }
    // }
}
