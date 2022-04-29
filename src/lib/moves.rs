use std::fmt::{Display, Formatter};
use crate::{Square, square_to_name};

#[derive(Debug, PartialEq)]
pub struct Move {
    pub from: Square,
    pub to: Square,
}


impl Display for Move {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        let from = square_to_name(self.from).to_lowercase();
        let to = square_to_name(self.to).to_lowercase();

        write!(formatter, "{}-{}", from, to)
    }
}


impl Move {
    pub fn new(from: Square, to: Square) -> Self {
        Move { from, to }
    }
}


#[cfg(test)]
mod tests {
    use crate::{E2, E4};
    use super::*;

    #[test]
    fn display() {
        assert_eq!(Move::new(E2, E4).to_string(), "e2-e4");
    }
}
