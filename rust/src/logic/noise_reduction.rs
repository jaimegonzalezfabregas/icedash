use std::{
    collections::{HashSet, VecDeque},
    vec,
};

use crate::{
    api::main::{Direction, Pos, Tile},
    logic::{
        board::{Board, BoardWrap},
        tile_map::TileMap,
    },
};

// pub fn remove_awkward_corners(map: &mut TileMap) {
//     let mut rng = rand::rng();

//     let mut rep = true;
//     let width = map.get_width();
//     let height = map.get_height();

//     while rep {
//         rep = false;
//         for y in 1..height - 2 {
//             for x in 1..width - 2 {
//                 let a = map.atxy(x, y);
//                 let b = map.atxy(x, y + 1);
//                 let c = map.atxy(x + 1, y);
//                 let d = map.atxy(x + 1, y + 1);

//                 let cuad = (a, b, c, d);

//                 match cuad {
//                     (Tile::Ice, Tile::Wall, Tile::Wall, Tile::Ice) => {
//                         rep = true;
//                         if *([true, false].choose(&mut rng).unwrap()) {
//                             *map.at_mut(Pos::new(x, y)) = Tile::Wall;
//                         } else {
//                             *map.at_mut(Pos::new(x + 1, y + 1)) = Tile::Wall;
//                         }
//                     }
//                     (Tile::Wall, Tile::Ice, Tile::Ice, Tile::Wall) => {
//                         rep = true;
//                         if *([true, false].choose(&mut rng).unwrap()) {
//                             *map.at_mut(Pos::new(x + 1, y)) = Tile::Wall;
//                         } else {
//                             *map.at_mut(Pos::new(x + 1, y)) = Tile::Wall;
//                         }
//                     }

//                     _ => {}
//                 }
//             }
//         }
//     }
// }

// fn remove_sorrounded_spaces<const N: usize>(
//     map: &mut TileMap,
//     vector_list: [(isize, isize); N],
//     threshold: usize,
// ) {
//     let mut to_check = map.all_inner_pos().collect::<Vec<_>>();
//     while let Some(p) = to_check.pop() {
//         if map.in_bounds(p) {
//             if map.at(p) != Tile::Wall {
//                 let mut neigh_count = 0;
//                 for (dx, dy) in vector_list {
//                     if map.in_bounds(p + Pos::new(dx, dy)) {
//                         let neigh = map.at(p + Pos::new(dx, dy));
//                         if neigh.is_solid() {
//                             neigh_count += 1;
//                         }
//                     }
//                 }

//                 if neigh_count >= threshold {
//                     map.set(p, Tile::Wall);

//                     let new_to_check = vector_list.map(|(dx, dy)| p - Pos { x: dx, y: dy });

//                     to_check.append(&mut new_to_check.into());
//                 }
//             }
//         }
//     }
// }

pub fn map_noise_cleanup(
    map: &mut TileMap,
    start: &mut Pos,
    start_direction: Direction,
    end: &mut Pos,
    end_direction: Direction,
) {
    if start.x == end.x || start.y == end.y {
        let mean = (*start + *end) / 2;

        map.set(mean, Tile::Wall);
    }

    // remove_awkward_corners(&mut map);

    // remove_sorrounded_spaces(
    //     &mut map,
    //     [
    //         (0, 1),
    //         (0, -1),
    //         (-1, 0),
    //         (1, 0),
    //         (1, 1),
    //         (1, -1),
    //         (-1, -1),
    //         (-1, 1),
    //     ],
    //     6,
    // );

    // remove_sorrounded_spaces(&mut map, [(0, 1), (0, -1), (-1, 0), (1, 0)], 3);

    map.set(*start, Tile::Entrance);

    map.set(*start + start_direction.vector(), Tile::Ice);
    map.set(
        *start + start_direction.vector() + start_direction.left().vector(),
        Tile::Wall,
    );
    map.set(
        *start + start_direction.vector() + start_direction.right().vector(),
        Tile::Wall,
    );

    map.set(*start + start_direction.vector() * 2, Tile::Ice);
    map.set(
        *start + start_direction.vector() * 2 + start_direction.left().vector(),
        Tile::Ice,
    );
    map.set(
        *start + start_direction.vector() * 2 + start_direction.right().vector(),
        Tile::Ice,
    );

    map.set(*end, Tile::Gate);

    map.set(*end + end_direction.vector(), Tile::Ice);
    map.set(
        *end + end_direction.vector() + end_direction.left().vector(),
        Tile::Ice,
    );
    map.set(
        *end + end_direction.vector() + end_direction.right().vector(),
        Tile::Ice,
    );

    map.set(*end + end_direction.vector() * 2, Tile::Ice);
    map.set(
        *end + end_direction.vector() * 2 + end_direction.left().vector(),
        Tile::Ice,
    );
    map.set(
        *end + end_direction.vector() * 2 + end_direction.right().vector(),
        Tile::Ice,
    );
}
use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Debug, Clone, PartialEq, Eq)]
struct ConnectedSearchState {
    pos: Pos,
    heuristic: usize,
}

impl Ord for ConnectedSearchState {
    fn cmp(&self, other: &Self) -> Ordering {
        other.heuristic.cmp(&self.heuristic)
        // We want the priority queue to pop the smallest cost first
    }
}

