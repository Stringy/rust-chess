use crate::bitboard::{Bitboard, UNIVERSE};
use crate::board::Piece::*;
use crate::board::{Board, Colour, Piece, Rank};
use crate::masks::pawns;
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
    /// assert_eq!(36, moves.len())
    /// ```
    ///
    pub fn all(board: &Board) -> Vec<Move> {
        match board.next_move {
            Colour::Black => MoveGenerator::black_pawn_moves(board),
            Colour::White => MoveGenerator::white_pawn_moves(board),
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

            if Rank::from(from) == Rank::Seven && moves != 0.into() {
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
}
