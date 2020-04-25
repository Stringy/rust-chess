use derive_more::BitOr;

use crate::bitboard::{Bitboard, Rank};
use crate::builder::BitboardBuilder;

const CLASSIC_WHITE_PAWNS: Bitboard   = Bitboard(0xFF00);
const CLASSIC_WHITE_ROOKS: Bitboard   = Bitboard(0x81);
const CLASSIC_WHITE_KNIGHTS: Bitboard = Bitboard(0x42);
const CLASSIC_WHITE_BISHOPS: Bitboard = Bitboard(0x24);
const CLASSIC_WHITE_QUEEN: Bitboard   = Bitboard(0x10);
const CLASSIC_WHITE_KING: Bitboard    = Bitboard(0x8);
const CLASSIC_BLACK_PAWNS: Bitboard   = Bitboard(0xFF000000000000);
const CLASSIC_BLACK_ROOKS: Bitboard   = Bitboard(0x8100000000000000);
const CLASSIC_BLACK_KNIGHTS: Bitboard = Bitboard(0x4200000000000000);
const CLASSIC_BLACK_BISHOPS: Bitboard = Bitboard(0x2400000000000000);
const CLASSIC_BLACK_QUEEN: Bitboard   = Bitboard(0x800000000000000);
const CLASSIC_BLACK_KING: Bitboard    = Bitboard(0x1000000000000000);

#[derive(Debug)]
pub enum Colour {
    Black,
    White,
}

impl Default for Colour {
    fn default() -> Self {
        Colour::White
    }
}

#[derive(Debug, BitOr)]
pub enum Castle {
    OO = 1,
    OOO = 2,
    OOAndOOO = 3
}

impl Default for Castle {
    fn default() -> Self {
        Castle::OOAndOOO
    }
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

#[derive(Debug)]
pub enum PieceValue {
    Pawn = 100,
    Rook = 500,
    KnightOrBishop = 300,
    Queen = 900,
    King = 9999
}

impl PieceValue {
    pub fn classic_starting() -> i32 {
        (PieceValue::Pawn as i32 * 8) +
            (PieceValue::Rook as i32 * 2) +
            (PieceValue::KnightOrBishop as i32 * 4) +
            (PieceValue::Queen as i32)
    }
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
    ///
    /// Constructs a new Board, based on a given layout.
    ///
    pub fn new(layout: Layout) -> Self {
        match layout {
            Layout::Classic => Board::new_classic(),
        }
    }

    ///
    /// Constructs a chess board based on the classic layout
    ///
    fn new_classic() -> Self {
        let white_pieces = BitboardBuilder::new()
            .rank(Rank::One, 0xFF)
            .rank(Rank::Two, 0xFF)
            .board();
        let black_pieces = BitboardBuilder::new()
            .rank(Rank::Seven, 0xFF)
            .rank(Rank::Eight, 0xFF)
            .board();

        Board {
            white_pieces,
            black_pieces,
            occupied: white_pieces | black_pieces,

            white_pawns: CLASSIC_WHITE_PAWNS,
            white_rooks: CLASSIC_WHITE_ROOKS,
            white_knights: CLASSIC_WHITE_KNIGHTS,
            white_bishops: CLASSIC_WHITE_BISHOPS,
            white_queens: CLASSIC_WHITE_QUEEN,
            white_king: CLASSIC_WHITE_KING,

            black_pawns: CLASSIC_BLACK_PAWNS,
            black_rooks: CLASSIC_BLACK_ROOKS,
            black_knights: CLASSIC_BLACK_KNIGHTS,
            black_bishops: CLASSIC_BLACK_BISHOPS,
            black_queens: CLASSIC_BLACK_QUEEN,
            black_king: CLASSIC_BLACK_KING,

            next_move: Colour::default(),

            white_castle: Castle::default(),
            black_castle: Castle::default(),

            move_count: 0,
            white_pawns_count: 8,
            white_rooks_count: 2,
            white_knights_count: 2,
            white_bishops_count: 2,
            white_queens_count: 1,
            white_king_count: 1,
            black_pawns_count: 8,
            black_rooks_count: 2,
            black_knights_count: 2,
            black_bishops_count: 2,
            black_queens_count: 1,
            black_king_count: 1,

            white_material: PieceValue::classic_starting(),
            black_material: -PieceValue::classic_starting(),
        }
    }
}
