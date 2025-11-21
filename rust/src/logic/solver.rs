use std::{collections::VecDeque, rc::Rc};

use itertools::Itertools;

use crate::{
    api::{
        board_description::{ BoardDescription, GameMode},
        direction::Direction,
        pos::Pos,
        tile::{ Tile},
    },
    logic::{board::Board, matrix::TileMap, visitations::Visitations},
};

const EXTRA_MOVES_SEARCH_MARGIN: usize = 3;
const TOP_SOLUTION_SIZE: usize = 10;

#[derive(Clone, Debug)]

pub struct Analysis {
    pub optimal_movement_count: usize,
    pub routes: Vec<Vec<Route>>,
}

impl Analysis {
    pub fn compute_fitness(&self, tile_map: &TileMap, board_description: &BoardDescription) -> f32 {
        if let GameMode::FindPerfectPath = board_description.game_mode {
            let mut good_route_fitness = self.routes[0][0].prefect_path_fitness(tile_map);

            for analysis in self.routes[0].iter() {
                good_route_fitness = good_route_fitness.min(analysis.prefect_path_fitness(tile_map))
            }

            let solution_distribution = (self.routes[1].len() as f32 + self.routes[2].len() as f32)
                / (1. + self.routes[0].len() as f32);

            let ret = good_route_fitness * solution_distribution;

            ret
        } else {
            let mut solution_count = 0.;
            let mut min_fitness = self.routes[0][0].any_path_fitness(tile_map);

            for same_lenght_routes in self.routes.iter() {
                for route in same_lenght_routes.iter() {
                    solution_count += 1.;
                    min_fitness = min_fitness.min(route.any_path_fitness(tile_map));
                }
            }

            let ret = min_fitness / solution_count;

            ret
        }
    }

    pub fn check_still_applies(&self, board: &Board, initial_gate_id: usize) -> bool {
        self.routes
            .iter()
            .flat_map(|e| e)
            .all(|route| route.solves(board, initial_gate_id))
    }

