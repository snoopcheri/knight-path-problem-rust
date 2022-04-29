mod lib;
use lib::prelude::*;
use crate::lib::board::Color::{BLACK, WHITE};


fn main() {
    let board = Board::default()
        .with_knight(G1, WHITE)
        .with_knight(G8, BLACK);

    println!("{}", board);
}
