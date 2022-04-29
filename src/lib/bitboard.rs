use std::fmt::{Display, Formatter, Result};

#[derive(Copy, Clone)]
pub struct BitBoard(u64);


impl BitBoard {
    #[inline]
    pub fn set_bit(&mut self, position: u32) {
        self.0 |= 1 << position;
    }


    #[inline]
    pub fn clear_bit(&mut self, position: u32) {
        self.0 &= !(1 << position);
    }


    #[inline]
    pub fn is_bit_set(&self, position: usize) -> bool {
        self.0 & (1 << position) != 0
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

                if self.is_bit_set(position) {
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
        // arrange
        let mut bb = BitBoard::default();

        // act
        bb.set_bit(0);
        bb.set_bit(1);
        bb.set_bit(60);
        bb.set_bit(63);

        // assert
        assert_eq!(bb.0, 0x9000_0000_0000_0003);
    }


    #[test]
    fn clear_bit() {
        // arrange
        let mut bb = BitBoard::default();
        bb.0 = 0xffff_ffff_ffff_ffff;

        // act
        bb.clear_bit(0);
        bb.clear_bit(1);
        bb.clear_bit(60);
        bb.clear_bit(63);

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
        let mut bb = BitBoard::default();
        bb.set_bit(0);
        bb.set_bit(42);
        bb.set_bit(63);

        // act + assert
        assert_eq!(bb.next_one(), Some(0));
        bb.clear_bit(0);
        //
        assert_eq!(bb.next_one(), Some(42));
        bb.clear_bit(42);

        assert_eq!(bb.next_one(), Some(63));
        bb.clear_bit(63);

        assert_eq!(bb.next_one(), None)
    }

}
