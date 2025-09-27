use std::{
    collections::{HashSet, VecDeque},
    time::{Duration, Instant},
};

use rand::{
    rng,
    seq::{IndexedRandom, IteratorRandom},
    Rng,
};
use sorted_vec::partial::ReverseSortedVec;

#[derive(Clone, PartialEq, Copy, Debug)]
pub enum Tile {
    Entrance,
    Gate,
    Wall,
    Ice,
    Ground,
    Outside,
}

impl Tile {
    fn simbol(&self) -> &str {
        match self {
            Tile::Entrance => "E",
            Tile::Gate => "G",
            Tile::Wall => "#",
            Tile::Ice => " ",
            Tile::Ground => ".",
            Tile::Outside => " ",
        }
    }
}

#[derive(Clone, PartialEq, Copy, Debug)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    #[flutter_rust_bridge::frb(sync)] // Synchronous mode for simplicity of the demo
    pub fn vector(&self) -> (isize, isize) {
        match self {
            Direction::North => (0, -1),
            Direction::South => (0, 1),
            Direction::East => (-1, 0),
            Direction::West => (1, 0),
        }
    }

    pub fn reverse(&self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }
}

#[derive(Clone)]
pub struct Analysis {
    solution: Vec<(Direction, (isize, isize))>,
    search_complexity: isize,
    search_tile_coverage: isize,
    solution_tile_coverage: isize,
    decision_positions: Vec<(isize, isize)>,
}

impl Analysis {
    fn fitness(&self) -> f32 {
        (self.decision_positions.len() * 10) as f32 + self.solution.len() as f32
    }
}

#[derive(Clone)]
pub struct Board {
    pub map: Vec<Vec<Tile>>,
    pub start: (isize, isize),
    pub end: (isize, isize),
    pub start_direction: Direction,
    pub end_direction: Direction,
    pub area: isize,
}
impl Board {
    fn mutate(&self, factor: f32) -> Board {
        let mut rng = rand::rng();

        let mut ret = self.clone();
        let height = ret.map.len();
        let width = ret.map[0].len();

        for y in 1..height - 1 {
            for x in 1..width - 1 {
                if rng.random::<f32>() < factor {
                    ret.map[y as usize][x as usize] = Tile::Ice;
                }
            }
        }

        board_cleanup(
            &mut ret.map,
            ret.start,
            ret.start_direction,
            ret.end,
            ret.end_direction,
        );

        ret
    }
}

#[derive(Clone)]
struct Creature {
    board: Board,
    analisis: Analysis,
    fitness: f32,
    mutation_count: usize,
}
impl Creature {
    fn new(b: Board) -> Option<Self> {
        let analysis = solve(&b);
        if let Some(analysis) = analysis {
            Some(Self {
                board: b,
                fitness: analysis.fitness(),
                analisis: analysis,
                mutation_count: 0,
            })
        } else {
            None
        }
    }

    fn mutate(&self, factor: f32) -> Option<Self> {
        Self::new(self.board.mutate(factor)).map(|mut ret| {
            ret.mutation_count = self.mutation_count + 1;
            ret
        })
    }
}

impl PartialOrd for Creature {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.fitness.partial_cmp(&other.fitness)
    }
}

impl PartialEq for Creature {
    fn eq(&self, other: &Self) -> bool {
        self.fitness == other.fitness
    }
}

