use crate::{Board, from_square, Move, Square, to_square};

pub fn generate_moves(board: &Board) -> Vec<Move> {
    board.knights().iter()
        .flat_map(|square| generate_moves_for_square(board, square))
        .collect::<Vec<_>>()
}

fn generate_moves_for_square(board: &Board, from: Square) -> Vec<Move> {
    KNIGHT_DISTANCES.iter()
        .flat_map(|distance| square_in_distance(from, distance))
        .filter(|square| !board.is_occupied(*square))
        .map(|to| Move::new(from, to))
        .collect::<Vec<_>>()
}

fn square_in_distance(from: Square, distance: &Distance) -> Option<Square> {
    let mut x = from_square(from).0 as i32;
    let mut y = from_square(from).1 as i32;

    x += distance.x as i32;
    y += distance.y as i32;

    match (x, y) {
        (0..=7, 0..=7) => Some(to_square(x as u32, y as u32)),
        _ => None
    }
}

struct Distance {
    x: i8,
    y: i8,
}

const KNIGHT_DISTANCES: [Distance; 8] = [
    Distance { x: 1, y: 2 },
    Distance { x: 2, y: 1 },
    Distance { x: 2, y: -1 },
    Distance { x: 1, y: -2 },
    Distance { x: -1, y: -2 },
    Distance { x: -2, y: -1 },
    Distance { x: -2, y: 1 },
    Distance { x: -1, y: 2 },
];


#[cfg(test)]
mod tests {
    use crate::{A3, B1, B5, BLACK, C2, C3, C4, D2, E4, WHITE};
    use super::*;

    #[test]
    fn test() {
        // arrange
        let board = Board::default()
            .with_knight(B1, WHITE)
            .with_knight(A3, BLACK);

        // act
        let moves = generate_moves(&board);

        // assert
        assert_eq!(
            moves,
            vec![
                Move::new(B1, C3),
                Move::new(B1, D2),
                Move::new(A3, B5),
                Move::new(A3, C4),
                Move::new(A3, C2),
            ]
        );
    }
}
