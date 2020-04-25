use crate::bitboard::{Bitboard, BIT_TABLE};
use crate::board_idx;
use lazy_static::lazy_static;

pub const DIAG_A8H1_MAGIC: [Bitboard; 15] = [
    Bitboard(0x0),
    Bitboard(0x0),
    Bitboard(0x0101010101010100),
    Bitboard(0x0101010101010100),
    Bitboard(0x0101010101010100),
    Bitboard(0x0101010101010100),
    Bitboard(0x0101010101010100),
    Bitboard(0x0101010101010100),
    Bitboard(0x0080808080808080),
    Bitboard(0x0040404040404040),
    Bitboard(0x0020202020202020),
    Bitboard(0x0010101010101010),
    Bitboard(0x0008080808080808),
    Bitboard(0x0),
    Bitboard(0x0),
];

pub const DIAG_A1H8_MAGIC: [Bitboard; 15] = [
    Bitboard(0x0),
    Bitboard(0x0),
    Bitboard(0x0101010101010100),
    Bitboard(0x0101010101010100),
    Bitboard(0x0101010101010100),
    Bitboard(0x0101010101010100),
    Bitboard(0x0101010101010100),
    Bitboard(0x0101010101010100),
    Bitboard(0x8080808080808000),
    Bitboard(0x4040404040400000),
    Bitboard(0x2020202020000000),
    Bitboard(0x1010101000000000),
    Bitboard(0x0808080000000000),
    Bitboard(0x0),
    Bitboard(0x0),
];

pub const FILE_MAGIC_MASKS: [Bitboard; 8] = [
    Bitboard(0x8040201008040200),
    Bitboard(0x4020100804020100),
    Bitboard(0x2010080402010080),
    Bitboard(0x1008040201008040),
    Bitboard(0x0804020100804020),
    Bitboard(0x0402010080402010),
    Bitboard(0x0201008040201008),
    Bitboard(0x0100804020100804),
];

#[rustfmt::skip]
pub const RANK_SHIFT: [usize; 64] = [
    1, 1, 1, 1, 1, 1, 1, 1,
    9, 9, 9, 9, 9, 9, 9, 9,
    17, 17, 17, 17, 17, 17, 17, 17,
    25, 25, 25, 25, 25, 25, 25, 25,
    33, 33, 33, 33, 33, 33, 33, 33,
    41, 41, 41, 41, 41, 41, 41, 41,
    49, 49, 49, 49, 49, 49, 49, 49,
    57, 57, 57, 57, 57, 57, 57, 57,
];

///
/// A type for encapsulating both the diagonal mask
/// and its magic constants
///
pub type DiagMask = ([Bitboard; 64], [Bitboard; 64]);

lazy_static! {
    pub static ref RANK_MASK: [Bitboard; 64] = calculate_rank_masks();
    pub static ref FILE_MASK: [Bitboard; 64] = calculate_file_masks();
    pub static ref A8H1_MASK: DiagMask = calculate_a8h1_masks();
    pub static ref A1H8_MASK: DiagMask = calculate_a1h8_masks();
    pub static ref FILE_MAGIC: [Bitboard; 64] = calculate_file_magic();
    pub static ref GEN_SLIDING: [[u8; 64]; 8] = calculate_general_sliding_attacks();
    pub static ref W_PAWN_MOVES: [Bitboard; 64] = calculate_white_pawn_single_moves();
    pub static ref W_PAWN_DBL_MOVES: [Bitboard; 64] = calculate_white_pawn_double_moves();
    pub static ref W_PAWN_ATTACKS: [Bitboard; 64] = calculate_white_pawn_attacks();
    pub static ref B_PAWN_MOVES: [Bitboard; 64] = calculate_black_pawn_single_moves();
    pub static ref B_PAWN_DBL_MOVES: [Bitboard; 64] = calculate_black_pawn_double_moves();
    pub static ref B_PAWN_ATTACKS: [Bitboard; 64] = calculate_black_pawn_attacks();
    pub static ref KNIGHT_MOVES: [Bitboard; 64] = calculate_knight_moves();
    pub static ref RANK_ATTACKS: [[Bitboard; 64]; 64] = calculate_rank_attacks();
    pub static ref FILE_ATTACKS: [[Bitboard; 64]; 64] = calculate_file_attacks();
    pub static ref KING_MOVES: [Bitboard; 64] = calculate_king_moves();
    pub static ref A8H1_MOVES: [[Bitboard; 64]; 64] = calculate_a8h1_attacks();
    pub static ref A1H8_MOVES: [[Bitboard; 64]; 64] = calculate_a1h8_attacks();
}

