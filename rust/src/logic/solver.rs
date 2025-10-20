use std::{collections::VecDeque, rc::Rc};

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
    pub move_sizes: Vec<isize>,
    pub hitted_boxes: usize,
    pub broken_walls: usize,
}

impl Route {
    pub fn fitness(&self) -> f32 {
        let move_size_mean = self.move_sizes.iter().fold(0, |acc, e| acc + e);

        let factors = [
            self.move_sizes.iter().filter(|e| **e > 3).count() as f32 * 10.,
            move_size_mean as f32 * 10.,
            self.solution.len() as f32,
        ];

        factors.iter().sum::<f32>()
    }
}

#[derive(Debug)]
struct PathNode {
    direction: Direction,
    position: Pos,
    last_move: Option<Rc<PathNode>>,
}

impl PathNode {
    fn into_vector(&self) -> Vec<(Direction, Pos)> {
        match &self.last_move {
            Some(last_move) => {
                let mut ret = last_move.into_vector();
                ret.push((self.direction, self.position));
                ret
            }
            None => vec![(self.direction, self.position)],
        }
    }
}

#[derive(Debug, Clone)]
struct Visitations(Vec<Vec<bool>>);

impl Visitations {
    fn new(width: isize, height: isize) -> Self {
        Visitations(vec![vec![false; width as usize]; height as usize])
    }

    fn contains(&self, p: &Pos) -> bool {
        self.0[p.y as usize][p.x as usize]
    }

    fn insert(&mut self, p: &Pos) {
        self.0[p.y as usize][p.x as usize] = true;
    }
}

#[derive(Debug)]
struct SearchState {
    // score: f32,
    board: Rc<Board>,
    tile_length: isize,
    path: Rc<PathNode>,
    path_len: usize,
    visitations: Visitations,
    broken_walls: usize,
    hitted_boxes: usize,
}

impl SearchState {
    fn step(&self, direction: Direction) -> Result<Self, &str> {
        let step_start = self.path.position;

        let new_step = step(&self.board.map, &step_start, direction);
        let step_length =
            (new_step.pos.x - step_start.x).abs() + (new_step.pos.y - step_start.y).abs();

        if self.visitations.contains(&new_step.pos) {
            return Err("Went into a loop");
        }

        // println!("path deepth is {}", self.path.len());

        let new_path = PathNode {
            direction: direction,
            position: new_step.pos,
            last_move: Some(self.path.clone()),
        };

        let mut new_board = Rc::clone(&self.board);

        let mut new_hitted_boxes = self.hitted_boxes;
        let mut new_visitations = self.visitations.clone();

        if let Tile::WeakWall = new_step.hit {
            Rc::make_mut(&mut new_board)
                .map
                .set(new_step.hit_pos, Tile::Ice);
            new_hitted_boxes += 1;
            new_visitations = Visitations::new(self.board.get_width(), self.board.get_height());
        }

        let mut new_broken_walls = self.broken_walls;

        if let Tile::Box = new_step.hit {
            Rc::make_mut(&mut new_board).box_cascade(new_step.hit_pos, direction);
            new_broken_walls += 1;
            new_visitations = Visitations::new(self.board.get_width(), self.board.get_height());
        }

        new_visitations.insert(&new_step.pos);

        Ok(SearchState {
            tile_length: self.tile_length + step_length,
            path: Rc::from(new_path),
            path_len: self.path_len + 1,
            visitations: new_visitations,
            board: new_board,
            broken_walls: new_broken_walls,
            hitted_boxes: new_hitted_boxes,
        })
    }
}

pub struct StepResult {
    pub hit: Tile,
    pub hit_pos: Pos,
    pub pos: Pos,
}

pub fn step(map: &TileMap, start: &Pos, direction: Direction) -> StepResult {
    let mut ret = start.clone();

    while !map.at(ret + direction.vector()).stops_player_during_sim() {
        ret += direction.vector();
    }

    StepResult {
        hit: map.at(ret + direction.vector()),
        hit_pos: ret + direction.vector(),
        pos: ret,
    }
}

pub fn analyze(initial_board: &Board) -> Option<Analysis> {
    // board.print(vec![]);

    let mut states = VecDeque::from([SearchState {
        tile_length: 0,
        path: Rc::from(PathNode {
            position: initial_board.start,
            direction: initial_board.start_direction,
            last_move: None,
        }),
        path_len: 1,
        visitations: Visitations::new(initial_board.get_width(), initial_board.get_height()),
        board: Rc::from(initial_board.clone()),
        broken_walls: 0,
        hitted_boxes: 0,
    }]);

    let mut solution_states = vec![vec![]; EXTRA_MOVES_SEARCH_MARGIN];

    let mut best_movement_count = None;

    while let Some(state) = states.pop_front() {
        let last_pos = state.path.position;

        println!("considering {:?}", state.path.into_vector());

        for dir in Direction::all() {
            let new_state = state.step(dir);

            let new_state = if let Ok(new_state) = new_state {
                new_state
            } else {
                continue;
            };

            if last_pos == state.board.end {
                let best_movement_count = if let Some(best_movement_count) = best_movement_count {
                    best_movement_count
                } else {
                    best_movement_count = Some(new_state.path_len);
                    new_state.path_len
                };

                solution_states[new_state.path_len - best_movement_count]
                    .push(state2analisis(new_state));
            } else {
                if let Some(best_movement_count) = best_movement_count {
                    if new_state.path_len < (best_movement_count + EXTRA_MOVES_SEARCH_MARGIN - 1) {
                        states.push_back(new_state);
                    }
                } else {
                    states.push_back(new_state);
                }
            }
        }
    }

    if let Some(max_movement) = best_movement_count {
        Some(Analysis {
            optimal_movement_count: max_movement,
            routes: solution_states,
        })
    } else {
        None
    }
}

fn state2analisis(state: SearchState) -> Route {
    let mut move_sizes = vec![];

    for (start, end) in state.path.into_vector().iter().tuple_windows() {
        let size = (start.1.y - end.1.y).abs() + (start.1.x - end.1.x).abs();

        move_sizes.push(size);
    }

    Route {
        move_sizes: move_sizes,
        solution: state.path.into_vector(),
        broken_walls: state.broken_walls,
        hitted_boxes: state.hitted_boxes,
    }
}
