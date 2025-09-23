use std::collections::VecDeque;

use rand::prelude::*;

#[derive(Clone, PartialEq, Copy, Debug)]
pub enum Tile {
    Entrance,
    Gate,
    Wall,
    Ice,
    Ground,
    Outside,
}

#[derive(Clone, PartialEq, Copy)]
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
}

#[derive(Clone)]
pub struct Analysis {
    solution: Vec<(Direction, (isize, isize))>,
    decision_count: isize,
}

pub struct Board {
    pub map: Vec<Vec<Tile>>,
    pub start: (isize, isize),
    pub end: (isize, isize),
    pub start_direction: Direction,
}

fn get_random_board() -> Board {
    let mut rng = rand::rng();
    let width = (5..15).choose(&mut rng).unwrap();
    let height = (5..15).choose(&mut rng).unwrap();
    let clutterness = 0.05 + rng.random::<f32>() * 0.2;

    let start_side = (0..3).choose(&mut rng).unwrap();
    let end_side = ((1..3).choose(&mut rng).unwrap() + start_side) % 4;

    let (start, start_direction) = match start_side {
        0 => (
            (0, (2..width - 2).choose(&mut rng).unwrap()),
            Direction::East,
        ),
        1 => (
            (width - 1, (2..height - 2).choose(&mut rng).unwrap()),
            Direction::West,
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

    let end = match end_side {
        0 => (0, (2..width - 2).choose(&mut rng).unwrap()),
        1 => (width, (2..height - 2).choose(&mut rng).unwrap()),
        2 => ((2..width - 2).choose(&mut rng).unwrap(), 0),
        _ => ((2..width - 2).choose(&mut rng).unwrap(), height),
    };

    let mut ret = Board {
        map: vec![vec![Tile::Wall; width as usize]; height as usize],
        start,
        start_direction,
        end,
    };

    for y in 1..height - 1 {
        let row = vec![];

        for x in 1..width - 1 {
            if rng.random::<f32>() > clutterness {
                ret.map[y as usize][x as usize] = Tile::Ice;
            }
        }

        ret.map.push(row);
    }
    println!("{start:?},{end:?},{:?}", ret.map);

    ret.map[start.1 as usize][start.0 as usize] = Tile::Entrance;
    ret.map[end.1 as usize][end.0 as usize] = Tile::Gate;

    ret
}

fn step(map: &Vec<Vec<Tile>>, start: &(isize, isize), direction: Direction) -> (isize, isize) {
    let mut ret = start.clone();

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

fn solve(
    map: &Vec<Vec<Tile>>,
    start: &(isize, isize),
    start_direction: Direction,
    end: &(isize, isize),
) -> Option<Vec<(Direction, (isize, isize))>> {
    let mut states = VecDeque::from([(vec![(start_direction, step(map, start, start_direction))])]);

    while let Some(state) = states.pop_front() {
        for dir in [
            Direction::North,
            Direction::East,
            Direction::East,
            Direction::West,
        ] {
            if dir == state.last().unwrap().0 {
                continue;
            }

            let step_start = state.last().unwrap().1;

            let new_step = step(map, &step_start, dir);

            if new_step != step_start {
                continue;
            }

            let mut new_history = state.clone();
            new_history.push((dir, new_step));

            if new_step == *end {
                return Some(new_history);
            }

            states.push_back(new_history);
        }
    }

    return None;
}

fn analyze(board: &Board) -> Option<Analysis> {
    let solution = solve(&board.map, &board.start, board.start_direction, &board.end)?;

    let mut decision_count = 0;

    for (_dir, pos) in &solution {
        let mut neighbour_count = 0;
        for (dx, dy) in [(0, 1), (1, 0), (-1, 0), (0, -1)] {
            let neighbour = board.map[(pos.1 + dy) as usize][(pos.0 + dx) as usize];
            if neighbour == Tile::Ice {
                neighbour_count += 1
            };
        }
        if neighbour_count > 2 {
            decision_count += 1;
        }
    }

    Some(Analysis {
        solution,
        decision_count,
    })
}

#[flutter_rust_bridge::frb(sync)] // Synchronous mode for simplicity of the demo
pub fn search_board() -> Board {
    loop {
        let board = get_random_board();

        if let Some(_) = analyze(&board) {
            return board;
        }
    }
}

#[flutter_rust_bridge::frb(init)]
pub fn init_app() {
    // Default utilities - feel free to customize
    flutter_rust_bridge::setup_default_user_utils();
}
