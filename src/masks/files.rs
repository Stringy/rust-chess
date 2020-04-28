use crate::bitboard::Bitboard;
#[allow(dead_code)]
pub const FILE_MASKS: [Bitboard; 64] = [
    // idx: 0
    //  0 0 0 0 0 0 0 0
    //  0 0 0 0 0 0 0 1
    //  0 0 0 0 0 0 0 1
    //  0 0 0 0 0 0 0 1
    //  0 0 0 0 0 0 0 1
    //  0 0 0 0 0 0 0 1
    //  0 0 0 0 0 0 0 1
    //  0 0 0 0 0 0 0 0
    Bitboard(0x1010101010100),
    // idx: 1
    //  0 0 0 0 0 0 0 0
    //  0 0 0 0 0 0 1 0
    //  0 0 0 0 0 0 1 0
    //  0 0 0 0 0 0 1 0
    //  0 0 0 0 0 0 1 0
    //  0 0 0 0 0 0 1 0
    //  0 0 0 0 0 0 1 0
    //  0 0 0 0 0 0 0 0
    Bitboard(0x2020202020200),
    // idx: 2
    //  0 0 0 0 0 0 0 0
    //  0 0 0 0 0 1 0 0
    //  0 0 0 0 0 1 0 0
    //  0 0 0 0 0 1 0 0
    //  0 0 0 0 0 1 0 0
    //  0 0 0 0 0 1 0 0
    //  0 0 0 0 0 1 0 0
    //  0 0 0 0 0 0 0 0
    Bitboard(0x4040404040400),
    // idx: 3
    //  0 0 0 0 0 0 0 0
    //  0 0 0 0 1 0 0 0
    //  0 0 0 0 1 0 0 0
    //  0 0 0 0 1 0 0 0
    //  0 0 0 0 1 0 0 0
    //  0 0 0 0 1 0 0 0
    //  0 0 0 0 1 0 0 0
    //  0 0 0 0 0 0 0 0
    Bitboard(0x8080808080800),
    // idx: 4
    //  0 0 0 0 0 0 0 0
    //  0 0 0 1 0 0 0 0
    //  0 0 0 1 0 0 0 0
    //  0 0 0 1 0 0 0 0
    //  0 0 0 1 0 0 0 0
    //  0 0 0 1 0 0 0 0
    //  0 0 0 1 0 0 0 0
    //  0 0 0 0 0 0 0 0
    Bitboard(0x10101010101000),
    // idx: 5
    //  0 0 0 0 0 0 0 0
    //  0 0 1 0 0 0 0 0
    //  0 0 1 0 0 0 0 0
    //  0 0 1 0 0 0 0 0
    //  0 0 1 0 0 0 0 0
    //  0 0 1 0 0 0 0 0
    //  0 0 1 0 0 0 0 0
    //  0 0 0 0 0 0 0 0
    Bitboard(0x20202020202000),
    // idx: 6
    //  0 0 0 0 0 0 0 0
    //  0 1 0 0 0 0 0 0
    //  0 1 0 0 0 0 0 0
    //  0 1 0 0 0 0 0 0
    //  0 1 0 0 0 0 0 0
    //  0 1 0 0 0 0 0 0
    //  0 1 0 0 0 0 0 0
    //  0 0 0 0 0 0 0 0
    Bitboard(0x40404040404000),
    // idx: 7
    //  0 0 0 0 0 0 0 0
    //  1 0 0 0 0 0 0 0
    //  1 0 0 0 0 0 0 0
    //  1 0 0 0 0 0 0 0
    //  1 0 0 0 0 0 0 0
    //  1 0 0 0 0 0 0 0
    //  1 0 0 0 0 0 0 0
    //  0 0 0 0 0 0 0 0
    Bitboard(0x80808080808000),
    // idx: 8
    //  0 0 0 0 0 0 0 0
    //  0 0 0 0 0 0 0 1
    //  0 0 0 0 0 0 0 1
    //  0 0 0 0 0 0 0 1
    //  0 0 0 0 0 0 0 1
    //  0 0 0 0 0 0 0 1
    //  0 0 0 0 0 0 0 1
    //  0 0 0 0 0 0 0 0
    Bitboard(0x1010101010100),
    // idx: 9
    //  0 0 0 0 0 0 0 0
    //  0 0 0 0 0 0 1 0
    //  0 0 0 0 0 0 1 0
    //  0 0 0 0 0 0 1 0
    //  0 0 0 0 0 0 1 0
    //  0 0 0 0 0 0 1 0
    //  0 0 0 0 0 0 1 0
    //  0 0 0 0 0 0 0 0
    Bitboard(0x2020202020200),
    // idx: 10
    //  0 0 0 0 0 0 0 0
    //  0 0 0 0 0 1 0 0
    //  0 0 0 0 0 1 0 0
    //  0 0 0 0 0 1 0 0
    //  0 0 0 0 0 1 0 0
    //  0 0 0 0 0 1 0 0
    //  0 0 0 0 0 1 0 0
    //  0 0 0 0 0 0 0 0
    Bitboard(0x4040404040400),
    // idx: 11
    //  0 0 0 0 0 0 0 0
    //  0 0 0 0 1 0 0 0
    //  0 0 0 0 1 0 0 0
    //  0 0 0 0 1 0 0 0
    //  0 0 0 0 1 0 0 0
    //  0 0 0 0 1 0 0 0
    //  0 0 0 0 1 0 0 0
    //  0 0 0 0 0 0 0 0
    Bitboard(0x8080808080800),
    // idx: 12
    //  0 0 0 0 0 0 0 0
    //  0 0 0 1 0 0 0 0
    //  0 0 0 1 0 0 0 0
    //  0 0 0 1 0 0 0 0
    //  0 0 0 1 0 0 0 0
    //  0 0 0 1 0 0 0 0
    //  0 0 0 1 0 0 0 0
    //  0 0 0 0 0 0 0 0
    Bitboard(0x10101010101000),
    // idx: 13
    //  0 0 0 0 0 0 0 0
    //  0 0 1 0 0 0 0 0
    //  0 0 1 0 0 0 0 0
    //  0 0 1 0 0 0 0 0
    //  0 0 1 0 0 0 0 0
    //  0 0 1 0 0 0 0 0
    //  0 0 1 0 0 0 0 0
    //  0 0 0 0 0 0 0 0
    Bitboard(0x20202020202000),
    // idx: 14
    //  0 0 0 0 0 0 0 0
    //  0 1 0 0 0 0 0 0
    //  0 1 0 0 0 0 0 0
    //  0 1 0 0 0 0 0 0
    //  0 1 0 0 0 0 0 0
    //  0 1 0 0 0 0 0 0
    //  0 1 0 0 0 0 0 0
    //  0 0 0 0 0 0 0 0
    Bitboard(0x40404040404000),
    // idx: 15
    //  0 0 0 0 0 0 0 0
    //  1 0 0 0 0 0 0 0
    //  1 0 0 0 0 0 0 0
    //  1 0 0 0 0 0 0 0
    //  1 0 0 0 0 0 0 0
    //  1 0 0 0 0 0 0 0
    //  1 0 0 0 0 0 0 0
    //  0 0 0 0 0 0 0 0
    Bitboard(0x80808080808000),
    // idx: 16
    //  0 0 0 0 0 0 0 0
    //  0 0 0 0 0 0 0 1
    //  0 0 0 0 0 0 0 1
    //  0 0 0 0 0 0 0 1
    //  0 0 0 0 0 0 0 1
    //  0 0 0 0 0 0 0 1
    //  0 0 0 0 0 0 0 1
    //  0 0 0 0 0 0 0 0
    Bitboard(0x1010101010100),
    // idx: 17
    //  0 0 0 0 0 0 0 0
    //  0 0 0 0 0 0 1 0
    //  0 0 0 0 0 0 1 0
    //  0 0 0 0 0 0 1 0
    //  0 0 0 0 0 0 1 0
    //  0 0 0 0 0 0 1 0
    //  0 0 0 0 0 0 1 0
    //  0 0 0 0 0 0 0 0
    Bitboard(0x2020202020200),
    // idx: 18
    //  0 0 0 0 0 0 0 0
    //  0 0 0 0 0 1 0 0
    //  0 0 0 0 0 1 0 0
    //  0 0 0 0 0 1 0 0
    //  0 0 0 0 0 1 0 0
    //  0 0 0 0 0 1 0 0
    //  0 0 0 0 0 1 0 0
    //  0 0 0 0 0 0 0 0
    Bitboard(0x4040404040400),
    // idx: 19
    //  0 0 0 0 0 0 0 0
    //  0 0 0 0 1 0 0 0
    //  0 0 0 0 1 0 0 0
    //  0 0 0 0 1 0 0 0
    //  0 0 0 0 1 0 0 0
    //  0 0 0 0 1 0 0 0
    //  0 0 0 0 1 0 0 0
    //  0 0 0 0 0 0 0 0
    Bitboard(0x8080808080800),
    // idx: 20
    //  0 0 0 0 0 0 0 0
    //  0 0 0 1 0 0 0 0
    //  0 0 0 1 0 0 0 0
    //  0 0 0 1 0 0 0 0
    //  0 0 0 1 0 0 0 0
    //  0 0 0 1 0 0 0 0
    //  0 0 0 1 0 0 0 0
    //  0 0 0 0 0 0 0 0
    Bitboard(0x10101010101000),
    // idx: 21
    //  0 0 0 0 0 0 0 0
    //  0 0 1 0 0 0 0 0
    //  0 0 1 0 0 0 0 0
    //  0 0 1 0 0 0 0 0
    //  0 0 1 0 0 0 0 0
    //  0 0 1 0 0 0 0 0
    //  0 0 1 0 0 0 0 0
    //  0 0 0 0 0 0 0 0
    Bitboard(0x20202020202000),
    // idx: 22
    //  0 0 0 0 0 0 0 0
    //  0 1 0 0 0 0 0 0
    //  0 1 0 0 0 0 0 0
    //  0 1 0 0 0 0 0 0
    //  0 1 0 0 0 0 0 0
    //  0 1 0 0 0 0 0 0
    //  0 1 0 0 0 0 0 0
    //  0 0 0 0 0 0 0 0
    Bitboard(0x40404040404000),
    // idx: 23
    //  0 0 0 0 0 0 0 0
    //  1 0 0 0 0 0 0 0
    //  1 0 0 0 0 0 0 0
    //  1 0 0 0 0 0 0 0
    //  1 0 0 0 0 0 0 0
    //  1 0 0 0 0 0 0 0
    //  1 0 0 0 0 0 0 0
    //  0 0 0 0 0 0 0 0
    Bitboard(0x80808080808000),
    // idx: 24
    //  0 0 0 0 0 0 0 0
    //  0 0 0 0 0 0 0 1
    //  0 0 0 0 0 0 0 1
    //  0 0 0 0 0 0 0 1
    //  0 0 0 0 0 0 0 1
    //  0 0 0 0 0 0 0 1
    //  0 0 0 0 0 0 0 1
    //  0 0 0 0 0 0 0 0
    Bitboard(0x1010101010100),
    // idx: 25
    //  0 0 0 0 0 0 0 0
    //  0 0 0 0 0 0 1 0
    //  0 0 0 0 0 0 1 0
    //  0 0 0 0 0 0 1 0
    //  0 0 0 0 0 0 1 0
    //  0 0 0 0 0 0 1 0
    //  0 0 0 0 0 0 1 0
    //  0 0 0 0 0 0 0 0
    Bitboard(0x2020202020200),
    // idx: 26
    //  0 0 0 0 0 0 0 0
    //  0 0 0 0 0 1 0 0
    //  0 0 0 0 0 1 0 0
    //  0 0 0 0 0 1 0 0
    //  0 0 0 0 0 1 0 0
    //  0 0 0 0 0 1 0 0
    //  0 0 0 0 0 1 0 0
    //  0 0 0 0 0 0 0 0
    Bitboard(0x4040404040400),
    // idx: 27
    //  0 0 0 0 0 0 0 0
    //  0 0 0 0 1 0 0 0
    //  0 0 0 0 1 0 0 0
    //  0 0 0 0 1 0 0 0
    //  0 0 0 0 1 0 0 0
    //  0 0 0 0 1 0 0 0
    //  0 0 0 0 1 0 0 0
    //  0 0 0 0 0 0 0 0
    Bitboard(0x8080808080800),
    // idx: 28
    //  0 0 0 0 0 0 0 0
    //  0 0 0 1 0 0 0 0
    //  0 0 0 1 0 0 0 0
    //  0 0 0 1 0 0 0 0
    //  0 0 0 1 0 0 0 0
    //  0 0 0 1 0 0 0 0
    //  0 0 0 1 0 0 0 0
    //  0 0 0 0 0 0 0 0
    Bitboard(0x10101010101000),
    // idx: 29
    //  0 0 0 0 0 0 0 0
    //  0 0 1 0 0 0 0 0
    //  0 0 1 0 0 0 0 0
    //  0 0 1 0 0 0 0 0
    //  0 0 1 0 0 0 0 0
    //  0 0 1 0 0 0 0 0
    //  0 0 1 0 0 0 0 0
    //  0 0 0 0 0 0 0 0
    Bitboard(0x20202020202000),
    // idx: 30
    //  0 0 0 0 0 0 0 0
    //  0 1 0 0 0 0 0 0
    //  0 1 0 0 0 0 0 0
    //  0 1 0 0 0 0 0 0
    //  0 1 0 0 0 0 0 0
    //  0 1 0 0 0 0 0 0
    //  0 1 0 0 0 0 0 0
    //  0 0 0 0 0 0 0 0
    Bitboard(0x40404040404000),
    // idx: 31
    //  0 0 0 0 0 0 0 0
    //  1 0 0 0 0 0 0 0
    //  1 0 0 0 0 0 0 0
    //  1 0 0 0 0 0 0 0
    //  1 0 0 0 0 0 0 0
    //  1 0 0 0 0 0 0 0
    //  1 0 0 0 0 0 0 0
    //  0 0 0 0 0 0 0 0
    Bitboard(0x80808080808000),
    // idx: 32
    //  0 0 0 0 0 0 0 0
    //  0 0 0 0 0 0 0 1
    //  0 0 0 0 0 0 0 1
    //  0 0 0 0 0 0 0 1
    //  0 0 0 0 0 0 0 1
    //  0 0 0 0 0 0 0 1
    //  0 0 0 0 0 0 0 1
    //  0 0 0 0 0 0 0 0
    Bitboard(0x1010101010100),
    // idx: 33
    //  0 0 0 0 0 0 0 0
    //  0 0 0 0 0 0 1 0
    //  0 0 0 0 0 0 1 0
    //  0 0 0 0 0 0 1 0
    //  0 0 0 0 0 0 1 0
    //  0 0 0 0 0 0 1 0
    //  0 0 0 0 0 0 1 0
    //  0 0 0 0 0 0 0 0
    Bitboard(0x2020202020200),
    // idx: 34
    //  0 0 0 0 0 0 0 0
    //  0 0 0 0 0 1 0 0
    //  0 0 0 0 0 1 0 0
    //  0 0 0 0 0 1 0 0
    //  0 0 0 0 0 1 0 0
    //  0 0 0 0 0 1 0 0
    //  0 0 0 0 0 1 0 0
    //  0 0 0 0 0 0 0 0
    Bitboard(0x4040404040400),
    // idx: 35
    //  0 0 0 0 0 0 0 0
    //  0 0 0 0 1 0 0 0
    //  0 0 0 0 1 0 0 0
    //  0 0 0 0 1 0 0 0
    //  0 0 0 0 1 0 0 0
    //  0 0 0 0 1 0 0 0
    //  0 0 0 0 1 0 0 0
    //  0 0 0 0 0 0 0 0
    Bitboard(0x8080808080800),
    // idx: 36
    //  0 0 0 0 0 0 0 0
    //  0 0 0 1 0 0 0 0
    //  0 0 0 1 0 0 0 0
    //  0 0 0 1 0 0 0 0
    //  0 0 0 1 0 0 0 0
    //  0 0 0 1 0 0 0 0
    //  0 0 0 1 0 0 0 0
    //  0 0 0 0 0 0 0 0
    Bitboard(0x10101010101000),
    // idx: 37
    //  0 0 0 0 0 0 0 0
    //  0 0 1 0 0 0 0 0
    //  0 0 1 0 0 0 0 0
    //  0 0 1 0 0 0 0 0
    //  0 0 1 0 0 0 0 0
    //  0 0 1 0 0 0 0 0
    //  0 0 1 0 0 0 0 0
    //  0 0 0 0 0 0 0 0
    Bitboard(0x20202020202000),
    // idx: 38
    //  0 0 0 0 0 0 0 0
    //  0 1 0 0 0 0 0 0
    //  0 1 0 0 0 0 0 0
    //  0 1 0 0 0 0 0 0
    //  0 1 0 0 0 0 0 0
    //  0 1 0 0 0 0 0 0
    //  0 1 0 0 0 0 0 0
    //  0 0 0 0 0 0 0 0
    Bitboard(0x40404040404000),
    // idx: 39
    //  0 0 0 0 0 0 0 0
    //  1 0 0 0 0 0 0 0
    //  1 0 0 0 0 0 0 0
    //  1 0 0 0 0 0 0 0
    //  1 0 0 0 0 0 0 0
    //  1 0 0 0 0 0 0 0
    //  1 0 0 0 0 0 0 0
    //  0 0 0 0 0 0 0 0
    Bitboard(0x80808080808000),
    // idx: 40
    //  0 0 0 0 0 0 0 0
    //  0 0 0 0 0 0 0 1
    //  0 0 0 0 0 0 0 1
    //  0 0 0 0 0 0 0 1
    //  0 0 0 0 0 0 0 1
    //  0 0 0 0 0 0 0 1
    //  0 0 0 0 0 0 0 1
    //  0 0 0 0 0 0 0 0
    Bitboard(0x1010101010100),
    // idx: 41
    //  0 0 0 0 0 0 0 0
    //  0 0 0 0 0 0 1 0
    //  0 0 0 0 0 0 1 0
    //  0 0 0 0 0 0 1 0
    //  0 0 0 0 0 0 1 0
    //  0 0 0 0 0 0 1 0
    //  0 0 0 0 0 0 1 0
    //  0 0 0 0 0 0 0 0
    Bitboard(0x2020202020200),
    // idx: 42
    //  0 0 0 0 0 0 0 0
    //  0 0 0 0 0 1 0 0
    //  0 0 0 0 0 1 0 0
    //  0 0 0 0 0 1 0 0
    //  0 0 0 0 0 1 0 0
    //  0 0 0 0 0 1 0 0
    //  0 0 0 0 0 1 0 0
    //  0 0 0 0 0 0 0 0
    Bitboard(0x4040404040400),
    // idx: 43
    //  0 0 0 0 0 0 0 0
    //  0 0 0 0 1 0 0 0
    //  0 0 0 0 1 0 0 0
    //  0 0 0 0 1 0 0 0
    //  0 0 0 0 1 0 0 0
    //  0 0 0 0 1 0 0 0
    //  0 0 0 0 1 0 0 0
    //  0 0 0 0 0 0 0 0
    Bitboard(0x8080808080800),
    // idx: 44
    //  0 0 0 0 0 0 0 0
    //  0 0 0 1 0 0 0 0
    //  0 0 0 1 0 0 0 0
    //  0 0 0 1 0 0 0 0
    //  0 0 0 1 0 0 0 0
    //  0 0 0 1 0 0 0 0
    //  0 0 0 1 0 0 0 0
    //  0 0 0 0 0 0 0 0
    Bitboard(0x10101010101000),
    // idx: 45
    //  0 0 0 0 0 0 0 0
    //  0 0 1 0 0 0 0 0
    //  0 0 1 0 0 0 0 0
    //  0 0 1 0 0 0 0 0
    //  0 0 1 0 0 0 0 0
    //  0 0 1 0 0 0 0 0
    //  0 0 1 0 0 0 0 0
    //  0 0 0 0 0 0 0 0
    Bitboard(0x20202020202000),
    // idx: 46
    //  0 0 0 0 0 0 0 0
    //  0 1 0 0 0 0 0 0
    //  0 1 0 0 0 0 0 0
    //  0 1 0 0 0 0 0 0
    //  0 1 0 0 0 0 0 0
    //  0 1 0 0 0 0 0 0
    //  0 1 0 0 0 0 0 0
    //  0 0 0 0 0 0 0 0
    Bitboard(0x40404040404000),
    // idx: 47
    //  0 0 0 0 0 0 0 0
    //  1 0 0 0 0 0 0 0
    //  1 0 0 0 0 0 0 0
    //  1 0 0 0 0 0 0 0
    //  1 0 0 0 0 0 0 0
    //  1 0 0 0 0 0 0 0
    //  1 0 0 0 0 0 0 0
    //  0 0 0 0 0 0 0 0
    Bitboard(0x80808080808000),
    // idx: 48
    //  0 0 0 0 0 0 0 0
    //  0 0 0 0 0 0 0 1
    //  0 0 0 0 0 0 0 1
    //  0 0 0 0 0 0 0 1
    //  0 0 0 0 0 0 0 1
    //  0 0 0 0 0 0 0 1
    //  0 0 0 0 0 0 0 1
    //  0 0 0 0 0 0 0 0
    Bitboard(0x1010101010100),
    // idx: 49
    //  0 0 0 0 0 0 0 0
    //  0 0 0 0 0 0 1 0
    //  0 0 0 0 0 0 1 0
    //  0 0 0 0 0 0 1 0
    //  0 0 0 0 0 0 1 0
    //  0 0 0 0 0 0 1 0
    //  0 0 0 0 0 0 1 0
    //  0 0 0 0 0 0 0 0
    Bitboard(0x2020202020200),
    // idx: 50
    //  0 0 0 0 0 0 0 0
    //  0 0 0 0 0 1 0 0
    //  0 0 0 0 0 1 0 0
    //  0 0 0 0 0 1 0 0
    //  0 0 0 0 0 1 0 0
    //  0 0 0 0 0 1 0 0
    //  0 0 0 0 0 1 0 0
    //  0 0 0 0 0 0 0 0
    Bitboard(0x4040404040400),
    // idx: 51
    //  0 0 0 0 0 0 0 0
    //  0 0 0 0 1 0 0 0
    //  0 0 0 0 1 0 0 0
    //  0 0 0 0 1 0 0 0
    //  0 0 0 0 1 0 0 0
    //  0 0 0 0 1 0 0 0
    //  0 0 0 0 1 0 0 0
    //  0 0 0 0 0 0 0 0
    Bitboard(0x8080808080800),
    // idx: 52
    //  0 0 0 0 0 0 0 0
    //  0 0 0 1 0 0 0 0
    //  0 0 0 1 0 0 0 0
    //  0 0 0 1 0 0 0 0
    //  0 0 0 1 0 0 0 0
    //  0 0 0 1 0 0 0 0
    //  0 0 0 1 0 0 0 0
    //  0 0 0 0 0 0 0 0
    Bitboard(0x10101010101000),
    // idx: 53
    //  0 0 0 0 0 0 0 0
    //  0 0 1 0 0 0 0 0
    //  0 0 1 0 0 0 0 0
    //  0 0 1 0 0 0 0 0
    //  0 0 1 0 0 0 0 0
    //  0 0 1 0 0 0 0 0
    //  0 0 1 0 0 0 0 0
    //  0 0 0 0 0 0 0 0
    Bitboard(0x20202020202000),
    // idx: 54
    //  0 0 0 0 0 0 0 0
    //  0 1 0 0 0 0 0 0
    //  0 1 0 0 0 0 0 0
    //  0 1 0 0 0 0 0 0
    //  0 1 0 0 0 0 0 0
    //  0 1 0 0 0 0 0 0
    //  0 1 0 0 0 0 0 0
    //  0 0 0 0 0 0 0 0
    Bitboard(0x40404040404000),
    // idx: 55
    //  0 0 0 0 0 0 0 0
    //  1 0 0 0 0 0 0 0
    //  1 0 0 0 0 0 0 0
    //  1 0 0 0 0 0 0 0
    //  1 0 0 0 0 0 0 0
    //  1 0 0 0 0 0 0 0
    //  1 0 0 0 0 0 0 0
    //  0 0 0 0 0 0 0 0
    Bitboard(0x80808080808000),
    // idx: 56
    //  0 0 0 0 0 0 0 0
    //  0 0 0 0 0 0 0 1
    //  0 0 0 0 0 0 0 1
    //  0 0 0 0 0 0 0 1
    //  0 0 0 0 0 0 0 1
    //  0 0 0 0 0 0 0 1
    //  0 0 0 0 0 0 0 1
    //  0 0 0 0 0 0 0 0
    Bitboard(0x1010101010100),
    // idx: 57
    //  0 0 0 0 0 0 0 0
    //  0 0 0 0 0 0 1 0
    //  0 0 0 0 0 0 1 0
    //  0 0 0 0 0 0 1 0
    //  0 0 0 0 0 0 1 0
    //  0 0 0 0 0 0 1 0
    //  0 0 0 0 0 0 1 0
    //  0 0 0 0 0 0 0 0
    Bitboard(0x2020202020200),
    // idx: 58
    //  0 0 0 0 0 0 0 0
    //  0 0 0 0 0 1 0 0
    //  0 0 0 0 0 1 0 0
    //  0 0 0 0 0 1 0 0
    //  0 0 0 0 0 1 0 0
    //  0 0 0 0 0 1 0 0
    //  0 0 0 0 0 1 0 0
    //  0 0 0 0 0 0 0 0
    Bitboard(0x4040404040400),
    // idx: 59
    //  0 0 0 0 0 0 0 0
    //  0 0 0 0 1 0 0 0
    //  0 0 0 0 1 0 0 0
    //  0 0 0 0 1 0 0 0
    //  0 0 0 0 1 0 0 0
    //  0 0 0 0 1 0 0 0
    //  0 0 0 0 1 0 0 0
    //  0 0 0 0 0 0 0 0
    Bitboard(0x8080808080800),
    // idx: 60
    //  0 0 0 0 0 0 0 0
    //  0 0 0 1 0 0 0 0
    //  0 0 0 1 0 0 0 0
    //  0 0 0 1 0 0 0 0
    //  0 0 0 1 0 0 0 0
    //  0 0 0 1 0 0 0 0
    //  0 0 0 1 0 0 0 0
    //  0 0 0 0 0 0 0 0
    Bitboard(0x10101010101000),
    // idx: 61
    //  0 0 0 0 0 0 0 0
    //  0 0 1 0 0 0 0 0
    //  0 0 1 0 0 0 0 0
    //  0 0 1 0 0 0 0 0
    //  0 0 1 0 0 0 0 0
    //  0 0 1 0 0 0 0 0
    //  0 0 1 0 0 0 0 0
    //  0 0 0 0 0 0 0 0
    Bitboard(0x20202020202000),
    // idx: 62
    //  0 0 0 0 0 0 0 0
    //  0 1 0 0 0 0 0 0
    //  0 1 0 0 0 0 0 0
    //  0 1 0 0 0 0 0 0
    //  0 1 0 0 0 0 0 0
    //  0 1 0 0 0 0 0 0
    //  0 1 0 0 0 0 0 0
    //  0 0 0 0 0 0 0 0
    Bitboard(0x40404040404000),
    // idx: 63
    //  0 0 0 0 0 0 0 0
    //  1 0 0 0 0 0 0 0
    //  1 0 0 0 0 0 0 0
    //  1 0 0 0 0 0 0 0
    //  1 0 0 0 0 0 0 0
    //  1 0 0 0 0 0 0 0
    //  1 0 0 0 0 0 0 0
    //  0 0 0 0 0 0 0 0
    Bitboard(0x80808080808000),
];

