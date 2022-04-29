mod lib;
use lib::prelude::*;


fn main() {
    let mut bb = BitBoard::default();

    bb.set_bit(42);
    bb.set_bit(43);
    bb.clear_bit(42);

    println!("{}", bb);
    println!("#ones: {}", bb.count_ones());
}
