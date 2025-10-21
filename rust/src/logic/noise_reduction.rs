use std::{collections::VecDeque, vec};

use rand::random;

use crate::{
    api::main::{Direction, Pos, Tile},
    logic::{
        board::{Board, TileMapWrap},
        tile_map::TileMap,
        visitations::Visitations,
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

        map.set(&mean, Tile::Wall);
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



    map.set(&(*start + start_direction.vector()), Tile::Ice);
    map.set(
        &(*start + start_direction.vector() + start_direction.left().vector()),
        Tile::Wall,
    );
    map.set(
        &(*start + start_direction.vector() + start_direction.right().vector()),
        Tile::Wall,
    );

    map.set(&(*start + start_direction.vector() * 2), Tile::Ice);
    map.set(
        &(*start + start_direction.vector() * 2 + start_direction.left().vector()),
        Tile::Ice,
    );
    map.set(
        &(*start + start_direction.vector() * 2 + start_direction.right().vector()),
        Tile::Ice,
    );


    map.set(&(*end + end_direction.vector()), Tile::Ice);
    map.set(
        &(*end + end_direction.vector() + end_direction.left().vector()),
        Tile::Ice,
    );
    map.set(
        &(*end + end_direction.vector() + end_direction.right().vector()),
        Tile::Ice,
    );

    map.set(&(*end + end_direction.vector() * 2), Tile::Ice);
    map.set(
        &(*end + end_direction.vector() * 2 + end_direction.left().vector()),
        Tile::Ice,
    );
    map.set(
        &(*end + end_direction.vector() * 2 + end_direction.right().vector()),
        Tile::Ice,
    );

        remove_rooms(map, &start, &start_direction);

            map.set(end, Tile::Gate);
    map.set(start, Tile::Entrance);


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

pub fn connected(seed1: Pos, seed2: Pos, board1: &TileMapWrap, board2: &TileMapWrap) -> bool {
    let mut search1 = BinaryHeap::new();
    let mut search2 = BinaryHeap::new();

    let mut found1 = Visitations::new(board1.base.get_width(), board1.base.get_height());
    let mut found2 = Visitations::new(board2.base.get_width(), board2.base.get_height());

    // Push the initial positions with 0 cost and heuristic distance
    search1.push(ConnectedSearchState {
        pos: seed1,
        heuristic: heuristic_distance(seed1, seed2),
    });
    found1.insert(&seed1);

    search2.push(ConnectedSearchState {
        pos: seed2,
        heuristic: heuristic_distance(seed2, seed1),
    });

    while !search1.is_empty() && !search2.is_empty() {
        let p1 = search1.pop().unwrap();
        let p2 = search2.pop().unwrap();

        // Add new positions to the search based on directions.
        for direction in Direction::all() {
            let next_pos1 = direction.vector() + p1.pos;

            if board2.base.in_bounds(&next_pos1) && !found1.contains(&next_pos1) {
                if !board1.at(&next_pos1).stops_player_during_gameplay() {
                    if found2.contains(&next_pos1) {
                        return true;
                    }
                    found1.insert(&next_pos1);
                    search1.push(ConnectedSearchState {
                        pos: next_pos1,
                        heuristic: heuristic_distance(next_pos1, seed2),
                    });
                }
            }

            let next_pos2 = direction.vector() + p2.pos;

            if board2.base.in_bounds(&next_pos2) && !found2.contains(&next_pos2) {
                if !board2.at(&next_pos2).stops_player_during_gameplay() {
                    if found1.contains(&next_pos2) {
                        return true;
                    }
                    found2.insert(&next_pos2);
                    search2.push(ConnectedSearchState {
                        pos: next_pos2,
                        heuristic: heuristic_distance(next_pos2, seed1),
                    });
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

pub fn remove_rooms(board: &mut TileMap, start: &Pos, start_direction: &Direction) {
    let all_pos = board.all_inner_pos().collect::<Vec<_>>();
    let entrance_corridor = *start + start_direction.vector();

    let mut rep = true;

    while rep {
        rep = false;

        for p1 in &all_pos {
            if board.at(&p1).stops_player_during_gameplay() {
                continue;
            }

            for (dx, dy) in [(0, 1), (1, 0)] {
                let p2 = *p1 + Pos::new(dx, dy);

                if board.at(&p2).stops_player_during_gameplay() {
                    continue;
                }

                let direction = Pos { x: dx, y: dy };
                let normal_direction = direction.rotate_left(1);

                if board.at(&(*p1 + normal_direction)) == Tile::Ice
                    && board.at(&(p2 + normal_direction)) == Tile::Ice
                {
                    continue;
                }

                if board.at(&(*p1 - normal_direction)) == Tile::Ice
                    && board.at(&(p2 - normal_direction)) == Tile::Ice
                {
                    continue;
                }

                if entrance_corridor == *p1 || entrance_corridor == p2 {
                    continue;
                }

                if !connected(
                    *p1,
                    p2,
                    &TileMapWrap {
                        base: &board,
                        p: p2,
                        tile: Tile::Wall,
                    },
                    &TileMapWrap {
                        base: &board,
                        p: *p1,
                        tile: Tile::Wall,
                    },
                ) {
                    if random::<f32>() > 0.5 {
                        board.set(&p1, Tile::Wall);
                    } else {
                        board.set(&p2, Tile::Wall);
                    }

                    rep = true;
                }
            }
        }
    }
}

pub fn flood(
    starting_positions: Vec<Pos>,
    board: &TileMap,
    traversable_tiles: Vec<Tile>,
) -> Visitations {
    let mut reachability = Visitations::new(board.get_width(), board.get_height());
    let mut flood_edge: VecDeque<Pos> = VecDeque::from(starting_positions);

    while let Some(next_check) = flood_edge.pop_front() {
        if !board.in_bounds(&next_check) {
            continue;
        }

        if !reachability.contains(&next_check) && traversable_tiles.contains(&board.at(&next_check))
        {
            reachability.insert(&next_check);

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
        &ret.map,
        vec![
            Tile::Ice,
            Tile::WeakWall,
            Tile::Box,
            Tile::Entrance,
            Tile::Gate,
        ],
    );
    let inner_pos = ret.map.all_pos().collect::<Vec<_>>();

    for p in inner_pos {
        if ret.end == p {
            continue;
        }

        if ret.start == p {
            continue;
        }
        if !reachability.contains(&p) {
            ret.map.set(&p, Tile::Wall);
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