impl PartialOrd for ConnectedSearchState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn connected(seed1: Pos, seed2: Pos, board1: &BoardWrap, board2: &BoardWrap) -> bool {
    let mut search1 = BinaryHeap::new();
    let mut search2 = BinaryHeap::new();

    let mut found = HashSet::<Pos>::new();

    // Push the initial positions with 0 cost and heuristic distance
    search1.push(ConnectedSearchState {
        pos: seed1,
        heuristic: heuristic_distance(seed1, seed2),
    });

    search2.push(ConnectedSearchState {
        pos: seed2,
        heuristic: heuristic_distance(seed2, seed1),
    });

    found.insert(seed1);
    found.insert(seed2);

    while !search1.is_empty() && !search2.is_empty() {
        let p1 = search1.pop().unwrap();
        let p2 = search2.pop().unwrap();

        // Add new positions to the search based on directions.
        for direction in Direction::all() {
            let next_pos1 = direction.vector() + p1.pos;
            if board1.at(next_pos1) == Tile::Ice {
                if !found.contains(&next_pos1) {
                    found.insert(next_pos1);
                    search1.push(ConnectedSearchState {
                        pos: next_pos1,
                        heuristic: heuristic_distance(next_pos1, seed2),
                    });
                } else {
                    println!("{:?}", next_pos1);

                    return true;
                }
            }

            let next_pos2 = direction.vector() + p2.pos;
            if board2.at(next_pos2) == Tile::Ice {
                if !found.contains(&next_pos2) {
                    found.insert(next_pos2);
                    search2.push(ConnectedSearchState {
                        pos: next_pos2,
                        heuristic: heuristic_distance(next_pos2, seed1),
                    });
                } else {
                    println!("{:?}", next_pos2);
                    return true;
                }
            }
        }
    }

    false
}

// Assuming you have a function to calculate the heuristic distance
fn heuristic_distance(pos1: Pos, pos2: Pos) -> usize {
    // Example: Manhattan distance
    ((pos1.x - pos2.x).abs() + (pos1.y - pos2.y).abs())
        .try_into()
        .unwrap()
}

pub fn room_at(p1: Pos, p2: Pos, board: &Board) -> bool {
    let entrance_corridor = board.start + board.start_direction.vector();

    if entrance_corridor == p1 || entrance_corridor == p2 {
        return false;
    }

    let ret = !connected(
        p1,
        p2,
        &BoardWrap {
            base: &board,
            p: p2,
            tile: Tile::Wall,
        },
        &BoardWrap {
            base: &board,
            p: p1,
            tile: Tile::Wall,
        },
    );

    println!("{ret} room at:");
    board.print(vec![p1,p2]);

    ret
}

pub fn has_rooms(board: &Board) -> bool {
    for p1 in board.map.all_inner_pos() {
        if board.map.at(p1) != Tile::Ice {
            continue;
        }

        for (dx, dy) in [(0, 1), (1, 0)] {
            let p2 = p1 + Pos::new(dx, dy);

            if board.map.at(p2) != Tile::Ice {
                continue;
            }

            let direction = Pos { x: dx, y: dy };
            let normal_direction = direction.rotate_left(1);

            if board.map.at(p1 + normal_direction) == Tile::Ice
                && board.map.at(p2 + normal_direction) == Tile::Ice
            {
                continue;
            }

            if board.map.at(p1 - normal_direction) == Tile::Ice
                && board.map.at(p2 - normal_direction) == Tile::Ice
            {
                continue;
            }

            if room_at(p1, p2, board) {
                // board.print(vec![(x1, y1), (x2, y2)]);
                return true;
            }
        }
    }
    false
}

pub fn is_board_valid(board: &Board) -> bool {
    !has_rooms(board)
}

pub fn flood(starting_positions: Vec<Pos>, board: &Board) -> Vec<Vec<bool>> {
    let mut reachability =
        vec![vec![false; board.map.get_width() as usize]; board.map.get_height() as usize];
    let mut flood_edge: VecDeque<Pos> = VecDeque::from(starting_positions);

    while let Some(next_check) = flood_edge.pop_front() {
        if !board.map.in_bounds(next_check) {
            continue;
        }

        if reachability[next_check.y as usize][next_check.x as usize] == false
            && !board.map.at(next_check).is_solid()
        {
            reachability[next_check.y as usize][next_check.x as usize] = true;

            for dir in [
                Direction::North,
                Direction::South,
                Direction::East,
                Direction::West,
            ] {
                flood_edge.push_back(next_check + dir.vector());
            }
        }
    }
    reachability
}

pub fn asthetic_cleanup(mut ret: Board) -> Board {
    let reachability = flood(
        vec![
            ret.start + ret.start_direction.vector(),
            ret.end + ret.end_direction.vector(),
        ],
        &ret,
    );

    for (y, row) in reachability.iter().enumerate() {
        for (x, reacheable) in row.iter().enumerate() {
            if ret.end.x == x as isize && ret.end.y == y as isize {
                continue;
            }

            if ret.start.x == x as isize && ret.start.y == y as isize {
                continue;
            }
            if !*reacheable {
                ret.map.set(Pos::new(x as isize, y as isize), Tile::Wall);
            }
        }
    }

    for _ in 0..4 {
        ret = ret.rotate_left();

        while ret.map.0[ret.map.0.len() - 1] == ret.map.0[ret.map.0.len() - 2] {
            ret.map.0.pop();
        }
    }

    ret
}