fn get_random_board() -> Board {
    let mut rng = rand::rng();
    let width = (5..20).choose(&mut rng).unwrap();
    let height = (5..20).choose(&mut rng).unwrap();
    let clutterness = 0.25 + rng.random::<f32>() * 0.2;

    let start_side = (0..3).choose(&mut rng).unwrap();
    let end_side = ((1..3).choose(&mut rng).unwrap() + start_side) % 4;

    let (start, start_direction) = match start_side {
        0 => (
            (0, (2..height - 2).choose(&mut rng).unwrap()),
            Direction::West,
        ),
        1 => (
            (width - 1, (2..height - 2).choose(&mut rng).unwrap()),
            Direction::East,
        ),
        2 => (
            ((2..width - 2).choose(&mut rng).unwrap(), 0),
            Direction::South,
        ),
        _ => (
            ((2..width - 2).choose(&mut rng).unwrap(), height - 1),
            Direction::North,
        ),
    };

    let (end, end_direction) = match end_side {
        0 => (
            (0, (2..height - 2).choose(&mut rng).unwrap()),
            Direction::West,
        ),
        1 => (
            (width - 1, (2..height - 2).choose(&mut rng).unwrap()),
            Direction::East,
        ),
        2 => (
            ((2..width - 2).choose(&mut rng).unwrap(), 0),
            Direction::South,
        ),
        _ => (
            ((2..width - 2).choose(&mut rng).unwrap(), height - 1),
            Direction::North,
        ),
    };

    let mut ret = Board {
        map: vec![vec![Tile::Wall; width as usize]; height as usize],
        start,
        start_direction,
        end,
        end_direction,
        area: width * height,
    };

    for y in 1..height - 1 {
        for x in 1..width - 1 {
            if rng.random::<f32>() > clutterness {
                ret.map[y as usize][x as usize] = Tile::Ice;
            }
        }
    }

    board_cleanup(&mut ret.map, start, start_direction, end, end_direction);

    ret
}

fn board_cleanup(
    ret: &mut Vec<Vec<Tile>>,
    start: (isize, isize),
    start_direction: Direction,
    end: (isize, isize),
    end_direction: Direction,
) {
    let mut rng = rand::rng();

    let width = ret[0].len() as isize;
    let height = ret.len() as isize;

    if start.0 == end.0 || start.1 == end.1 {
        ret[((start.1 + end.1) / 2) as usize][((start.0 + end.0) / 2) as usize] = Tile::Wall;
    }

    let mut rep = true;
    while rep {
        rep = false;
        for y in 1..height - 2 {
            for x in 1..width - 2 {
                let a = ret[(y) as usize][(x) as usize];
                let b = ret[(y + 1) as usize][(x) as usize];
                let c = ret[(y) as usize][(x + 1) as usize];
                let d = ret[(y + 1) as usize][(x + 1) as usize];

                let cuad = (a, b, c, d);

                match cuad {
                    (Tile::Ice, Tile::Wall, Tile::Wall, Tile::Ice) => {
                        rep = true;
                        if *([true, false].choose(&mut rng).unwrap()) {
                            ret[(y) as usize][(x) as usize] = Tile::Wall;
                        } else {
                            ret[(y + 1) as usize][(x + 1) as usize] = Tile::Wall;
                        }
                    }
                    (Tile::Wall, Tile::Ice, Tile::Ice, Tile::Wall) => {
                        rep = true;
                        if *([true, false].choose(&mut rng).unwrap()) {
                            ret[(y + 1) as usize][(x) as usize] = Tile::Wall;
                        } else {
                            ret[(y) as usize][(x + 1) as usize] = Tile::Wall;
                        }
                    }

                    _ => {}
                }
            }
        }
    }

    let mut rep = true;
    while rep {
        rep = false;
        for y in 1..height - 1 {
            for x in 1..width - 1 {
                if ret[(y) as usize][(x) as usize] != Tile::Wall {
                    let mut neigh_count = 0;
                    for (dx, dy) in [
                        (0, 1),
                        (0, -1),
                        (-1, 0),
                        (1, 0),
                        (1, 1),
                        (1, -1),
                        (-1, -1),
                        (-1, 1),
                    ] {
                        let neigh = ret[(y + dy) as usize][(x + dx) as usize];
                        if neigh == Tile::Wall {
                            neigh_count += 1;
                        }
                    }

                    if neigh_count >= 6 {
                        ret[(y) as usize][(x) as usize] = Tile::Wall;
                        rep = true;
                    }
                }
            }
        }
    }

    ret[start.1 as usize][start.0 as usize] = Tile::Entrance;
    ret[(start.1 + start_direction.vector().1) as usize]
        [(start.0 + start_direction.vector().0) as usize] = Tile::Ice;

    let (dx, dy) = end_direction.vector();

    ret[end.1 as usize][end.0 as usize] = Tile::Gate;
    ret[(end.1 + dy) as usize][(end.0 + dx) as usize] = Tile::Ice;
    ret[(end.1 + dy + dx) as usize][(end.0 + dx - dy) as usize] = Tile::Ice;
    ret[(end.1 + dy - dx) as usize][(end.0 + dx + dy) as usize] = Tile::Ice;
}

