use std::collections::{HashSet, VecDeque};

use itertools::Itertools;

use crate::{
    api::main::{Direction, Pos, Tile},
    logic::{board::Board, tile_map::TileMap},
};

#[derive(Clone, Debug)]

pub struct Analysis {
    pub optimal_movement_count: usize,
    pub optimal_routes: Vec<Route>,
    pub suboptimal_routes: Vec<Route>,
}

impl Analysis {
    pub fn compute_fitness(&self) -> f32 {
        let good_to_bad_ratio = 1.; //            self.suboptimal_routes.len() as f32 / self.optimal_routes.len() as f32; //bigger is better

        let mut good_route_fitness = self.optimal_routes[0].fitness();

        for analysis in self.optimal_routes.iter() {
            good_route_fitness = good_route_fitness.min(analysis.fitness())
        }

        good_route_fitness * good_to_bad_ratio
    }
}

#[derive(Clone, Debug)]
pub struct Route {
    pub solution: Vec<(Direction, Pos)>,
    pub decision_positions: Vec<Pos>,
    pub move_sizes: Vec<isize>,
}

impl Route {
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
    tile_length: isize,
    path: Vec<(Direction, Pos)>,
    decision_positions: Vec<Pos>,
    visitations: HashSet<Pos>,
}

impl SearchState {
    fn step(&self, direction: Direction, board: &Board) -> (isize, Result<Self, &str>) {
        let step_start = self.path.last().unwrap().1;

        let new_step = step(&board.map, &step_start, direction);
        let step_length = (new_step.x - step_start.x).abs() + (new_step.y - step_start.y).abs();
        if self.visitations.contains(&new_step) {
            return (step_length, Err("Went into a loop"));
        }

        let mut new_path = self.path.clone();
        new_path.push((direction, new_step));

        let mut new_visitations = self.visitations.clone();
        new_visitations.insert(new_step);

        (
            step_length,
            Ok(SearchState {
                tile_length: self.tile_length + step_length,
                path: new_path,
                decision_positions: self.decision_positions.clone(),
                visitations: new_visitations,
            }),
        )
    }
}

pub fn step(map: &TileMap, start: &Pos, direction: Direction) -> Pos {
    let mut ret = start.clone();

    ret += direction.vector();

    while !map.at(ret).is_solid() {
        // TODO use canWalkInto from dart
        ret += direction.vector();
    }

    if map.at(ret) != Tile::Gate {
        ret -= direction.vector();
    }

    ret
}

const EXTRA_MOVES_SEARCH_MARGIN: usize = 1;

pub fn analyze(board: &Board) -> Option<Analysis> {
    // board.print(vec![]);

    let mut states = VecDeque::from([SearchState {
        tile_length: 0,
        path: vec![(
            board.start_direction,
            step(&board.map, &board.start, board.start_direction),
        )],
        decision_positions: vec![],
        visitations: HashSet::new(),
    }]);

    let mut solution_states = vec![];

    let mut max_movement = None;

    while let Some(state) = states.pop_front() {
        let last_dir = state.path.last().unwrap().0;
        let last_pos = state.path.last().unwrap().1;

        let potencial_directions: Vec<Direction> = [last_dir.left(), last_dir.right()]
            .into_iter()
            .filter(|dir| !board.map.at(last_pos + dir.vector()).is_solid())
            .collect();

        let mut new_states = vec![];
        let mut long_directions = 0;

        for dir in potencial_directions {
            let (step_length, new_state) = state.step(dir, board);

            let new_state = if let Ok(new_state) = new_state {
                new_state
            } else {
                continue;
            };

            if step_length > 1 {
                long_directions += 1;
            }

            if new_state.path.last().unwrap().1 == board.end {
                max_movement = Some(new_state.path.len());
                solution_states.push(new_state);
            } else {
                if let Some(max_movement) = max_movement {
                    if new_state.path.len() < (max_movement + EXTRA_MOVES_SEARCH_MARGIN) {
                        new_states.push(new_state);
                    }
                } else {
                    new_states.push(new_state);
                }
            }
        }

        for mut new_state in new_states {
            if long_directions == 2 {
                new_state.decision_positions.push(last_pos);
            }

            states.push_back(new_state);
        }
    }

    let all_routes = solution_states
        .into_iter()
        .map(state2analisis)
        .collect::<Vec<_>>();

    if let Some(max_movement) = max_movement {
        Some(Analysis {
            optimal_movement_count: max_movement,
            optimal_routes: all_routes
                .iter()
                .filter(|r| r.solution.len() == max_movement)
                .cloned()
                .collect(),
            suboptimal_routes: all_routes
                .into_iter()
                .filter(|r| r.solution.len() == max_movement)
                .collect(),
        })
    } else {
        None
    }
}

fn state2analisis(state: SearchState) -> Route {
    let mut move_sizes = vec![];

    for (start, end) in state.path.iter().tuple_windows() {
        let size = (start.1.y - end.1.y).abs() + (start.1.x - end.1.x).abs();

        move_sizes.push(size);
    }

    Route {
        move_sizes: move_sizes,
        solution: state.path,
        decision_positions: state.decision_positions,
    }
}
