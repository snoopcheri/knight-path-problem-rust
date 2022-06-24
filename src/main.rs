mod lib;

use std::collections::HashMap;
use lib::prelude::*;
use crate::lib::board::Color::{BLACK, WHITE};
use crate::lib::solver::Solver;


fn main() {
    let all_boards = generate_distinct_boards(2, 2);

    let start_board = Board {
        white_knights: BitBoard::default().set(G1).set(B1),
        black_knights: BitBoard::default().set(G8).set(B8),
    };

    let end_board = start_board.with_switched_knights();
    let solver = Solver::new(all_boards);
    let solution = solver.solve_for(&start_board, &end_board);

    show_solution(&start_board, &end_board, solution)
}

fn show_solution(from: &Board, to: &Board, solution: Option<u32>) {
    println!("From board\n{}", from);
    println!("To board\n{}", to);

    match solution {
        Some(depth) => println!("#steps required = {}", depth),
        None => println!("No path found"),
    };
}
