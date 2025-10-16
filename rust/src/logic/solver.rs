use std::{
    collections::{HashSet, VecDeque},
    hash::{DefaultHasher, Hash, Hasher},
};

use itertools::Itertools;

use crate::{
    api::main::{Direction, Pos, Tile},
    logic::{board::Board, tile_map::TileMap},
};

const EXTRA_MOVES_SEARCH_MARGIN: usize = 3;

#[derive(Clone, Debug)]

pub struct Analysis {
    pub optimal_movement_count: usize,
    pub routes: Vec<Vec<Route>>,
}

impl Analysis {
    pub fn compute_fitness(&self) -> f32 {
        let mut good_route_fitness = self.routes[0][0].fitness();

        for analysis in self.routes[0].iter() {
            good_route_fitness = good_route_fitness.min(analysis.fitness())
        }

        let solution_distribution = (self.routes[1].len() as f32 + self.routes[2].len() as f32)
            / (1. + self.routes[0].len() as f32);

        let ret = good_route_fitness * solution_distribution;

        if ret != 0. {

            // println!("ret: {ret} ({}+{})/(1+{})",self.routes[1].len(),self.routes[2].len(),self.routes[0].len());
        };
        ret
    }
}

#[derive(Clone, Debug)]
pub struct Route {
    pub solution: Vec<(Direction, Pos)>,
    pub decision_positions: Vec<Pos>,
    pub move_sizes: Vec<isize>,
    pub hitted_boxes: usize,
    pub broken_walls: usize,
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
    board: Board,
    tile_length: isize,
    path: Vec<(Direction, Pos)>,
    decision_positions: Vec<Pos>,
    visitations: HashSet<Pos>,
    broken_walls: usize,
    hitted_boxes: usize,
}

impl SearchState {
    fn step(&self, direction: Direction, board: &Board) -> (isize, Result<Self, &str>) {
        let step_start = self.path.last().unwrap().1;

        let new_step = step(&board.map, &step_start, direction);
        let step_length =
            (new_step.pos.x - step_start.x).abs() + (new_step.pos.y - step_start.y).abs();

        if self.visitations.contains(&new_step.pos) {
            return (step_length, Err("Went into a loop"));
        }

        let mut new_path = self.path.clone();
        new_path.push((direction, new_step.pos));

        let mut new_board = board.to_owned();

        let mut new_hitted_boxes = self.hitted_boxes;

        if let Tile::WeakWall(hits) = new_step.hit {
            if hits - 1 == 0 {
                new_board.map.set(new_step.hit_pos, Tile::Ice);
                new_hitted_boxes += 1;
            } else {
                new_board
                    .map
                    .set(new_step.hit_pos, Tile::WeakWall(hits - 1));
            }
        }
        let mut new_broken_walls = self.broken_walls;

        if let Tile::Box = new_step.hit {
            new_board.box_cascade(new_step.hit_pos, direction);
            new_broken_walls += 1;
        }

        let mut new_visitations = self.visitations.clone();

        new_visitations.insert(new_step.pos);

        (
            step_length,
            Ok(SearchState {
                tile_length: self.tile_length + step_length,
                path: new_path,
                decision_positions: self.decision_positions.clone(),
                visitations: new_visitations,
                board: new_board,
                broken_walls: new_broken_walls,
                hitted_boxes: new_hitted_boxes,
            }),
        )
    }
}

pub struct StepResult {
    pub hit: Tile,
    pub hit_pos: Pos,
    pub pos: Pos,
}

pub fn step(map: &TileMap, start: &Pos, direction: Direction) -> StepResult {
    let mut ret = start.clone();

    ret += direction.vector();

    while !map.at(ret).is_solid() {
        ret += direction.vector();
    }

    if map.at(ret) != Tile::Gate {
        ret -= direction.vector();
    }

    StepResult {
        hit: map.at(ret),
        hit_pos: ret + direction.vector(),
        pos: ret,
    }
}

pub fn analyze(board: &Board) -> Option<Analysis> {
    // board.print(vec![]);

    let entering_animation = step(&board.map, &board.start, board.start_direction);

    let mut states = VecDeque::from([SearchState {
        tile_length: 0,
        path: vec![(board.start_direction, entering_animation.pos)],
        decision_positions: vec![],
        visitations: HashSet::new(),
        board: board.to_owned(),
        broken_walls: 0,
        hitted_boxes: 0,
    }]);

    let mut solution_states = vec![vec![]; EXTRA_MOVES_SEARCH_MARGIN];

    let mut best_movement_count = None;

    while let Some(state) = states.pop_front() {
        let last_dir = state.path.last().unwrap().0;
        let last_pos = state.path.last().unwrap().1;

        let potencial_directions: Vec<Direction> = [last_dir.left(), last_dir.right()]
            .into_iter()
            .filter(|dir| !state.board.map.at(last_pos + dir.vector()).is_solid())
            .collect();

        let mut new_states = vec![];
        let mut long_directions = 0;

        for dir in potencial_directions {
            let (step_length, new_state) = state.step(dir, &state.board);

            let new_state = if let Ok(new_state) = new_state {
                new_state
            } else {
                continue;
            };

            if step_length > 1 {
                long_directions += 1;
            }

            if new_state.path.last().unwrap().1 == state.board.end {
                let best_movement_count = if let Some(best_movement_count) = best_movement_count {
                    best_movement_count
                } else {
                    best_movement_count = Some(new_state.path.len());
                    new_state.path.len()
                };

                solution_states[new_state.path.len() - best_movement_count]
                    .push(state2analisis(new_state));
            } else {
                if let Some(max_movement) = best_movement_count {
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

    let all_routes = solution_states;

    if let Some(max_movement) = best_movement_count {
        Some(Analysis {
            optimal_movement_count: max_movement,
            routes: all_routes,
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
        broken_walls: state.broken_walls,
        hitted_boxes: state.hitted_boxes,
    }
}