///
/// Calculates the rank mask for any index on the board
///
fn calculate_rank_masks() -> [Bitboard; 64] {
    let mut rank_mask = [Bitboard(0); 64];
    for file in 0..9 {
        for rank in 0..9 {
            rank_mask[board_idx!(rank, file)] = BIT_TABLE[board_idx!(rank, 2)]
                | BIT_TABLE[board_idx!(rank, 3)]
                | BIT_TABLE[board_idx!(rank, 4)]
                | BIT_TABLE[board_idx!(rank, 5)]
                | BIT_TABLE[board_idx!(rank, 6)]
                | BIT_TABLE[board_idx!(rank, 7)];
        }
    }

    rank_mask
}

///
/// Calculates the file mask for any index on the board
///
fn calculate_file_masks() -> [Bitboard; 64] {
    let mut file_masks = [Bitboard(0); 64];
    for file in 0..9 {
        for rank in 0..9 {
            file_masks[board_idx!(rank, file)] = BIT_TABLE[board_idx!(2, file)]
                | BIT_TABLE[board_idx!(3, file)]
                | BIT_TABLE[board_idx!(4, file)]
                | BIT_TABLE[board_idx!(5, file)]
                | BIT_TABLE[board_idx!(6, file)]
                | BIT_TABLE[board_idx!(7, file)];
        }
    }

    file_masks
}

///
/// Calculates the A8H1 diagonal masks
///
fn calculate_a8h1_masks() -> ([Bitboard; 64], [Bitboard; 64]) {
    let mut diag_masks = [Bitboard(0); 64];
    let mut diag_magic = [Bitboard(0); 64];
    for file in 0..9 {
        for rank in 0..9 {
            let diag = rank + file;
            diag_masks[board_idx!(rank, file)] = DIAG_A8H1_MAGIC[diag - 2];
            diag_magic[board_idx!(rank, file)] = 0.into();

            if diag < 10 {
                for sq in 2..diag {
                    diag_masks[board_idx!(rank, file)] |= BIT_TABLE[board_idx!(diag - sq, sq)];
                }
            } else {
                for sq in 2..diag {
                    diag_masks[board_idx!(rank, file)] |=
                        BIT_TABLE[board_idx!(9 - sq, diag + sq - 9)];
                }
            }
        }
    }

    (diag_masks, diag_magic)
}

///
/// Calculates the A1H8 diagonal masks
///
fn calculate_a1h8_masks() -> ([Bitboard; 64], [Bitboard; 64]) {
    let mut diag_masks = [Bitboard(0); 64];
    let mut diag_magic = [Bitboard(0); 64];
    for file in 0..9 {
        for rank in 0..9 {
            let diag = rank + file;
            diag_masks[board_idx!(rank, file)] = DIAG_A1H8_MAGIC[diag + 7];
            diag_magic[board_idx!(rank, file)] = 0.into();

            if diag < 10 {
                for sq in 2..9 - diag {
                    diag_masks[board_idx!(rank, file)] |= BIT_TABLE[board_idx!(sq, diag + sq)];
                }
            } else {
                for sq in 2..9 + diag {
                    diag_masks[board_idx!(rank, file)] |= BIT_TABLE[board_idx!(sq, sq - diag)];
                }
            }
        }
    }

    (diag_masks, diag_magic)
}

///
/// Calculates the file magic masks
///
fn calculate_file_magic() -> [Bitboard; 64] {
    let mut file_magic = [Bitboard(0); 64];

    for file in 0..9 {
        for rank in 0..9 {
            file_magic[board_idx!(rank, file)] = FILE_MAGIC_MASKS[file - 1];
        }
    }

    file_magic
}

