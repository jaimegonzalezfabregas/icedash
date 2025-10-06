use rand::seq::IndexedRandom;
use std::collections::VecDeque;

use crate::{
    api::main::{Direction, Pos, Tile},
    logic::{board::Board, tile_map::TileMap},
};

pub fn map_noise_cleanup(
    map: &mut TileMap,
    start: &mut Pos,
    start_direction: Direction,
    end: &mut Pos,
    end_direction: Direction,
) {
    let mut rng = rand::rng();

    let width = map.get_width();
    let height = map.get_height();

    if start.x == end.x || start.y == end.y {
        let mean = (*start + *end) / 2;

        map.set(mean, Tile::Wall);
    }

    let mut rep = true;
    while rep {
        rep = false;
        for y in 1..height - 2 {
            for x in 1..width - 2 {
                let a = map.atxy(x, y);
                let b = map.atxy(x, y + 1);
                let c = map.atxy(x + 1, y);
                let d = map.atxy(x + 1, y + 1);

                let cuad = (a, b, c, d);

                match cuad {
                    (Tile::Ice, Tile::Wall, Tile::Wall, Tile::Ice) => {
                        rep = true;
                        if *([true, false].choose(&mut rng).unwrap()) {
                            *map.at_mut(Pos::new(x, y)) = Tile::Wall;
                        } else {
                            *map.at_mut(Pos::new(x + 1, y + 1)) = Tile::Wall;
                        }
                    }
                    (Tile::Wall, Tile::Ice, Tile::Ice, Tile::Wall) => {
                        rep = true;
                        if *([true, false].choose(&mut rng).unwrap()) {
                            *map.at_mut(Pos::new(x + 1, y)) = Tile::Wall;
                        } else {
                            *map.at_mut(Pos::new(x + 1, y)) = Tile::Wall;
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
        let all_inner_pos = map.all_inner_pos().collect::<Vec<_>>();
        for p in all_inner_pos {
            if map.at(p) != Tile::Wall {
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
                    let neigh = map.at(p + Pos::new(dx, dy));
                    if neigh.is_solid() {
                        neigh_count += 1;
                    }
                }

                if neigh_count >= 6 {
                    map.set(p, Tile::Wall);
                    rep = true;
                }
            }
        }
    }

    let mut rep = true;
    while rep {
        rep = false;
        let all_inner_pos = map.all_inner_pos().collect::<Vec<_>>();
        for p in all_inner_pos {
            if map.at(p) != Tile::Wall {
                let mut neigh_count = 0;
                for (dx, dy) in [(0, 1), (0, -1), (-1, 0), (1, 0)] {
                    let neigh = map.at(p + Pos::new(dx, dy));
                    if neigh.is_solid() {
                        neigh_count += 1;
                    }
                }

                if neigh_count >= 3 {
                    map.set(p, Tile::Wall);
                    rep = true;
                }
            }
        }
    }

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

pub fn room_at(p1: Pos, p2: Pos, board: &Board) -> bool {
    let entrance_corridor = board.start + board.start_direction.vector();

    if entrance_corridor == p1 || entrance_corridor == p2 {
        return false;
    }

    let mut split_board1 = board.clone();
    let mut split_board2 = board.clone();

    split_board2.map.set(p1, Tile::Wall);
    split_board1.map.set(p2, Tile::Wall);

    let reachibility1 = flood(vec![p2], &split_board2);
    let reachibility2 = flood(vec![p1], &split_board1);

    for (y, row) in reachibility1.into_iter().enumerate() {
        for (x, reach) in row.into_iter().enumerate() {
            if reach && reachibility2[y][x] {
                return false;
            }
        }
    }
    true
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

        if ret.map.0[ret.map.0.len() - 1] == ret.map.0[ret.map.0.len() - 2] {
            ret.map.0.pop();
        }
    }

    ret
}
