use std::collections::{HashSet, VecDeque};

use itertools::Itertools;

use crate::api::main::{Board, Direction, Tile};

#[derive(Clone)]
pub struct Analysis {
    pub solution: Vec<(Direction, (isize, isize))>,
    pub search_complexity: isize,
    pub search_tile_coverage: isize,
    pub solution_tile_coverage: isize,
    pub decision_positions: Vec<(isize, isize)>,
    pub move_sizes: Vec<isize>,
    pub area: usize,
}

impl Analysis {
    pub fn fitness(&self) -> f32 {
        let move_size_mean = self.move_sizes.iter().fold(0, |acc, e| acc + e);

        let factors = [
            self.decision_positions.len() as f32 * 100000.,
            self.move_sizes.iter().filter(|e| **e > 3).count() as f32 * 10.,
            move_size_mean as f32 * 10.,
            self.solution.len() as f32,
        ];

        factors.iter().sum::<f32>()
    }
}

#[derive(Debug)]
struct SearchState {
    // score: f32,
    length: isize,
    path: Vec<(Direction, (isize, isize))>,
    decision_positions: Vec<(isize, isize)>,
}

pub fn step(
    map: &Vec<Vec<crate::api::main::Tile>>,
    start: &(isize, isize),
    direction: Direction,
) -> (isize, isize) {
    let mut ret = start.clone();

    ret.0 += direction.vector().0;
    ret.1 += direction.vector().1;

    while map[ret.1 as usize][ret.0 as usize] == Tile::Ice {
        // TODO use canWalkInto from dart
        ret.0 += direction.vector().0;
        ret.1 += direction.vector().1;
    }

    if map[ret.1 as usize][ret.0 as usize] != Tile::Gate {
        ret.0 -= direction.vector().0;
        ret.1 -= direction.vector().1;
    }

    ret
}

pub fn solve(board: &Board) -> Option<Analysis> {
    // board.print();

    let mut visitations = HashSet::new();

    let mut states = VecDeque::from([SearchState {
        length: 0,
        path: vec![(
            board.start_direction,
            step(&board.map, &board.start, board.start_direction),
        )],
        decision_positions: vec![],
    }]);
    let mut search_complexity = 0;
    let mut search_tile_coverage = 0;

    let mut try_reverse = true;

    while let Some(state) = states.pop_front() {
        let lenght = state.length;
        let path = state.path;
        let last_dir = path.last().unwrap().0;
        let last_pos = path.last().unwrap().1;

        let potencial_directions: Vec<Direction> = [
            Direction::North,
            Direction::South,
            Direction::East,
            Direction::West,
        ]
        .into_iter()
        .filter(|dir| *dir != last_dir)
        .filter(|dir| try_reverse || *dir != last_dir.reverse())
        .filter(|dir| {
            board.map[(last_pos.1 + dir.vector().1) as usize]
                [(last_pos.0 + dir.vector().0) as usize]
                == Tile::Ice
        })
        .collect();

        let mut new_states = vec![];
        let mut long_directions = 0;

        for dir in potencial_directions {
            let step_start = path.last().unwrap().1;

            let new_step = step(&board.map, &step_start, dir);
            let step_length = (new_step.0 - step_start.0).abs() + (new_step.1 - step_start.1).abs();
            if visitations.contains(&new_step) {
                continue;
            }

            if step_length > 1 {
                long_directions += 1;
            }

            let mut new_path = path.clone();
            new_path.push((dir, new_step));
            let new_length = lenght + step_length;

            if new_step == board.end {
                let mut move_sizes = vec![];

                for (start, end) in path.iter().tuple_windows() {
                    let size = (start.1 .0 - end.1 .0).abs() + (start.1 .1 - end.1 .1).abs();

                    move_sizes.push(size);
                }

                return Some(Analysis {
                    move_sizes: move_sizes,
                    solution: path,
                    search_complexity,
                    search_tile_coverage,
                    solution_tile_coverage: new_length,
                    decision_positions: state.decision_positions,
                    area: board.map.len() * board.map[0].len(),
                });
            }
            visitations.insert(new_step);

            new_states.push(SearchState {
                length: new_length,
                path: new_path,
                decision_positions: state.decision_positions.clone(),
            });
            search_complexity += 1;
            search_tile_coverage += step_length;
        }

        for mut new_state in new_states {
            if long_directions > 1 {
                new_state.decision_positions.push(last_pos);
            }

            states.push_back(new_state);
        }
        try_reverse = false;
    }

    return None;
}
