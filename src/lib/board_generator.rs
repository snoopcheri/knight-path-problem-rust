use std::time::Instant;
use crate::{BitBoardIterator, BLACK, Board, WHITE};
use crate::lib::board::Color;

pub fn generate_distinct_boards(white_knights_count: u8, black_knights_count: u8) -> Vec<Board> {
    let start = Instant::now();

    let mut boards = vec![];
    boards.push(Board::default());

    for _ in 0..white_knights_count {
        boards = boards_with_knight(&boards, WHITE);
    }

    for _ in 0..black_knights_count {
        boards = boards_with_knight(&boards, BLACK);
    }

    let total_boards = boards.len();

    boards.sort_unstable();
    boards.dedup();

    println!("Generated {} boards (#distinct={}) in {:?}", total_boards, boards.len(), start.elapsed());

    boards
}

fn boards_with_knight(boards: &Vec<Board>, color: Color) -> Vec<Board> {
    boards.iter()
        .flat_map(|board| boards_with_knight_for_board(board, color))
        .collect::<Vec<_>>()
}

fn boards_with_knight_for_board(board: &Board, color: Color) -> Vec<Board> {
    let empty_squares = BitBoardIterator::new(board.knights().toggled());

    empty_squares
        .map(|empty_square| board.with_knight(empty_square, color))
        .collect::<Vec<_>>()
}
