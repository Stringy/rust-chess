use crate::bitboard::{Bitboard, EMPTY, UNIVERSE};
use crate::board::Piece::*;
use crate::board::{Board, Colour, Piece, Rank};
use crate::masks::a1h8::*;
use crate::masks::a8h1::*;
use crate::masks::files::*;
use crate::masks::ranks::RANK_MASKS;
use crate::masks::sliding::{FILE_ATTACKS, RANK_ATTACKS};
use crate::masks::special::KNIGHT_ATTACKS;
use crate::masks::{pawns, RANK_SHIFT};
use crate::moves::{Move, MoveBuilder};

pub struct MoveGenerator<'a> {
    board: &'a Board,
    target: Bitboard,
    free: Bitboard,
}

impl<'a> MoveGenerator<'a> {
    pub fn new(board: &'a Board) -> Self {
        MoveGenerator {
            board,
            free: UNIVERSE & board.occupied,
            target: match board.next_move {
                Colour::Black => UNIVERSE & !board.black_pieces,
                Colour::White => UNIVERSE & !board.white_pieces,
            },
        }
    }

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
    /// let generator = MoveGenerator::new(&board);
    /// let moves = generator.all();
    /// assert_eq!(moves.len(), 36);
    /// ```
    ///
    pub fn all(&self) -> Vec<Move> {
        match self.board.next_move {
            Colour::Black => {
                let mut moves = self.black_pawns();
                let mut knights = self.knights();
                let mut bishops = self.bishops();
                let mut rooks = self.rooks();
                let mut queens = self.queens();

                moves.append(&mut knights);
                moves.append(&mut bishops);
                moves.append(&mut rooks);
                moves.append(&mut queens);

                moves
            }
            Colour::White => {
                let mut moves = self.white_pawns();
                let mut knights = self.knights();
                let mut bishops = self.bishops();
                let mut rooks = self.rooks();
                let mut queens = self.queens();

                moves.append(&mut knights);
                moves.append(&mut bishops);
                moves.append(&mut rooks);
                moves.append(&mut queens);

                moves
            }
        }
    }

