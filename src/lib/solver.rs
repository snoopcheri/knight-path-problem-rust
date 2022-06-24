use std::collections::{HashMap, HashSet};
use std::time::Instant;
use crate::{Board, generate_moves};

pub struct Solver {
    // TODO Store reference only?
    boards: Vec<Board>,
}

impl Solver {

    pub fn new(boards: Vec<Board>) -> Self {
        Solver {
            boards: boards
        }
    }

    pub fn solve_for(&self, start_board: &Board, end_board: &Board) -> Option<u32> {
        println!("Solving for board\n{}", end_board);

        let mut unsolved = HashSet::<_>::from_iter(&self.boards);
        let mut solved = HashMap::<&Board, u32>::new();
        let mut depth = 0;

        self.mark_as_solved(&end_board, 0, &mut unsolved, &mut solved);

        loop {
            let start = Instant::now();
            let mut solved_at_depth = vec![];

            print!("depth={}, #unsolved={}, #solved={}", depth, unsolved.len(), solved.len());

            for &board in &unsolved {
                self.successors_of(board).iter()
                    .flat_map(|successor| solved.get(successor))
                    .min()
                    .map(|_depth| solved_at_depth.push(board));
            }

            println!(" -> found {} new solutions in {:?}", solved_at_depth.len(), start.elapsed());

            if solved_at_depth.is_empty() {
                break;
            }

            for board in solved_at_depth {
                self.mark_as_solved(board, depth + 1, &mut unsolved, &mut solved);
            }

            depth = depth + 1;
        }

        match solved.get(start_board) {
            Some(&depth) => Some(depth),
            None => None,
        }
    }

    fn successors_of(&self, board: &Board) -> Vec<Board> {
        generate_moves(board).iter()
            .map(|mv| board.after_move(mv))
            .collect::<Vec<_>>()
    }

    fn mark_as_solved<'a>(&self, board: &'a Board, depth: u32, unsolved: &mut HashSet<&Board>, solved: & mut HashMap<&'a Board, u32>) {
        unsolved.remove(board);
        solved.insert(board, depth);
    }


}