///
/// Calculates masks for general sliding attacks on any position
/// on the board.
///
fn calculate_general_sliding_attacks() -> [[u8; 64]; 8] {
    let mut sliding_attacks = [[0u8; 64]; 8];

    for sq in 0..8 {
        for state6 in 0..64 {
            let state8 = state6 << 8;
            let mut attack = 0u8;

            if sq < 7 {
                attack |= 1 << (sq + 1);
            }

            for slide in sq + 2..8 {
                if ((1 << (slide - 1)) & !state8) != 0 {
                    attack |= 1 << slide;
                }
            }

            if sq > 0 {
                attack |= 1 << (sq - 1);
            }

            for slide in sq + 2..0 {
                if ((1 << (slide + 1)) & !state8) != 0 {
                    attack |= 1 << slide;
                }
            }

            sliding_attacks[sq][state6] = attack;
        }
    }

    sliding_attacks
}

///
/// Calculates all white pawn single space moves
///
fn calculate_white_pawn_single_moves() -> [Bitboard; 64] {
    let mut moves = [Bitboard(0); 64];

    for file in 0..8 {
        for rank in 0..8 {
            let mrank = rank + 1;
            if mrank != 8 {
                moves[board_idx!(rank, file)] |= BIT_TABLE[board_idx!(mrank, file)];
            }
        }
    }

    moves
}

///
/// Calculates all white pawn double space moves
///
fn calculate_white_pawn_double_moves() -> [Bitboard; 64] {
    let mut moves = [Bitboard(0); 64];

    for file in 0..8 {
        let mrank = 3;
        moves[board_idx!(1, file)] |= BIT_TABLE[board_idx!(mrank, file)];
    }

    moves
}

///
/// Calculates all white pawn attacks
///
fn calculate_white_pawn_attacks() -> [Bitboard; 64] {
    let mut attacks = [Bitboard(0); 64];

    for file in 0..8 {
        for rank in 0..7 {
            let a_rank = rank + 1;
            attacks[board_idx!(rank, file)] = BIT_TABLE[board_idx!(a_rank, file + 1)];
            attacks[board_idx!(rank, file)] = BIT_TABLE[board_idx!(a_rank, file - 1)];
        }
    }

    attacks
}

///
/// Calculates all black pawn single space moves
///
fn calculate_black_pawn_single_moves() -> [Bitboard; 64] {
    let mut moves = [Bitboard(0); 64];

    for file in 0..8 {
        for rank in 7..0 {
            moves[board_idx!(rank, file)] |= BIT_TABLE[board_idx!(rank - 1, file)];
        }
    }

    moves
}

///
/// Calculates all black pawn double space moves
///
fn calculate_black_pawn_double_moves() -> [Bitboard; 64] {
    let mut moves = [Bitboard(0); 64];

    for file in 0..8 {
        let mrank = 4;
        moves[board_idx!(6, file)] |= BIT_TABLE[board_idx!(mrank, file)];
    }

    moves
}

///
/// Calculates all black pawn attacks
///
fn calculate_black_pawn_attacks() -> [Bitboard; 64] {
    let mut attacks = [Bitboard(0); 64];

    for file in 0..8 {
        for rank in 7..0 {
            let a_rank = rank - 1;
            attacks[board_idx!(rank, file)] = BIT_TABLE[board_idx!(a_rank, file + 1)];
            attacks[board_idx!(rank, file)] = BIT_TABLE[board_idx!(a_rank, file - 1)];
        }
    }

    attacks
}

///
/// Calculates all the knight moves
///
fn calculate_knight_moves() -> [Bitboard; 64] {
    let mut moves = [Bitboard(0); 64];

    for file in 0..8 {
        for rank in 0..8 {
            moves[board_idx!(rank, file)] |= BIT_TABLE[board_idx!(rank + 1, file - 2)];
            moves[board_idx!(rank, file)] |= BIT_TABLE[board_idx!(rank + 2, file - 1)];
            moves[board_idx!(rank, file)] |= BIT_TABLE[board_idx!(rank + 2, file + 1)];
            moves[board_idx!(rank, file)] |= BIT_TABLE[board_idx!(rank + 1, file + 2)];
            moves[board_idx!(rank, file)] |= BIT_TABLE[board_idx!(rank - 1, file + 2)];
            moves[board_idx!(rank, file)] |= BIT_TABLE[board_idx!(rank - 2, file + 1)];
            moves[board_idx!(rank, file)] |= BIT_TABLE[board_idx!(rank - 2, file - 1)];
            moves[board_idx!(rank, file)] |= BIT_TABLE[board_idx!(rank - 1, file - 2)];
        }
    }

    moves
}