    pub fn print(&self) {
        println!("analisis:");

        for (tier, routes) in self.routes.iter().enumerate() {
            println!("tier {tier}");
            for route in routes {
                print!(":");
                for (step, _) in &route.solution {
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
}

impl Route {
    pub fn prefect_path_fitness(&self, tile_map: &TileMap) -> f32 {
        let mut move_sizes = vec![];

        for (start, end) in self.solution.iter().tuple_windows() {
            let size = (start.1.y - end.1.y).abs() + (start.1.x - end.1.x).abs();

            move_sizes.push(size);
        }

        let move_size_mean = move_sizes.iter().fold(0, |acc, e| acc + e);

        let mut boxes_in_the_way = 0;
        let mut weakwalls_in_the_way = 0;
        for (start, end) in self.solution.iter().tuple_windows() {
            if start.0 == end.0 {
                if tile_map.at(&(start.1 + end.0.vector())) == Tile::WeakWall {
                    weakwalls_in_the_way += 1;
                }
                if tile_map.at(&(start.1 + end.0.vector())) == Tile::Box {
                    boxes_in_the_way += 1;
                }
            }
        }

        let mut decision_positions = 0;
        for (start, end) in self.solution.iter().tuple_windows() {
            if !tile_map
                .at(&(start.1 + end.0.right().vector()))
                .stops_player_during_gameplay(false)
                && !tile_map
                    .at(&(start.1 + end.0.right().vector() * 2))
                    .stops_player_during_gameplay(false)
            {
                decision_positions += 1;
            }
            if !tile_map
                .at(&(start.1 + end.0.left().vector()))
                .stops_player_during_gameplay(false)
                && !tile_map
                    .at(&(start.1 + end.0.left().vector() * 2))
                    .stops_player_during_gameplay(false)
            {
                decision_positions += 1;
            }
        }

        let positive_factors = [
            decision_positions as f32 * 100.,
            move_sizes.iter().filter(|e| **e > 3).count() as f32 * 10.,
            move_size_mean as f32 * 10.,
            self.solution.len() as f32,
        ];

        let negative_factors = [weakwalls_in_the_way as f32, boxes_in_the_way as f32];

        positive_factors.iter().sum::<f32>() / (negative_factors.iter().sum::<f32>() + 1.)
    }

    pub fn any_path_fitness(&self, tile_map: &TileMap) -> f32 {

        let mut decision_positions = 0;
        for (start, end) in self.solution.iter().tuple_windows() {
            if !tile_map
                .at(&(start.1 + end.0.right().vector()))
                .stops_player_during_gameplay(false)
                && !tile_map
                    .at(&(start.1 + end.0.right().vector() * 2))
                    .stops_player_during_gameplay(false)
            {
                decision_positions += 1;
            }
            if !tile_map
                .at(&(start.1 + end.0.left().vector()))
                .stops_player_during_gameplay(false)
                && !tile_map
                    .at(&(start.1 + end.0.left().vector() * 2))
                    .stops_player_during_gameplay(false)
            {
                decision_positions += 1;
            }
        }

        let positive_factors = [
            decision_positions as f32 * 100.,
            self.solution.len() as f32,
        ];


        positive_factors.iter().sum::<f32>()
    }

    fn solves(&self, board: &Board, initial_gate_id: usize) -> bool {
        // println!("-----");
        // println!("check if {self:?} solves ");

        // board.print(vec![]);

        let mut cursor = SearchState {
            tile_length: 0,
            path: Rc::from(PathNode::Root {
                root_direction: board.get_gate_direction(initial_gate_id),
                root_position: board.get_gate_position(initial_gate_id),
            }),
            path_len: 0,
            visitations: Visitations::new(board.get_width(), board.get_height()),
            board: Rc::from(board.clone()),
            broken_walls: 0,
            hitted_boxes: 0,
        };

        for (direction, _) in self.solution.iter() {
            // println!("moving {:?}", board.get_gate_direction(initial_gate_id));

            cursor = match cursor.step(direction) {
                Ok(new_cursor) => {
                    // println!("new cursor is {new_cursor:?}");

                    new_cursor
                }
                Err(_) => {
                    return false;
                }
            };
        }

        if self.solution.last().unwrap().1 != cursor.path.get_position() {
            // println!("position desync {pos:?} {:?}", cursor.path.get_position());
            return false;
        }

        true
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
        ret = ret + direction.vector();
    }

    StepResult {
        hit: map.at(&(ret + direction.vector())),
        hit_pos: ret + direction.vector(),
        pos: ret,
    }
}

#[derive(Debug)]
struct SearchState {
    board: Rc<Board>,
    tile_length: isize,
    path: Rc<PathNode>,
    path_len: usize,
    visitations: Visitations,
    broken_walls: usize,
    hitted_boxes: usize,
}

impl SearchState {
    fn step(&self, direction: &Direction) -> Result<Self, String> {
        let step_start = self.path.get_position();

        let new_step = step(&self.board.map, &step_start, direction);
        let step_length =
            (new_step.pos.x - step_start.x).abs() + (new_step.pos.y - step_start.y).abs();

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

        if new_visitations.contains(&new_step.pos) {
            return Err(format!("Went into a loop at {:?}", &new_step.pos));
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

pub fn analyze(
    initial_board: &Board,
    entry_gate_id: usize,
    exit_gate_id: usize,
) -> Result<Analysis, String> {
    // board.print(vec![]);

    let mut initial_visitations =
        Visitations::new(initial_board.get_width(), initial_board.get_height());
    initial_visitations.insert(&initial_board.get_gate_position(entry_gate_id));

    let mut states = VecDeque::from([SearchState {
        tile_length: 0,
        path: Rc::from(PathNode::Root {
            root_direction: initial_board.get_gate_direction(entry_gate_id),
            root_position: initial_board.get_gate_position(entry_gate_id),
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
        // println!("considering {:?}", state.path.into_vector());
        for dir in state.path.next_posible_directions() {
            let new_state = state.step(&dir);

            let new_state = if let Ok(new_state) = new_state {
                new_state
            } else {
                // println!("{:?} going to {dir:?}", new_state);
                continue;
            };

            if new_state.path.get_position() == state.board.get_gate_position(exit_gate_id) {
                let best_movement_count = if let Some(best_movement_count) = best_movement_count {
                    best_movement_count
                } else {
                    best_movement_count = Some(new_state.path_len);

                    new_state.path_len
                };

                solution_states[new_state.path_len - best_movement_count]
                    .push(state2analysis(new_state));
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

fn state2analysis(state: SearchState) -> Route {
    Route {
        solution: state.path.into_vector(),
    }
}
