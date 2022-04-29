use std::fmt::{Display, Formatter, Result};

pub struct BitBoard(u64);


impl BitBoard {
    #[inline]
    pub fn set_bit(&mut self, position: usize) {
        self.0 |= 1 << position;
    }


    #[inline]
    pub fn clear_bit(&mut self, position: usize) {
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
            str.push_str(&y.to_string());

            for x in 0..8 {
                str.push(' ');

                let position = (x * 8) + y;

                if self.is_bit_set(position) {
                    str.push('*');
                } else {
                    str.push('.');
                }
            }

            str.push_str("\n");
        }

        str.push_str(" 0 1 2 3 4 5 6 7\n");

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
}
