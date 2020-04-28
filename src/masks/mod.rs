pub mod a1h8;
pub mod a8h1;
pub mod files;
pub mod pawns;
pub mod ranks;
pub mod sliding;
pub mod special;

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
