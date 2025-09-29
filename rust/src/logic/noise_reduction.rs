use std::collections::VecDeque;

use itertools::Itertools;
use rand::seq::IndexedRandom;

use crate::{
    api::main::{Board, Direction, Tile},
    logic::solver::Analysis,
};

pub fn board_noise_cleanup(
    ret: &mut Vec<Vec<Tile>>,
    start: &mut (isize, isize),
    start_direction: Direction,
    end: &mut (isize, isize),
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

    let mut rep = true;
    while rep {
        rep = false;
        for y in 1..height - 1 {
            for x in 1..width - 1 {
                if ret[(y) as usize][(x) as usize] != Tile::Wall {
                    let mut neigh_count = 0;
                    for (dx, dy) in [(0, 1), (0, -1), (-1, 0), (1, 0)] {
                        let neigh = ret[(y + dy) as usize][(x + dx) as usize];
                        if neigh == Tile::Wall {
                            neigh_count += 1;
                        }
                    }

                    if neigh_count >= 3 {
                        ret[(y) as usize][(x) as usize] = Tile::Wall;
                        rep = true;
                    }
                }
            }
        }
    }

    ret[start.1 as usize][start.0 as usize] = Tile::Entrance;
    ret[end.1 as usize][end.0 as usize] = Tile::Gate;

    let (dx, dy) = start_direction.vector();

    ret[(start.1 + dy) as usize][(start.0 + dx) as usize] = Tile::Ice;
    ret[(start.1 + dy + dx) as usize][(start.0 + dx - dy) as usize] = Tile::Ice;
    ret[(start.1 + dy - dx) as usize][(start.0 + dx + dy) as usize] = Tile::Ice;

    ret[(start.1 + dy * 2) as usize][(start.0 + dx * 2) as usize] = Tile::Ice;
    ret[(start.1 + dy * 2 + dx) as usize][(start.0 + dx * 2 - dy) as usize] = Tile::Ice;
    ret[(start.1 + dy * 2 - dx) as usize][(start.0 + dx * 2 + dy) as usize] = Tile::Ice;

    let (dx, dy) = end_direction.vector();
    ret[(end.1 + dy) as usize][(end.0 + dx) as usize] = Tile::Ice;
    ret[(end.1 + dy + dx) as usize][(end.0 + dx - dy) as usize] = Tile::Ice;
    ret[(end.1 + dy - dx) as usize][(end.0 + dx + dy) as usize] = Tile::Ice;

    ret[(end.1 + dy * 2) as usize][(end.0 + dx * 2) as usize] = Tile::Ice;
    ret[(end.1 + dy * 2 + dx) as usize][(end.0 + dx * 2 - dy) as usize] = Tile::Ice;
    ret[(end.1 + dy * 2 - dx) as usize][(end.0 + dx * 2 + dy) as usize] = Tile::Ice;
}

pub fn is_board_valid(board: &Board) -> bool {
    let height = board.map.len() as isize;
    let width = board.map[0].len() as isize;

    for y in 1..height - 1 {
        for x in 1..width - 1 {
            if board.map[(y) as usize][(x) as usize] == Tile::Ice {
                let mut ice_neigh = vec![];
                for (dx, dy) in [(0, 1), (0, -1), (-1, 0), (1, 0)] {
                    let neigh = board.map[(y + dy) as usize][(x + dx) as usize];
                    if neigh == Tile::Ice {
                        ice_neigh.push(((x + dx), (y + dy)));
                    }
                }
                let mut split_board = board.clone();
                split_board.map[y as usize][x as usize] = Tile::Wall;

                for pair in ice_neigh.iter().permutations(2) {
                    let reachibility_of_a = flood(vec![*pair[0]], &split_board);

                    if !reachibility_of_a[pair[1].1 as usize][pair[1].0 as usize] {
                        // split_board.print(vec![(x,y), *pair[0], *pair[1]]);

                        return false;
                    }
                }
            }
        }
    }
    true
}

pub fn flood(starting_positions: Vec<(isize, isize)>, board: &Board) -> Vec<Vec<bool>> {
    let mut reachability = vec![vec![false; board.map[0].len()]; board.map.len()];
    let mut flood_edge: VecDeque<(isize, isize)> = VecDeque::from(starting_positions);

    while let Some(next_check) = flood_edge.pop_front() {
        if next_check.0 < 0 || next_check.0 >= (board.map[0].len()) as isize {
            continue;
        }
        if next_check.1 < 0 || next_check.1 >= (board.map.len()) as isize {
            continue;
        }

        if reachability[next_check.1 as usize][next_check.0 as usize] == false
            && board.map[next_check.1 as usize][next_check.0 as usize] != Tile::Wall
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
    reachability
}

pub fn asthetic_cleanup(mut ret: Board) -> Board {
    let reachability = flood(vec![ret.start, ret.end], &ret);

    for (y, row) in reachability.iter().enumerate() {
        for (x, reacheable) in row.iter().enumerate() {
            if !*reacheable {
                ret.map[y][x] = Tile::Wall;
            }
        }
    }

    println!("removing unused borders");

    while ret.map[ret.map.len() - 2].iter().all(|e| *e == Tile::Wall) {
        ret.map.remove(ret.map.len() - 2);
    }

    while ret.map.iter().all(|e| e[e.len() - 2] == Tile::Wall) {
        ret.map.iter_mut().for_each(|e| {
            e.remove(e.len() - 1);
        });
    }

    while ret.map[1].iter().all(|e| *e == Tile::Wall) {
        ret.map.remove(1);

        if ret.end.1 == ret.map.len() as isize {
            ret.end.1 = ret.map.len() as isize - 1;
        } else if ret.end.1 != 0 {
            ret.end.1 -= 1;
        }

        if ret.start.1 == ret.map.len() as isize {
            ret.start.1 = ret.map.len() as isize - 1;
        } else if ret.start.1 != 0 {
            ret.start.1 -= 1;
        }
    }

    while ret.map.iter().all(|e| e[1] == Tile::Wall) {
        ret.map.iter_mut().for_each(|e| {
            e.remove(1);
        });

        if ret.end.0 == ret.map[0].len() as isize {
            ret.end.0 = ret.map[0].len() as isize - 1;
        } else if ret.end.0 != 0 {
            ret.end.0 -= 1;
        }

        if ret.start.0 == ret.map[0].len() as isize {
            ret.start.0 = ret.map[0].len() as isize - 1;
        } else if ret.start.0 != 0 {
            ret.start.0 -= 1;
        }
    }

    ret
}
