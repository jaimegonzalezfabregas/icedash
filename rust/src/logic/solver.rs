use std::{collections::VecDeque, rc::Rc};

use itertools::Itertools;

use crate::{
    api::main::{Direction, Pos, Tile},
    logic::{board::Board, tile_map::TileMap, visitations::Visitations},
};

const EXTRA_MOVES_SEARCH_MARGIN: usize = 3;
const TOP_SOLUTION_SIZE: usize = 20;

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

    pub fn print(&self){
        println!("analisis:");
    
        for (tier, routes) in self.routes.iter().enumerate(){

            println!("tier {tier}");
            for route in routes {
                print!(":");
                for (step,_) in &route.solution{
                    print!(" {}", step.icon());
                }
                println!("");    
            }
            
        }
    
    }
}

#[derive(Clone, Debug)]
pub struct Route {
    pub solution: Vec<(Direction, Pos)>,
    pub move_sizes: Vec<isize>,
    pub hitted_boxes: usize,
    pub broken_walls: usize,
    pub weakwalls_in_the_way: usize,
    pub boxes_in_the_way: usize,
}

impl Route {
    pub fn fitness(&self) -> f32 {
        let move_size_mean = self.move_sizes.iter().fold(0, |acc, e| acc + e);

        let positive_factors = [
            self.move_sizes.iter().filter(|e| **e > 3).count() as f32 * 10.,
            move_size_mean as f32 * 10.,
            self.solution.len() as f32,
        ];

        let negative_factors = [self.boxes_in_the_way as f32];

        positive_factors.iter().sum::<f32>() / (negative_factors.iter().sum::<f32>() + 1.)
    }
}

#[derive(Debug)]
enum PathNode {
    Node {
        board_change: bool,
        direction: Direction,
        position: Pos,
        last_move: Rc<PathNode>,
    },
    Root {
        root_direction: Direction,
        root_position: Pos,
    },
}

impl PathNode {
    fn into_vector(&self) -> Vec<(Direction, Pos)> {
        match &self {
            Self::Node {
                direction,
                position,
                last_move,
                ..
            } => {
                let mut ret = last_move.into_vector();
                ret.push((*direction, *position));
                ret
            }
            Self::Root { .. } => vec![],
        }
    }

    fn get_position(&self) -> Pos {
        match self {
            PathNode::Node { position, .. } => *position,
            PathNode::Root { root_position, .. } => *root_position,
        }
    }

    fn next_posible_directions(&self) -> Vec<Direction> {
        return Direction::all();
        match self {
            PathNode::Node {
                board_change,
                direction,
                ..
            } => {
                if *board_change {
                    Direction::all()
                } else {
                    vec![direction.left(), direction.right()]
                }
            }
            PathNode::Root { root_direction, .. } => vec![*root_direction],
        }
    }
}

pub fn step(map: &TileMap, start: &Pos, direction: &Direction) -> StepResult {
    let mut ret = start.clone();

    while !map
        .at(&(ret + direction.vector()))
        .stops_player_during_sim()
    {
        ret += direction.vector();
    }

    StepResult {
        hit: map.at(&(ret + direction.vector())),
        hit_pos: ret + direction.vector(),
        pos: ret,
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
    fn step(&self, direction: &Direction) -> Result<Self, &str> {
        let step_start = self.path.get_position();

        let new_step = step(&self.board.map, &step_start, direction);
        let step_length =
            (new_step.pos.x - step_start.x).abs() + (new_step.pos.y - step_start.y).abs();

        if self.visitations.contains(&new_step.pos) {
            return Err("Went into a loop");
        }

        let mut new_board = Rc::clone(&self.board);
        let mut new_board_changed = false;
        let mut new_hitted_boxes = self.hitted_boxes;
        let mut new_visitations = self.visitations.clone();

        if let Tile::WeakWall = new_step.hit {
            Rc::make_mut(&mut new_board)
                .map
                .set(&new_step.hit_pos, Tile::Ice);
            new_hitted_boxes += 1;
            new_visitations = Visitations::new(self.board.get_width(), self.board.get_height());
            new_board_changed = true;
        }

        let mut new_broken_walls = self.broken_walls;

        if let Tile::Box = new_step.hit {
            Rc::make_mut(&mut new_board).box_cascade(&new_step.hit_pos, direction);
            new_broken_walls += 1;
            new_visitations = Visitations::new(self.board.get_width(), self.board.get_height());
            new_board_changed = true;
        }

        let new_path = PathNode::Node {
            direction: direction.to_owned(),
            position: new_step.pos,
            last_move: self.path.clone(),
            board_change: new_board_changed,
        };

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

pub fn analyze(initial_board: &Board) -> Result<Analysis, String> {
    // board.print(vec![]);

    let mut initial_visitations =
        Visitations::new(initial_board.get_width(), initial_board.get_height());
    initial_visitations.insert(&initial_board.start);

    let mut states = VecDeque::from([SearchState {
        tile_length: 0,
        path: Rc::from(PathNode::Root {
            root_direction: initial_board.start_direction,
            root_position: initial_board.start,
        }),
        path_len: 0,
        visitations: initial_visitations,
        board: Rc::from(initial_board.clone()),
        broken_walls: 0,
        hitted_boxes: 0,
    }]);

    let mut solution_states = vec![vec![]; EXTRA_MOVES_SEARCH_MARGIN];

    let mut best_movement_count = None;

    while let Some(state) = states.pop_front() {
        for dir in state.path.next_posible_directions() {
            let new_state = state.step(&dir);

            let new_state = if let Ok(new_state) = new_state {
                new_state
            } else {
                continue;
            };

            if new_state.path.get_position() == state.board.end {
                let best_movement_count = if let Some(best_movement_count) = best_movement_count {
                    best_movement_count
                } else {
                    best_movement_count = Some(new_state.path_len);

                    new_state.path_len
                };

                solution_states[new_state.path_len - best_movement_count]
                    .push(state2analysis(new_state, &initial_board.map));
            } else {
                if let Some(best_movement_count) = best_movement_count {
                    if new_state.path_len < (best_movement_count + EXTRA_MOVES_SEARCH_MARGIN - 1) {
                        states.push_back(new_state);
                    }
                } else if TOP_SOLUTION_SIZE > new_state.path_len {
                    states.push_back(new_state);
                }
            }
        }
    }

    if let Some(max_movement) = best_movement_count {
        Ok(Analysis {
            optimal_movement_count: max_movement,
            routes: solution_states,
        })
    } else {
        Err(String::from("Unsolvable room"))
    }
}

fn state2analysis(state: SearchState, starting_tilemap: &TileMap) -> Route {
    let mut move_sizes = vec![];
    let mut weakwalls_in_the_way = 0;
    let mut boxes_in_the_way = 0;

    for (start, end) in state.path.into_vector().iter().tuple_windows() {
        let size = (start.1.y - end.1.y).abs() + (start.1.x - end.1.x).abs();

        move_sizes.push(size);
    }

    for (start, end) in state.path.into_vector().iter().tuple_windows() {
        if start.0 == end.0 {
            if starting_tilemap.at(&(start.1 + end.0.vector())) == Tile::WeakWall {
                weakwalls_in_the_way += 1;
            }
            if starting_tilemap.at(&(start.1 + end.0.vector())) == Tile::Box {
                boxes_in_the_way += 1;
            }
        }
    }

    Route {
        move_sizes: move_sizes,
        solution: state.path.into_vector(),
        broken_walls: state.broken_walls,
        hitted_boxes: state.hitted_boxes,
        weakwalls_in_the_way,
        boxes_in_the_way,
    }
}
