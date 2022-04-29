mod lib;
use lib::prelude::*;


fn main() {
    let bb = BitBoard::default()
        .set(E1)
        .set(E8);

    println!("{}", bb);
}
