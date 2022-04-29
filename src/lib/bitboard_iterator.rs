use crate::BitBoard;

pub struct BitBoardIterator {
    bb: BitBoard
}


impl BitBoardIterator {
    pub fn new(bb: BitBoard) -> Self {
        BitBoardIterator {
            bb: bb
        }
    }
}


impl Iterator for BitBoardIterator {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let next_one = self.bb.next_one();

        match next_one {
            Some(next) => {
                self.bb = self.bb.cleared(next);
                Some(next)
            },
            _ => None
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iterator() {
        // arrange
        let bb = BitBoard::default()
            .set(0)
            .set(42)
            .set(63);

        let mut bb_iterator = BitBoardIterator::new(bb);

        assert_eq!(bb_iterator.next(), Some(0));
        assert_eq!(bb_iterator.next(), Some(42));
        assert_eq!(bb_iterator.next(), Some(63));
        assert_eq!(bb_iterator.next(), None);
    }
}
