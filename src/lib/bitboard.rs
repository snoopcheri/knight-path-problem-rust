use std::fmt::{Display, Formatter, Result};
use std::ops;
use crate::BitBoardIterator;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct BitBoard(u64);

impl ops::BitOr for BitBoard {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitBoard {
    pub fn new(val: u64) -> Self {
        BitBoard(val)
    }

    #[inline]
    pub fn get(&self, bit: usize) -> bool {
        self.0 & (1 << bit) != 0
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
    pub fn count_ones(&self) -> u32 {
        self.0.count_ones()
    }

    #[inline]
    pub fn toggled(&self) -> BitBoard {
        BitBoard(self.0 ^ u64::MAX)
    }

    #[inline]
    pub fn next_one(&self) -> Option<u32> {
        let trailing_zeros = self.0.trailing_zeros();

        match trailing_zeros {
            0..=63 => Some(trailing_zeros),
            _ => None
        }
    }
    
    pub fn iter(&self) -> BitBoardIterator {
        BitBoardIterator::new(*self)
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
    fn bit_or() {
        // arrange
        let bb1 = BitBoard::default().set(4);
        let bb2 = BitBoard::default().set(7);

        // act
        let bb = bb1 | bb2;

        // assert
        assert_eq!(bb.get(4), true);
        assert_eq!(bb.get(7), true);
    }

    #[test]
    fn count_ones() {
        // arrange
        let bb = BitBoard::new(42);

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

    #[test]
    fn toggled() {
        // arrange
        let board = BitBoard::default()
            .set(0)
            .set(42)
            .set(63);

        // act
        let toggled = board.toggled();

        // assert
        for bit in 0..63 {
            if bit == 0 || bit == 42 || bit == 63 {
                assert_eq!(toggled.get(bit), false, "Bit {} should not be set", bit)
            } else {
                assert_eq!(toggled.get(bit), true, "Bit {} should be set", bit)
            }
        }
    }
}
