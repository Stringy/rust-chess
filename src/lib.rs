extern crate derive_more;
extern crate lazy_static;

pub mod bitboard;
pub mod board;
pub mod builder;
pub mod moves;

mod masks;

use itertools::Itertools;

///
/// Returns an iterator over all the files/ranks in the bitboard
/// starting at 0,0 (A1) -> 7,7 (H8)
///
/// This is returned as the cartesian product of 0..8 and 0..8
///
pub fn files_and_ranks() -> impl Iterator<Item = (usize, usize)> {
    (0..8).cartesian_product(0..8)
}

///
/// Returns an iterator over the files of the board
///
pub fn files() -> impl Iterator<Item = usize> {
    0..8
}

///
/// Returns an iterator over the ranks of the board
///
pub fn ranks() -> impl Iterator<Item = usize> {
    0..8
}
