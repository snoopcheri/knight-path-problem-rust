use std::fmt::{Display, Formatter, Result};

#[derive(Copy, Clone)]
pub struct BitBoard(u64);


impl BitBoard {
    pub fn new(val: u64) -> Self {
        BitBoard(val)
    }

    #[inline]
    pub fn set(&self, bit: u32) -> BitBoard {
        BitBoard(self.0 | 1 << bit)
    }


    #[inline]
    pub fn cleared(&self, bit: u32) -> BitBoard {
        BitBoard(self.0 & !(1 << bit))
    }


    #[inline]
    pub fn get(&self, bit: usize) -> bool {
        self.0 & (1 << bit) != 0
    }


    #[inline]
    pub fn count_ones(&self) -> u32 {
        self.0.count_ones()
    }


    #[inline]
    pub fn next_one(&self) -> Option<u32> {
        let trailing_zeros = self.0.trailing_zeros();

        match trailing_zeros {
            0..=63 => Some(trailing_zeros),
            _ => None
        }
    }
}

impl Default for BitBoard {
    fn default() -> Self {
        BitBoard(0)
    }
}


impl Display for BitBoard {
    fn fmt(&self, formatter: &mut Formatter) -> Result {
        let mut str = String::new();

        for y in (0..8).rev() {
            str.push_str(&(y + 1).to_string());

            for x in 0..8 {
                str.push(' ');

                let position = x + (y * 8);

                if self.get(position) {
                    str.push('*');
                } else {
                    str.push('.');
                }
            }

            str.push_str("\n");
        }

        str.push_str("  a b c d e f g h\n");

        write!(formatter, "{}", str)
    }
}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn set_bit() {
        // arrange + act
        let bb = BitBoard::default()
            .set(0)
            .set(1)
            .set(60)
            .set(63);

        // assert
        assert_eq!(bb.0, 0x9000_0000_0000_0003);
    }


    #[test]
    fn clear_bit() {
        // arrange + act
        let bb = BitBoard::new(0xffff_ffff_ffff_ffff)
            .cleared(0)
            .cleared(1)
            .cleared(60)
            .cleared(63);

        // assert
        assert_eq!(bb.0, 0x6fff_ffff_ffff_fffc)
    }


    #[test]
    fn count_ones() {
        // arrange
        let mut bb = BitBoard::default();
        bb.0 = 42;

        // act
        let ones = bb.count_ones();

        // assert
        assert_eq!(ones, 3);
    }

    #[test]
    fn next_one() {
        // arrange
        let mut bb = BitBoard::default()
            .set(0)
            .set(42)
            .set(63);

        // act + assert
        assert_eq!(bb.next_one(), Some(0));
        bb = bb.cleared(0);
        //
        assert_eq!(bb.next_one(), Some(42));
        bb = bb.cleared(42);

        assert_eq!(bb.next_one(), Some(63));
        bb = bb.cleared(63);

        assert_eq!(bb.next_one(), None)
    }

}
