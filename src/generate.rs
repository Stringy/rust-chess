use crate::bitboard::{Bitboard, EMPTY, UNIVERSE};
use crate::board::Piece::*;
use crate::board::{Board, Colour, Piece, Rank};
use crate::masks::pawns;
use crate::masks::special::KNIGHT_ATTACKS;
use crate::moves::{Move, MoveBuilder};

pub struct MoveGenerator;

impl MoveGenerator {
    ///
    /// Generates all moves for the current turn of the game
    /// either black or white
    ///
    /// # Example
    ///
    /// ```
    /// use chess::board::{Layout, Board};
    /// use chess::generate::MoveGenerator;
    /// let board = Board::new(Layout::Classic);
    /// let moves = MoveGenerator::all(&board);
    /// assert_eq!(moves.len(), 36);
    /// ```
    ///
    pub fn all(board: &Board) -> Vec<Move> {
        match board.next_move {
            Colour::Black => {
                let mut moves = MoveGenerator::black_pawn_moves(board);
                let target = UNIVERSE & !board.black_pieces;
                let mut knights =
                    MoveGenerator::knight_moves(board, board.black_knights, BlackKnight, target);

                moves.append(&mut knights);
                moves
            }
            Colour::White => {
                let mut moves = MoveGenerator::white_pawn_moves(board);
                let target = UNIVERSE & !board.white_pieces;
                let mut knights =
                    MoveGenerator::knight_moves(board, board.white_knights, WhiteKnight, target);

                moves.append(&mut knights);
                moves
            }
        }
    }

    ///
    /// Generates all the white pawn moves for the current
    /// board.
    ///
    pub fn white_pawn_moves(board: &Board) -> Vec<Move> {
        let pawns = board.white_pawns;
        let free_squares = UNIVERSE & !board.occupied;
        let mut all_moves = Vec::new();

        pawns.iter().for_each(|from| {
            // Calculate the bitmask for all the moves for this given index
            let mut moves = pawns::W_PAWN_SINGLE_MOVES[from] & free_squares;

            if Rank::from(from) == Rank::Two && moves != 0.into() {
                // If we're on Rank 7 then we have to be in a position
                // to double move, because we can't move backwards
                moves |= pawns::W_PAWN_DOUBLE_MOVES[from] & free_squares;
            }
            moves |= pawns::W_PAWN_ATTACKS[from] & board.black_pieces;

            for to in moves.iter() {
                let mut builder = MoveBuilder::new();
                let builder = builder
                    .piece(WhitePawn)
                    .from(from)
                    .to(to)
                    .capture(board.pieces[to]);

                if Rank::from(to) == Rank::Eight {
                    // promotion. Cover all possible pieces that can be promoted to.
                    [WhiteQueen, WhiteKnight, WhiteBishop, WhiteRook]
                        .iter()
                        .for_each(|promo| {
                            all_moves.push(builder.promote(*promo).build());
                        })
                } else {
                    all_moves.push(builder.build());
                }
            }
        });

        all_moves
    }

    ///
    /// Generates all the black pawn moves for the current
    /// Board
    ///
    pub fn black_pawn_moves(board: &Board) -> Vec<Move> {
        let pawns = board.black_pawns;
        let free_squares = UNIVERSE & !board.occupied;
        let mut all_moves = Vec::new();

        pawns.iter().for_each(|from| {
            // Calcualte the bitmask for all the moves for this given index
            let mut moves = pawns::B_PAWN_SINGLE_MOVES[from] & free_squares;

            if Rank::from(from) == Rank::Seven && moves != EMPTY {
                // If we're on Rank 7 then we have to be in a position
                // to double move, because we can't move backwards
                moves |= pawns::B_PAWN_DOUBLE_MOVES[from] & free_squares;
            }

            moves |= pawns::B_PAWN_ATTACKS[from] & board.white_pieces;

            for to in moves.iter() {
                let mut builder = MoveBuilder::new();
                let builder = builder
                    .piece(BlackPawn)
                    .from(from)
                    .to(to)
                    .capture(board.pieces[to]);

                if Rank::from(to) == Rank::One {
                    // promotion. Cover all possible pieces that can be promoted to.
                    [BlackQueen, BlackKnight, BlackBishop, BlackRook]
                        .iter()
                        .for_each(|promo| {
                            all_moves.push(builder.promote(*promo).build());
                        })
                } else {
                    all_moves.push(builder.build());
                }
            }
        });

        all_moves
    }

    ///
    /// Generates all knight moves for the given colour on the board.
    ///
    pub fn knight_moves(
        board: &Board,
        pieces: Bitboard,
        piece: Piece,
        target: Bitboard,
    ) -> Vec<Move> {
        let mut all_moves = Vec::new();
        let mut builder = MoveBuilder::new();

        pieces.iter().for_each(|knight| {
            let builder = builder.piece(piece).from(knight);
            let moves = KNIGHT_ATTACKS[knight] & target;

            for mov in moves.iter() {
                let bb = builder.to(mov).capture(board.pieces[mov]).build();
                all_moves.push(bb)
            }
        });

        all_moves
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::board::Layout;

    #[test]
    fn test_white_pawn_moves_classic_start() {
        let board = Board::new(Layout::Classic);
        let moves = MoveGenerator::white_pawn_moves(&board);
        // expect all 8 pawns moving single and double squares
        assert_eq!(moves.len(), 16);
    }

    #[test]
    fn test_black_pawn_moves_classic_start() {
        let board = Board::new(Layout::Classic);
        let moves = MoveGenerator::black_pawn_moves(&board);

        // expect all 8 pawns moving single and double squares
        assert_eq!(moves.len(), 16);
    }

    #[test]
    fn test_all() {
        let board = Board::new(Layout::Classic);
        let moves = MoveGenerator::all(&board);
        assert_eq!(moves.len(), 20);
    }
}
