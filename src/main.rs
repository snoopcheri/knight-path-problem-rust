mod lib;
use lib::prelude::*;


fn main() {
    let mut bb = BitBoard::default();

    bb.set_bit(E1);
    bb.set_bit(E8);

    println!("{}", bb);
    println!("#ones: {}", bb.count_ones());
}
