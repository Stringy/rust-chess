use chess::bitboard::Bitboard;
use chess::board::Piece::*;
use chess::board::{Board, Layout, Rank};
use chess::board_idx;
use chess::builder::BitboardBuilder;
use chess::generate::MoveGenerator;
use chess::moves::Move;

#[test]
fn test_white_pawns_promotion() {
    let mut board = Board::empty();
    board.white_pawns = BitboardBuilder::new().rank(Rank::Seven, 0b0001000).board();
    board.occupied = board.white_pieces;

    let mg = MoveGenerator::new(&board);
    let moves = mg.white_pawns();
    // expect one promotion for each of Queen, Knight, Bishop, and Rook
    #[rustfmt::skip]
    let expected = [
        Move::new(WhitePawn, board_idx!(6, 3), board_idx!(7, 3), None, Some(WhiteQueen)),
        Move::new(WhitePawn, board_idx!(6, 3), board_idx!(7, 3), None, Some(WhiteKnight)),
        Move::new(WhitePawn, board_idx!(6, 3), board_idx!(7, 3), None, Some(WhiteBishop)),
        Move::new(WhitePawn, board_idx!(6, 3), board_idx!(7, 3), None, Some(WhiteRook)),
    ];

    moves.iter().for_each(|n| println!("{:?}", n));
    println!();
    expected.iter().for_each(|n| println!("{:?}", n));

    assert_eq!(moves.len(), 4);
}

#[test]
fn test_white_pawns_enpassant() {}

#[test]
fn test_white_pawns_capture() {}

#[test]
fn test_white_pawns_starting_moves() {}
