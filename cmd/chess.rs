extern crate chess;

use chess::board::{Board, Layout};

fn main() {
    let board = Board::new(Layout::Classic);
    println!("{:?}", board);
}

