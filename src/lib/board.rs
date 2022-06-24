use std::fmt::{Display, Formatter};
use crate::{BitBoard, BLACK, Square, to_square, WHITE};

#[derive(Copy, Clone)]
pub enum Color {
    WHITE,
    BLACK,
}

#[derive(PartialEq, PartialOrd, Eq, Ord)]
pub struct Board {
    pub white_knights: BitBoard,
    pub black_knights: BitBoard,
    // TODO: Ignoring allowed squares for the time being
}


impl Default for Board {
    fn default() -> Self {
        Board {
            white_knights: BitBoard::default(),
            black_knights: BitBoard::default(),
        }
    }
}


impl Display for Board {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        let mut str = String::new();

        for y in (0..8).rev() {
            str.push_str(&(y + 1).to_string());

            for x in 0..8 {
                str.push(' ');

                let square = to_square(x, y);

                if self.white_knights.get(square as usize) {
                    str.push('N');
                } else if self.black_knights.get(square as usize) {
                    str.push('n');
                } else {
                    str.push('.');
                }
            }

            str.push_str("\n");
        }

        str.push_str("  a b c d e f g h\n");

        write!(formatter, "{}", str)    }
}


impl Board {
    pub fn knights(&self) -> BitBoard {
        self.white_knights | self.black_knights
    }

    pub fn with_knight(&self, square: Square, color: Color) -> Board {
        match color {
            WHITE => Board { white_knights: self.white_knights.set(square), black_knights: self.black_knights },
            BLACK => Board { white_knights: self.white_knights, black_knights: self.black_knights.set(square) },
        }
    }

    pub fn is_occupied(&self, square: Square) -> bool {
        self.knights().get(square as usize)
    }
}


#[cfg(test)]
mod tests {
    use crate::lib::square::*;
    use crate::lib::board::Color::*;
    use super::*;

    #[test]
    fn with_knight() {
        // arrange
        let mut board = Board::default();

        // act + assert
        board = board.with_knight(G1, WHITE);
        assert_eq!(board.knights(), BitBoard::default().set(G1));

        // act + assert
        board = board.with_knight(G8, BLACK);
        assert_eq!(board.knights(), BitBoard::default().set(G1).set(G8));
    }
}