#[allow(dead_code)]
pub const FILE_MAGIC: [Bitboard; 64] = [
    // idx: 0
    //  1 0 0 0 0 0 0 0
    //  0 1 0 0 0 0 0 0
    //  0 0 1 0 0 0 0 0
    //  0 0 0 1 0 0 0 0
    //  0 0 0 0 1 0 0 0
    //  0 0 0 0 0 1 0 0
    //  0 0 0 0 0 0 1 0
    //  0 0 0 0 0 0 0 0
    Bitboard(0x8040201008040200),
    // idx: 1
    //  0 1 0 0 0 0 0 0
    //  0 0 1 0 0 0 0 0
    //  0 0 0 1 0 0 0 0
    //  0 0 0 0 1 0 0 0
    //  0 0 0 0 0 1 0 0
    //  0 0 0 0 0 0 1 0
    //  0 0 0 0 0 0 0 1
    //  0 0 0 0 0 0 0 0
    Bitboard(0x4020100804020100),
    // idx: 2
    //  0 0 1 0 0 0 0 0
    //  0 0 0 1 0 0 0 0
    //  0 0 0 0 1 0 0 0
    //  0 0 0 0 0 1 0 0
    //  0 0 0 0 0 0 1 0
    //  0 0 0 0 0 0 0 1
    //  0 0 0 0 0 0 0 0
    //  1 0 0 0 0 0 0 0
    Bitboard(0x2010080402010080),
    // idx: 3
    //  0 0 0 1 0 0 0 0
    //  0 0 0 0 1 0 0 0
    //  0 0 0 0 0 1 0 0
    //  0 0 0 0 0 0 1 0
    //  0 0 0 0 0 0 0 1
    //  0 0 0 0 0 0 0 0
    //  1 0 0 0 0 0 0 0
    //  0 1 0 0 0 0 0 0
    Bitboard(0x1008040201008040),
    // idx: 4
    //  0 0 0 0 1 0 0 0
    //  0 0 0 0 0 1 0 0
    //  0 0 0 0 0 0 1 0
    //  0 0 0 0 0 0 0 1
    //  0 0 0 0 0 0 0 0
    //  1 0 0 0 0 0 0 0
    //  0 1 0 0 0 0 0 0
    //  0 0 1 0 0 0 0 0
    Bitboard(0x804020100804020),
    // idx: 5
    //  0 0 0 0 0 1 0 0
    //  0 0 0 0 0 0 1 0
    //  0 0 0 0 0 0 0 1
    //  0 0 0 0 0 0 0 0
    //  1 0 0 0 0 0 0 0
    //  0 1 0 0 0 0 0 0
    //  0 0 1 0 0 0 0 0
    //  0 0 0 1 0 0 0 0
    Bitboard(0x402010080402010),
    // idx: 6
    //  0 0 0 0 0 0 1 0
    //  0 0 0 0 0 0 0 1
    //  0 0 0 0 0 0 0 0
    //  1 0 0 0 0 0 0 0
    //  0 1 0 0 0 0 0 0
    //  0 0 1 0 0 0 0 0
    //  0 0 0 1 0 0 0 0
    //  0 0 0 0 1 0 0 0
    Bitboard(0x201008040201008),
    // idx: 7
    //  0 0 0 0 0 0 0 1
    //  0 0 0 0 0 0 0 0
    //  1 0 0 0 0 0 0 0
    //  0 1 0 0 0 0 0 0
    //  0 0 1 0 0 0 0 0
    //  0 0 0 1 0 0 0 0
    //  0 0 0 0 1 0 0 0
    //  0 0 0 0 0 1 0 0
    Bitboard(0x100804020100804),
    // idx: 8
    //  1 0 0 0 0 0 0 0
    //  0 1 0 0 0 0 0 0
    //  0 0 1 0 0 0 0 0
    //  0 0 0 1 0 0 0 0
    //  0 0 0 0 1 0 0 0
    //  0 0 0 0 0 1 0 0
    //  0 0 0 0 0 0 1 0
    //  0 0 0 0 0 0 0 0
    Bitboard(0x8040201008040200),
    // idx: 9
    //  0 1 0 0 0 0 0 0
    //  0 0 1 0 0 0 0 0
    //  0 0 0 1 0 0 0 0
    //  0 0 0 0 1 0 0 0
    //  0 0 0 0 0 1 0 0
    //  0 0 0 0 0 0 1 0
    //  0 0 0 0 0 0 0 1
    //  0 0 0 0 0 0 0 0
    Bitboard(0x4020100804020100),
    // idx: 10
    //  0 0 1 0 0 0 0 0
    //  0 0 0 1 0 0 0 0
    //  0 0 0 0 1 0 0 0
    //  0 0 0 0 0 1 0 0
    //  0 0 0 0 0 0 1 0
    //  0 0 0 0 0 0 0 1
    //  0 0 0 0 0 0 0 0
    //  1 0 0 0 0 0 0 0
    Bitboard(0x2010080402010080),
    // idx: 11
    //  0 0 0 1 0 0 0 0
    //  0 0 0 0 1 0 0 0
    //  0 0 0 0 0 1 0 0
    //  0 0 0 0 0 0 1 0
    //  0 0 0 0 0 0 0 1
    //  0 0 0 0 0 0 0 0
    //  1 0 0 0 0 0 0 0
    //  0 1 0 0 0 0 0 0
    Bitboard(0x1008040201008040),
    // idx: 12
    //  0 0 0 0 1 0 0 0
    //  0 0 0 0 0 1 0 0
    //  0 0 0 0 0 0 1 0
    //  0 0 0 0 0 0 0 1
    //  0 0 0 0 0 0 0 0
    //  1 0 0 0 0 0 0 0
    //  0 1 0 0 0 0 0 0
    //  0 0 1 0 0 0 0 0
    Bitboard(0x804020100804020),
    // idx: 13
    //  0 0 0 0 0 1 0 0
    //  0 0 0 0 0 0 1 0
    //  0 0 0 0 0 0 0 1
    //  0 0 0 0 0 0 0 0
    //  1 0 0 0 0 0 0 0
    //  0 1 0 0 0 0 0 0
    //  0 0 1 0 0 0 0 0
    //  0 0 0 1 0 0 0 0
    Bitboard(0x402010080402010),
    // idx: 14
    //  0 0 0 0 0 0 1 0
    //  0 0 0 0 0 0 0 1
    //  0 0 0 0 0 0 0 0
    //  1 0 0 0 0 0 0 0
    //  0 1 0 0 0 0 0 0
    //  0 0 1 0 0 0 0 0
    //  0 0 0 1 0 0 0 0
    //  0 0 0 0 1 0 0 0
    Bitboard(0x201008040201008),
    // idx: 15
    //  0 0 0 0 0 0 0 1
    //  0 0 0 0 0 0 0 0
    //  1 0 0 0 0 0 0 0
    //  0 1 0 0 0 0 0 0
    //  0 0 1 0 0 0 0 0
    //  0 0 0 1 0 0 0 0
    //  0 0 0 0 1 0 0 0
    //  0 0 0 0 0 1 0 0
    Bitboard(0x100804020100804),
    // idx: 16
    //  1 0 0 0 0 0 0 0
    //  0 1 0 0 0 0 0 0
    //  0 0 1 0 0 0 0 0
    //  0 0 0 1 0 0 0 0
    //  0 0 0 0 1 0 0 0
    //  0 0 0 0 0 1 0 0
    //  0 0 0 0 0 0 1 0
    //  0 0 0 0 0 0 0 0
    Bitboard(0x8040201008040200),
    // idx: 17
    //  0 1 0 0 0 0 0 0
    //  0 0 1 0 0 0 0 0
    //  0 0 0 1 0 0 0 0
    //  0 0 0 0 1 0 0 0
    //  0 0 0 0 0 1 0 0
    //  0 0 0 0 0 0 1 0
    //  0 0 0 0 0 0 0 1
    //  0 0 0 0 0 0 0 0
    Bitboard(0x4020100804020100),
    // idx: 18
    //  0 0 1 0 0 0 0 0
    //  0 0 0 1 0 0 0 0
    //  0 0 0 0 1 0 0 0
    //  0 0 0 0 0 1 0 0
    //  0 0 0 0 0 0 1 0
    //  0 0 0 0 0 0 0 1
    //  0 0 0 0 0 0 0 0
    //  1 0 0 0 0 0 0 0
    Bitboard(0x2010080402010080),
    // idx: 19
    //  0 0 0 1 0 0 0 0
    //  0 0 0 0 1 0 0 0
    //  0 0 0 0 0 1 0 0
    //  0 0 0 0 0 0 1 0
    //  0 0 0 0 0 0 0 1
    //  0 0 0 0 0 0 0 0
    //  1 0 0 0 0 0 0 0
    //  0 1 0 0 0 0 0 0
    Bitboard(0x1008040201008040),
    // idx: 20
    //  0 0 0 0 1 0 0 0
    //  0 0 0 0 0 1 0 0
    //  0 0 0 0 0 0 1 0
    //  0 0 0 0 0 0 0 1
    //  0 0 0 0 0 0 0 0
    //  1 0 0 0 0 0 0 0
    //  0 1 0 0 0 0 0 0
    //  0 0 1 0 0 0 0 0
    Bitboard(0x804020100804020),
    // idx: 21
    //  0 0 0 0 0 1 0 0
    //  0 0 0 0 0 0 1 0
    //  0 0 0 0 0 0 0 1
    //  0 0 0 0 0 0 0 0
    //  1 0 0 0 0 0 0 0
    //  0 1 0 0 0 0 0 0
    //  0 0 1 0 0 0 0 0
    //  0 0 0 1 0 0 0 0
    Bitboard(0x402010080402010),
    // idx: 22
    //  0 0 0 0 0 0 1 0
    //  0 0 0 0 0 0 0 1
    //  0 0 0 0 0 0 0 0
    //  1 0 0 0 0 0 0 0
    //  0 1 0 0 0 0 0 0
    //  0 0 1 0 0 0 0 0
    //  0 0 0 1 0 0 0 0
    //  0 0 0 0 1 0 0 0
    Bitboard(0x201008040201008),
    // idx: 23
    //  0 0 0 0 0 0 0 1
    //  0 0 0 0 0 0 0 0
    //  1 0 0 0 0 0 0 0
    //  0 1 0 0 0 0 0 0
    //  0 0 1 0 0 0 0 0
    //  0 0 0 1 0 0 0 0
    //  0 0 0 0 1 0 0 0
    //  0 0 0 0 0 1 0 0
    Bitboard(0x100804020100804),
    // idx: 24
    //  1 0 0 0 0 0 0 0
    //  0 1 0 0 0 0 0 0
    //  0 0 1 0 0 0 0 0
    //  0 0 0 1 0 0 0 0
    //  0 0 0 0 1 0 0 0
    //  0 0 0 0 0 1 0 0
    //  0 0 0 0 0 0 1 0
    //  0 0 0 0 0 0 0 0
    Bitboard(0x8040201008040200),
    // idx: 25
    //  0 1 0 0 0 0 0 0
    //  0 0 1 0 0 0 0 0
    //  0 0 0 1 0 0 0 0
    //  0 0 0 0 1 0 0 0
    //  0 0 0 0 0 1 0 0
    //  0 0 0 0 0 0 1 0
    //  0 0 0 0 0 0 0 1
    //  0 0 0 0 0 0 0 0
    Bitboard(0x4020100804020100),
    // idx: 26
    //  0 0 1 0 0 0 0 0
    //  0 0 0 1 0 0 0 0
    //  0 0 0 0 1 0 0 0
    //  0 0 0 0 0 1 0 0
    //  0 0 0 0 0 0 1 0
    //  0 0 0 0 0 0 0 1
    //  0 0 0 0 0 0 0 0
    //  1 0 0 0 0 0 0 0
    Bitboard(0x2010080402010080),
    // idx: 27
    //  0 0 0 1 0 0 0 0
    //  0 0 0 0 1 0 0 0
    //  0 0 0 0 0 1 0 0
    //  0 0 0 0 0 0 1 0
    //  0 0 0 0 0 0 0 1
    //  0 0 0 0 0 0 0 0
    //  1 0 0 0 0 0 0 0
    //  0 1 0 0 0 0 0 0
    Bitboard(0x1008040201008040),
    // idx: 28
    //  0 0 0 0 1 0 0 0
    //  0 0 0 0 0 1 0 0
    //  0 0 0 0 0 0 1 0
    //  0 0 0 0 0 0 0 1
    //  0 0 0 0 0 0 0 0
    //  1 0 0 0 0 0 0 0
    //  0 1 0 0 0 0 0 0
    //  0 0 1 0 0 0 0 0
    Bitboard(0x804020100804020),
    // idx: 29
    //  0 0 0 0 0 1 0 0
    //  0 0 0 0 0 0 1 0
    //  0 0 0 0 0 0 0 1
    //  0 0 0 0 0 0 0 0
    //  1 0 0 0 0 0 0 0
    //  0 1 0 0 0 0 0 0
    //  0 0 1 0 0 0 0 0
    //  0 0 0 1 0 0 0 0
    Bitboard(0x402010080402010),
    // idx: 30
    //  0 0 0 0 0 0 1 0
    //  0 0 0 0 0 0 0 1
    //  0 0 0 0 0 0 0 0
    //  1 0 0 0 0 0 0 0
    //  0 1 0 0 0 0 0 0
    //  0 0 1 0 0 0 0 0
    //  0 0 0 1 0 0 0 0
    //  0 0 0 0 1 0 0 0
    Bitboard(0x201008040201008),
    // idx: 31
    //  0 0 0 0 0 0 0 1
    //  0 0 0 0 0 0 0 0
    //  1 0 0 0 0 0 0 0
    //  0 1 0 0 0 0 0 0
    //  0 0 1 0 0 0 0 0
    //  0 0 0 1 0 0 0 0
    //  0 0 0 0 1 0 0 0
    //  0 0 0 0 0 1 0 0
    Bitboard(0x100804020100804),
    // idx: 32
    //  1 0 0 0 0 0 0 0
    //  0 1 0 0 0 0 0 0
    //  0 0 1 0 0 0 0 0
    //  0 0 0 1 0 0 0 0
    //  0 0 0 0 1 0 0 0
    //  0 0 0 0 0 1 0 0
    //  0 0 0 0 0 0 1 0
    //  0 0 0 0 0 0 0 0
    Bitboard(0x8040201008040200),
    // idx: 33
    //  0 1 0 0 0 0 0 0
    //  0 0 1 0 0 0 0 0
    //  0 0 0 1 0 0 0 0
    //  0 0 0 0 1 0 0 0
    //  0 0 0 0 0 1 0 0
    //  0 0 0 0 0 0 1 0
    //  0 0 0 0 0 0 0 1
    //  0 0 0 0 0 0 0 0
    Bitboard(0x4020100804020100),
    // idx: 34
    //  0 0 1 0 0 0 0 0
    //  0 0 0 1 0 0 0 0
    //  0 0 0 0 1 0 0 0
    //  0 0 0 0 0 1 0 0
    //  0 0 0 0 0 0 1 0
    //  0 0 0 0 0 0 0 1
    //  0 0 0 0 0 0 0 0
    //  1 0 0 0 0 0 0 0
    Bitboard(0x2010080402010080),
    // idx: 35
    //  0 0 0 1 0 0 0 0
    //  0 0 0 0 1 0 0 0
    //  0 0 0 0 0 1 0 0
    //  0 0 0 0 0 0 1 0
    //  0 0 0 0 0 0 0 1
    //  0 0 0 0 0 0 0 0
    //  1 0 0 0 0 0 0 0
    //  0 1 0 0 0 0 0 0
    Bitboard(0x1008040201008040),
    // idx: 36
    //  0 0 0 0 1 0 0 0
    //  0 0 0 0 0 1 0 0
    //  0 0 0 0 0 0 1 0
    //  0 0 0 0 0 0 0 1
    //  0 0 0 0 0 0 0 0
    //  1 0 0 0 0 0 0 0
    //  0 1 0 0 0 0 0 0
    //  0 0 1 0 0 0 0 0
    Bitboard(0x804020100804020),
    // idx: 37
    //  0 0 0 0 0 1 0 0
    //  0 0 0 0 0 0 1 0
    //  0 0 0 0 0 0 0 1
    //  0 0 0 0 0 0 0 0
    //  1 0 0 0 0 0 0 0
    //  0 1 0 0 0 0 0 0
    //  0 0 1 0 0 0 0 0
    //  0 0 0 1 0 0 0 0
    Bitboard(0x402010080402010),
    // idx: 38
    //  0 0 0 0 0 0 1 0
    //  0 0 0 0 0 0 0 1
    //  0 0 0 0 0 0 0 0
    //  1 0 0 0 0 0 0 0
    //  0 1 0 0 0 0 0 0
    //  0 0 1 0 0 0 0 0
    //  0 0 0 1 0 0 0 0
    //  0 0 0 0 1 0 0 0
    Bitboard(0x201008040201008),
    // idx: 39
    //  0 0 0 0 0 0 0 1
    //  0 0 0 0 0 0 0 0
    //  1 0 0 0 0 0 0 0
    //  0 1 0 0 0 0 0 0
    //  0 0 1 0 0 0 0 0
    //  0 0 0 1 0 0 0 0
    //  0 0 0 0 1 0 0 0
    //  0 0 0 0 0 1 0 0
    Bitboard(0x100804020100804),
    // idx: 40
    //  1 0 0 0 0 0 0 0
    //  0 1 0 0 0 0 0 0
    //  0 0 1 0 0 0 0 0
    //  0 0 0 1 0 0 0 0
    //  0 0 0 0 1 0 0 0
    //  0 0 0 0 0 1 0 0
    //  0 0 0 0 0 0 1 0
    //  0 0 0 0 0 0 0 0
    Bitboard(0x8040201008040200),
    // idx: 41
    //  0 1 0 0 0 0 0 0
    //  0 0 1 0 0 0 0 0
    //  0 0 0 1 0 0 0 0
    //  0 0 0 0 1 0 0 0
    //  0 0 0 0 0 1 0 0
    //  0 0 0 0 0 0 1 0
    //  0 0 0 0 0 0 0 1
    //  0 0 0 0 0 0 0 0
    Bitboard(0x4020100804020100),
    // idx: 42
    //  0 0 1 0 0 0 0 0
    //  0 0 0 1 0 0 0 0
    //  0 0 0 0 1 0 0 0
    //  0 0 0 0 0 1 0 0
    //  0 0 0 0 0 0 1 0
    //  0 0 0 0 0 0 0 1
    //  0 0 0 0 0 0 0 0
    //  1 0 0 0 0 0 0 0
    Bitboard(0x2010080402010080),
    // idx: 43
    //  0 0 0 1 0 0 0 0
    //  0 0 0 0 1 0 0 0
    //  0 0 0 0 0 1 0 0
    //  0 0 0 0 0 0 1 0
    //  0 0 0 0 0 0 0 1
    //  0 0 0 0 0 0 0 0
    //  1 0 0 0 0 0 0 0
    //  0 1 0 0 0 0 0 0
    Bitboard(0x1008040201008040),
    // idx: 44
    //  0 0 0 0 1 0 0 0
    //  0 0 0 0 0 1 0 0
    //  0 0 0 0 0 0 1 0
    //  0 0 0 0 0 0 0 1
    //  0 0 0 0 0 0 0 0
    //  1 0 0 0 0 0 0 0
    //  0 1 0 0 0 0 0 0
    //  0 0 1 0 0 0 0 0
    Bitboard(0x804020100804020),
    // idx: 45
    //  0 0 0 0 0 1 0 0
    //  0 0 0 0 0 0 1 0
    //  0 0 0 0 0 0 0 1
    //  0 0 0 0 0 0 0 0
    //  1 0 0 0 0 0 0 0
    //  0 1 0 0 0 0 0 0
    //  0 0 1 0 0 0 0 0
    //  0 0 0 1 0 0 0 0
    Bitboard(0x402010080402010),
    // idx: 46
    //  0 0 0 0 0 0 1 0
    //  0 0 0 0 0 0 0 1
    //  0 0 0 0 0 0 0 0
    //  1 0 0 0 0 0 0 0
    //  0 1 0 0 0 0 0 0
    //  0 0 1 0 0 0 0 0
    //  0 0 0 1 0 0 0 0
    //  0 0 0 0 1 0 0 0
    Bitboard(0x201008040201008),
    // idx: 47
    //  0 0 0 0 0 0 0 1
    //  0 0 0 0 0 0 0 0
    //  1 0 0 0 0 0 0 0
    //  0 1 0 0 0 0 0 0
    //  0 0 1 0 0 0 0 0
    //  0 0 0 1 0 0 0 0
    //  0 0 0 0 1 0 0 0
    //  0 0 0 0 0 1 0 0
    Bitboard(0x100804020100804),
    // idx: 48
    //  1 0 0 0 0 0 0 0
    //  0 1 0 0 0 0 0 0
    //  0 0 1 0 0 0 0 0
    //  0 0 0 1 0 0 0 0
    //  0 0 0 0 1 0 0 0
    //  0 0 0 0 0 1 0 0
    //  0 0 0 0 0 0 1 0
    //  0 0 0 0 0 0 0 0
    Bitboard(0x8040201008040200),
    // idx: 49
    //  0 1 0 0 0 0 0 0
    //  0 0 1 0 0 0 0 0
    //  0 0 0 1 0 0 0 0
    //  0 0 0 0 1 0 0 0
    //  0 0 0 0 0 1 0 0
    //  0 0 0 0 0 0 1 0
    //  0 0 0 0 0 0 0 1
    //  0 0 0 0 0 0 0 0
    Bitboard(0x4020100804020100),
    // idx: 50
    //  0 0 1 0 0 0 0 0
    //  0 0 0 1 0 0 0 0
    //  0 0 0 0 1 0 0 0
    //  0 0 0 0 0 1 0 0
    //  0 0 0 0 0 0 1 0
    //  0 0 0 0 0 0 0 1
    //  0 0 0 0 0 0 0 0
    //  1 0 0 0 0 0 0 0
    Bitboard(0x2010080402010080),
    // idx: 51
    //  0 0 0 1 0 0 0 0
    //  0 0 0 0 1 0 0 0
    //  0 0 0 0 0 1 0 0
    //  0 0 0 0 0 0 1 0
    //  0 0 0 0 0 0 0 1
    //  0 0 0 0 0 0 0 0
    //  1 0 0 0 0 0 0 0
    //  0 1 0 0 0 0 0 0
    Bitboard(0x1008040201008040),
    // idx: 52
    //  0 0 0 0 1 0 0 0
    //  0 0 0 0 0 1 0 0
    //  0 0 0 0 0 0 1 0
    //  0 0 0 0 0 0 0 1
    //  0 0 0 0 0 0 0 0
    //  1 0 0 0 0 0 0 0
    //  0 1 0 0 0 0 0 0
    //  0 0 1 0 0 0 0 0
    Bitboard(0x804020100804020),
    // idx: 53
    //  0 0 0 0 0 1 0 0
    //  0 0 0 0 0 0 1 0
    //  0 0 0 0 0 0 0 1
    //  0 0 0 0 0 0 0 0
    //  1 0 0 0 0 0 0 0
    //  0 1 0 0 0 0 0 0
    //  0 0 1 0 0 0 0 0
    //  0 0 0 1 0 0 0 0
    Bitboard(0x402010080402010),
    // idx: 54
    //  0 0 0 0 0 0 1 0
    //  0 0 0 0 0 0 0 1
    //  0 0 0 0 0 0 0 0
    //  1 0 0 0 0 0 0 0
    //  0 1 0 0 0 0 0 0
    //  0 0 1 0 0 0 0 0
    //  0 0 0 1 0 0 0 0
    //  0 0 0 0 1 0 0 0
    Bitboard(0x201008040201008),
    // idx: 55
    //  0 0 0 0 0 0 0 1
    //  0 0 0 0 0 0 0 0
    //  1 0 0 0 0 0 0 0
    //  0 1 0 0 0 0 0 0
    //  0 0 1 0 0 0 0 0
    //  0 0 0 1 0 0 0 0
    //  0 0 0 0 1 0 0 0
    //  0 0 0 0 0 1 0 0
    Bitboard(0x100804020100804),
    // idx: 56
    //  1 0 0 0 0 0 0 0
    //  0 1 0 0 0 0 0 0
    //  0 0 1 0 0 0 0 0
    //  0 0 0 1 0 0 0 0
    //  0 0 0 0 1 0 0 0
    //  0 0 0 0 0 1 0 0
    //  0 0 0 0 0 0 1 0
    //  0 0 0 0 0 0 0 0
    Bitboard(0x8040201008040200),
    // idx: 57
    //  0 1 0 0 0 0 0 0
    //  0 0 1 0 0 0 0 0
    //  0 0 0 1 0 0 0 0
    //  0 0 0 0 1 0 0 0
    //  0 0 0 0 0 1 0 0
    //  0 0 0 0 0 0 1 0
    //  0 0 0 0 0 0 0 1
    //  0 0 0 0 0 0 0 0
    Bitboard(0x4020100804020100),
    // idx: 58
    //  0 0 1 0 0 0 0 0
    //  0 0 0 1 0 0 0 0
    //  0 0 0 0 1 0 0 0
    //  0 0 0 0 0 1 0 0
    //  0 0 0 0 0 0 1 0
    //  0 0 0 0 0 0 0 1
    //  0 0 0 0 0 0 0 0
    //  1 0 0 0 0 0 0 0
    Bitboard(0x2010080402010080),
    // idx: 59
    //  0 0 0 1 0 0 0 0
    //  0 0 0 0 1 0 0 0
    //  0 0 0 0 0 1 0 0
    //  0 0 0 0 0 0 1 0
    //  0 0 0 0 0 0 0 1
    //  0 0 0 0 0 0 0 0
    //  1 0 0 0 0 0 0 0
    //  0 1 0 0 0 0 0 0
    Bitboard(0x1008040201008040),
    // idx: 60
    //  0 0 0 0 1 0 0 0
    //  0 0 0 0 0 1 0 0
    //  0 0 0 0 0 0 1 0
    //  0 0 0 0 0 0 0 1
    //  0 0 0 0 0 0 0 0
    //  1 0 0 0 0 0 0 0
    //  0 1 0 0 0 0 0 0
    //  0 0 1 0 0 0 0 0
    Bitboard(0x804020100804020),
    // idx: 61
    //  0 0 0 0 0 1 0 0
    //  0 0 0 0 0 0 1 0
    //  0 0 0 0 0 0 0 1
    //  0 0 0 0 0 0 0 0
    //  1 0 0 0 0 0 0 0
    //  0 1 0 0 0 0 0 0
    //  0 0 1 0 0 0 0 0
    //  0 0 0 1 0 0 0 0
    Bitboard(0x402010080402010),
    // idx: 62
    //  0 0 0 0 0 0 1 0
    //  0 0 0 0 0 0 0 1
    //  0 0 0 0 0 0 0 0
    //  1 0 0 0 0 0 0 0
    //  0 1 0 0 0 0 0 0
    //  0 0 1 0 0 0 0 0
    //  0 0 0 1 0 0 0 0
    //  0 0 0 0 1 0 0 0
    Bitboard(0x201008040201008),
    // idx: 63
    //  0 0 0 0 0 0 0 1
    //  0 0 0 0 0 0 0 0
    //  1 0 0 0 0 0 0 0
    //  0 1 0 0 0 0 0 0
    //  0 0 1 0 0 0 0 0
    //  0 0 0 1 0 0 0 0
    //  0 0 0 0 1 0 0 0
    //  0 0 0 0 0 1 0 0
    Bitboard(0x100804020100804),
];