fn step(map: &Vec<Vec<Tile>>, start: &(isize, isize), direction: Direction) -> (isize, isize) {
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

#[derive(Debug)]
struct SearchState {
    // score: f32,
    length: isize,
    path: Vec<(Direction, (isize, isize))>,
    decision_positions: Vec<(isize, isize)>,
}

// impl Ord for SearchState {
//     fn cmp(&self, other: &Self) -> std::cmp::Ordering {
//         self.score.partial_cmp(&other.score).unwrap()
//     }
// }

// impl PartialOrd for SearchState {
//     fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
//         self.score.partial_cmp(&other.score)
//     }
// }

// impl Eq for SearchState {}

// impl PartialEq for SearchState {
//     fn eq(&self, other: &Self) -> bool {
//         self.score == other.score
//     }
// }

fn solve(board: &Board) -> Option<Analysis> {
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

        let mut new_decision_list = state.decision_positions;

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
                return Some(Analysis {
                    solution: path,
                    search_complexity,
                    search_tile_coverage,
                    solution_tile_coverage: new_length,
                    decision_positions: new_decision_list,
                });
            }
            visitations.insert(new_step);

            new_states.push(SearchState {
                length: new_length,
                path: new_path,
                decision_positions: new_decision_list.clone(),
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

fn presentation_cleanup(mut ret: Board) -> Board {
    let mut reachability = vec![vec![false; ret.map[0].len()]; ret.map.len()];

    let mut flood_edge: VecDeque<(isize, isize)> = VecDeque::from([ret.start, ret.end]);

    println!("{:?}", flood_edge);

    while let Some(next_check) = flood_edge.pop_front() {
        if next_check.0 < 0 || next_check.0 >= (ret.map[0].len()) as isize {
            continue;
        }
        if next_check.1 < 0 || next_check.1 >= (ret.map.len()) as isize {
            continue;
        }

        if reachability[next_check.1 as usize][next_check.0 as usize] == false
            && ret.map[next_check.1 as usize][next_check.0 as usize] != Tile::Wall
        {
            reachability[next_check.1 as usize][next_check.0 as usize] = true;

            for dir in [
                Direction::North,
                Direction::South,
                Direction::East,
                Direction::West,
            ] {
                flood_edge
                    .push_back((next_check.0 + dir.vector().0, next_check.1 + dir.vector().1));
            }
        }
    }

    for (y, row) in reachability.iter().enumerate() {
        for (x, reacheable) in row.iter().enumerate() {
            if (!*reacheable) {
                ret.map[y][x] = Tile::Wall;
            }
        }
    }

    ret
}

#[flutter_rust_bridge::frb(sync)] // Synchronous mode for simplicity of the demo
pub fn search_board() -> Board {
    let mut rng = rand::rng();

    let start_time = Instant::now();

    let mut population: ReverseSortedVec<Creature> = (0..100)
        .map(|_| get_random_board())
        .map(|b| Creature::new(b))
        .filter(|e| e.is_some())
        .map(|e| e.unwrap())
        .collect();

    while population.len() == 0 || start_time.elapsed() < Duration::from_secs(1) {
        while population.len() < 100 {
            let creature = population.choose(&mut rng);

            if let Some(creature) = creature {
                if let Some(new_creature) = creature.mutate(0.1) {
                    population.insert(new_creature);
                }
            } else {
                if let Some(new_creature) = Creature::new(get_random_board()) {
                    population.insert(new_creature);
                }
            }
        }

        population = population[0..30].into_iter().cloned().collect();
    }

    println!("mutation count of winner {}", population[0].mutation_count);

    population[0].board.clone()
}

#[flutter_rust_bridge::frb(init)]
pub fn init_app() {
    // Default utilities - feel free to customize
    flutter_rust_bridge::setup_default_user_utils();
}