///
/// Calculates all the king moves
///
fn calculate_king_moves() -> [Bitboard; 64] {
    let mut moves = [Bitboard(0); 64];

    for file in 0..8 {
        for rank in 0..8 {
            moves[board_idx!(rank, file)] |= BIT_TABLE[board_idx!(rank, file - 1)];
            moves[board_idx!(rank, file)] |= BIT_TABLE[board_idx!(rank + 1, file - 1)];
            moves[board_idx!(rank, file)] |= BIT_TABLE[board_idx!(rank + 1, file)];
            moves[board_idx!(rank, file)] |= BIT_TABLE[board_idx!(rank + 1, file + 1)];
            moves[board_idx!(rank, file)] |= BIT_TABLE[board_idx!(rank, file + 1)];
            moves[board_idx!(rank, file)] |= BIT_TABLE[board_idx!(rank - 1, file + 1)];
            moves[board_idx!(rank, file)] |= BIT_TABLE[board_idx!(rank - 1, file)];
            moves[board_idx!(rank, file)] |= BIT_TABLE[board_idx!(rank - 1, file - 1)];
        }
    }

    moves
}

///
/// Calculate all the rank attacks
///
fn calculate_rank_attacks() -> [[Bitboard; 64]; 64] {
    let mut attacks = [[Bitboard(0); 64]; 64];

    for file in 0..8 {
        for rank in 0..8 {
            for state in 0..64 {
                attacks[board_idx!(rank, file)][state] |= ((GEN_SLIDING[file - 1][state] as u64)
                    << RANK_SHIFT[board_idx!(rank, file) - 1])
                    .into();
            }
        }
    }

    attacks
}

///
/// Calculate all the file attacks
///
fn calculate_file_attacks() -> [[Bitboard; 64]; 64] {
    let mut attacks = [[Bitboard(0); 64]; 64];

    for file in 0..8 {
        for rank in 0..8 {
            for state in 0..64 {
                for attack_bit in 0..9 {
                    if GEN_SLIDING[8 - rank][state] & (1 << attack_bit) != 0 {
                        attacks[board_idx!(rank, file)][state] |=
                            BIT_TABLE[board_idx!(8 - attack_bit, file)];
                    }
                }
            }
        }
    }

    attacks
}

///
/// Calculates sliding moves on the A8H1 diagonals
///
fn calculate_a8h1_attacks() -> [[Bitboard; 64]; 64] {
    let mut attacks = [[Bitboard(0); 64]; 64];

    for file in 0..8 {
        for rank in 0..8 {
            for state in 0..64 {
                for attack_bit in 0..8 {
                    if GEN_SLIDING[board_idx!(8 - rank, file - 1)][state] & (1 << attack_bit) != 0 {
                        let diag = file + rank;
                        let (rnk, fle) = if diag < 0 {
                            (diag - file, attack_bit + 1)
                        } else {
                            (8 - attack_bit, diag - rank)
                        };

                        if fle > 0 && fle < 9 && rnk > 0 && rnk < 9 {
                            attacks[board_idx!(rank, file)][state] =
                                BIT_TABLE[board_idx!(rnk, fle)];
                        }
                    }
                }
            }
        }
    }

    attacks
}
fn calculate_a1h8_attacks() -> [[Bitboard; 64]; 64] {
    let mut attacks = [[Bitboard(0); 64]; 64];

    for file in 0..8 {
        for rank in 0..8 {
            for state in 0..64 {
                for attack_bit in 0..8 {
                    if GEN_SLIDING[board_idx!(rank - 1, file - 1)][state] & (1 << attack_bit) != 0 {
                        let diag = file - rank;
                        let (rnk, fle) = if diag < 0 {
                            (file - diag, attack_bit + 1)
                        } else {
                            (attack_bit + 1, diag + rank)
                        };

                        if fle > 0 && fle < 9 && rnk > 0 && rnk < 9 {
                            attacks[board_idx!(rank, file)][state] =
                                BIT_TABLE[board_idx!(rnk, fle)];
                        }
                    }
                }
            }
        }
    }

    attacks
}
