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
                let successors = self.successors_of(board);
                let min_depth = self.min_depth(&successors, &solved);

                match min_depth {
                    Some(d) => {
                        debug_assert!(d == depth);
                        solved_at_depth.push(board);
                    },
                    None => {},
                };
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

    fn min_depth(&self, successors: &Vec<Board>, solved: &HashMap<&Board, u32>) -> Option<u32> {
        let mut min = None;

        for successor in successors {
            match solved.get(successor) {
                Some(&depth) => {
                    min = match min {
                        Some(current) => if current < depth { Some(current) } else { Some(depth) },
                        None => Some(depth),
                    };
                },
                None => {},
            }
        }

        min
    }

    fn mark_as_solved<'a>(&self, board: &'a Board, depth: u32, unsolved: &mut HashSet<&Board>, solved: & mut HashMap<&'a Board, u32>) {
        unsolved.remove(board);
        solved.insert(board, depth);
    }


}