    ///
    /// Generates all the white pawn moves for the current
    /// board.
    ///
    pub fn white_pawns(&self) -> Vec<Move> {
        let pawns = self.board.white_pawns;
        let free_squares = UNIVERSE & !self.board.occupied;
        let mut all_moves = Vec::new();

        pawns.iter().for_each(|from| {
            // Calculate the bitmask for all the moves for this given index
            let mut moves = pawns::W_PAWN_SINGLE_MOVES[from] & free_squares;

            if Rank::from(from) == Rank::Two && moves != 0.into() {
                // If we're on Rank 7 then we have to be in a position
                // to double move, because we can't move backwards
                moves |= pawns::W_PAWN_DOUBLE_MOVES[from] & free_squares;
            }
            moves |= pawns::W_PAWN_ATTACKS[from] & self.board.black_pieces;

            for to in moves.iter() {
                let mut builder = MoveBuilder::new();
                let builder = builder
                    .piece(WhitePawn)
                    .from(from)
                    .to(to)
                    .capture(self.board.pieces[to]);

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
    pub fn black_pawns(&self) -> Vec<Move> {
        let pawns = self.board.black_pawns;
        let free_squares = UNIVERSE & !self.board.occupied;
        let mut all_moves = Vec::new();

        pawns.iter().for_each(|from| {
            // Calcualte the bitmask for all the moves for this given index
            let mut moves = pawns::B_PAWN_SINGLE_MOVES[from] & free_squares;

            if Rank::from(from) == Rank::Seven && moves != EMPTY {
                // If we're on Rank 7 then we have to be in a position
                // to double move, because we can't move backwards
                moves |= pawns::B_PAWN_DOUBLE_MOVES[from] & free_squares;
            }

            moves |= pawns::B_PAWN_ATTACKS[from] & self.board.white_pieces;

            for to in moves.iter() {
                let mut builder = MoveBuilder::new();
                let builder = builder
                    .piece(BlackPawn)
                    .from(from)
                    .to(to)
                    .capture(self.board.pieces[to]);

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
    /// Generates all knight moves for the current board state
    ///
    pub fn knights(&self) -> Vec<Move> {
        let (piece, pieces) = match self.board.next_move {
            Colour::White => (WhiteKnight, self.board.white_knights),
            Colour::Black => (BlackKnight, self.board.black_knights),
        };

        self.generate_moves(piece, pieces, MoveGenerator::knight_gen)
    }

    ///
    /// Generates all bishop moves for the current board state
    ///
    pub fn bishops(&self) -> Vec<Move> {
        let (piece, pieces) = match self.board.next_move {
            Colour::White => (WhiteBishop, self.board.white_bishops),
            Colour::Black => (BlackBishop, self.board.black_bishops),
        };

        self.generate_moves(piece, pieces, MoveGenerator::bishop_gen)
    }

    ///
    /// Generates all rooks moves for the current board state
    ///
    pub fn rooks(&self) -> Vec<Move> {
        let (piece, pieces) = match self.board.next_move {
            Colour::White => (WhiteRook, self.board.white_rooks),
            Colour::Black => (BlackRook, self.board.black_rooks),
        };

        self.generate_moves(piece, pieces, MoveGenerator::rook_gen)
    }

    ///
    /// Generates all queens moves for the current board state
    ///
    pub fn queens(&self) -> Vec<Move> {
        let (piece, pieces) = match self.board.next_move {
            Colour::White => (WhiteQueen, self.board.white_queens),
            Colour::Black => (BlackQueen, self.board.black_queens),
        };

        self.generate_moves(piece, pieces, MoveGenerator::queen_gen)
    }

    ///
    /// Generic function for iterating over a bitboard, generating
    /// move bitboards, and creating moves
    ///
    fn generate_moves(
        &self,
        piece: Piece,
        pieces: Bitboard,
        generator: fn(&MoveGenerator, usize) -> Bitboard,
    ) -> Vec<Move> {
        let mut all_moves = Vec::new();

        pieces.iter().for_each(|from| {
            let moves = generator(self, from);
            moves.iter().for_each(|to| {
                let mut builder = MoveBuilder::new();
                let mov = builder
                    .piece(piece)
                    .from(from)
                    .to(to)
                    .capture(self.board.pieces[to])
                    .build();
                all_moves.push(mov);
            });
        });

        all_moves
    }

    ///
    /// Generates all knight moves for a generic position on the board
    ///
    fn knight_gen(gen: &MoveGenerator, from: usize) -> Bitboard {
        KNIGHT_ATTACKS[from] & gen.target
    }

    fn bishop_gen(mg: &MoveGenerator, from: usize) -> Bitboard {
        MoveGenerator::a1h8_gen(mg, from) | MoveGenerator::a8h1_gen(mg, from)
    }

    fn rook_gen(mg: &MoveGenerator, from: usize) -> Bitboard {
        MoveGenerator::rank_gen(mg, from) | MoveGenerator::file_gen(mg, from)
    }

    fn queen_gen(mg: &MoveGenerator, from: usize) -> Bitboard {
        MoveGenerator::rook_gen(mg, from) | MoveGenerator::bishop_gen(mg, from)
    }

    fn rank_gen(mg: &MoveGenerator, from: usize) -> Bitboard {
        let n: usize = ((mg.board.occupied & RANK_MASKS[from]) >> RANK_SHIFT[from]).0 as usize;
        RANK_ATTACKS[from][n] & mg.target
    }

    fn file_gen(mg: &MoveGenerator, from: usize) -> Bitboard {
        let mask = ((mg.board.occupied & FILE_MASKS[from]) * FILE_MAGIC[from]) >> 57;
        let n = mask.0 as usize;
        FILE_ATTACKS[from][n] & mg.target
    }

    fn a8h1_gen(mg: &MoveGenerator, from: usize) -> Bitboard {
        let mask = ((mg.board.occupied & A8H1_MASK[from]) * A8H1_MAGIC[from]) >> 57;
        let n = mask.0 as usize;
        A8H1_ATTACKS[from][n] & mg.target
    }

    fn a1h8_gen(mg: &MoveGenerator, from: usize) -> Bitboard {
        let n = (((mg.board.occupied & A1H8_MASK[from]) * A1H8_MAGIC[from]) >> 57).0 as usize;
        A1H8_ATTACKS[from][n] & mg.target
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::board::Layout;

    #[test]
    fn test_white_pawn_moves_classic_start() {
        let board = Board::new(Layout::Classic);
        let generator = MoveGenerator::new(&board);
        let moves = generator.white_pawns();
        // expect all 8 pawns moving single and double squares
        assert_eq!(moves.len(), 16);
    }

    #[test]
    fn test_black_pawn_moves_classic_start() {
        let board = Board::new(Layout::Classic);
        let generator = MoveGenerator::new(&board);
        let moves = generator.black_pawns();

        // expect all 8 pawns moving single and double squares
        assert_eq!(moves.len(), 16);
    }

    #[test]
    fn test_all() {
        let board = Board::new(Layout::Classic);
        let generator = MoveGenerator::new(&board);
        let moves = generator.all();
        assert_eq!(moves.len(), 20);
    }
}
