use rand::{random, seq::IteratorRandom, Rng};

use crate::{api::main::{Direction, Pos, Tile}, logic::{noise_reduction::{is_board_valid, map_noise_cleanup}, solver::step, tile_map::TileMap}};

#[derive(Clone)]
pub struct Board {
    pub map: TileMap,
    pub start: Pos,
    pub end: Pos,
    pub reset_pos: Pos,
    pub start_direction: Direction,
    pub end_direction: Direction,
}

impl Board {
    pub fn get_height(&self) -> isize {
        self.map.get_height()
    }

    pub fn get_width(&self) -> isize {
        self.map.get_width()
    }

    pub fn mutate(&self, factor: f32) -> Option<Self> {
        let mut rng = rand::rng();

        let mut ret = self.clone();

        for pos in self.map.all_inner_pos() {
            if rng.random::<f32>() < factor {
                match ret.map.at(pos) {
                    Tile::Wall => ret.map.set(pos, Tile::Ice),
                    Tile::Ice => ret.map.set(pos, Tile::Wall),
                    _ => {}
                }
            }
        }

        map_noise_cleanup(
            &mut ret.map,
            &mut ret.start,
            ret.start_direction,
            &mut ret.end,
            ret.end_direction,
        );

        ret.reset_pos = step(&ret.map, &ret.start, ret.start_direction);

        if is_board_valid(&ret) {
            Some(ret)
        } else {
            None
        }
    }

    pub fn new_random() -> Option<Self> {
        let mut rng = rand::rng();
        let width = (7..15).choose(&mut rng)?;
        let height = (7..15).choose(&mut rng)?;

        let start_side = (0..3).choose(&mut rng)?;
        let end_side = ((1..3).choose(&mut rng)? + start_side) % 4;

        let gate_range_horizontal = &(3..height - 3);
        let gate_range_vertical = &(3..width - 3);

        let (start, start_direction) = match start_side {
            0 => (
                Pos::new(0, gate_range_horizontal.clone().choose(&mut rng)?),
                Direction::West,
            ),
            1 => (
                Pos::new(width - 1, gate_range_horizontal.clone().choose(&mut rng)?),
                Direction::East,
            ),
            2 => (
                Pos::new(gate_range_vertical.clone().choose(&mut rng)?, 0),
                Direction::South,
            ),
            _ => (
                Pos::new(gate_range_vertical.clone().choose(&mut rng)?, height - 1),
                Direction::North,
            ),
        };

        let (end, end_direction) = match end_side {
            0 => (
                Pos::new(0, gate_range_horizontal.clone().choose(&mut rng)?),
                Direction::West,
            ),
            1 => (
                Pos::new(width - 1, gate_range_horizontal.clone().choose(&mut rng)?),
                Direction::East,
            ),
            2 => (
                Pos::new(gate_range_vertical.clone().choose(&mut rng)?, 0),
                Direction::South,
            ),
            _ => (
                Pos::new(gate_range_vertical.clone().choose(&mut rng)?, height - 1),
                Direction::North,
            ),
        };

        let mut map = vec![vec![Tile::Wall; width as usize]; height as usize];

        for x in 1..width - 1 {
            for y in 1..height - 1 {
                map[y as usize][x as usize] = Tile::Ice;
            }
        }

        let pilars = ((width * height) / 10..(width * height) / 5).choose(&mut rng)?;

        for _ in 0..pilars {
            let x = (1..(width - 1) as usize).choose(&mut rng)?;
            let y = (1..(height - 1) as usize).choose(&mut rng)?;

            map[y][x] = Tile::Wall;
        }

        let vignet = ((width * height) / 10..(width * height) / 5).choose(&mut rng)?;

        for _ in 0..vignet {
            let x = (1..(width - 1) as usize).choose(&mut rng)?;
            let y = (1..(height - 1) as usize).choose(&mut rng)?;

            let normal_x = (x as f32 / width as f32) - 0.5;
            let normal_y = (y as f32 / height as f32) - 0.5;

            let normal_d = normal_x * normal_x + normal_y * normal_y;

            if random::<f32>() > normal_d {
                map[y][x] = Tile::Wall;
            }

            map[y][x] = Tile::Wall;
        }
        let mut start = start;
        let mut end = end;

        let mut map = TileMap(map);

        map_noise_cleanup(
            &mut map,
            &mut start,
            start_direction,
            &mut end,
            end_direction,
        );

        let ret = Board {
            reset_pos: step(&map, &start, start_direction),
            map,
            start,
            start_direction,
            end,
            end_direction,
        };

        if is_board_valid(&ret) {
            Some(ret)
        } else {
            None
        }
    }

    pub(crate) fn print(&self, highlight: Vec<Pos>) {
        println!(
            "printing start {:?} {:?} end {:?} {:?}",
            self.start, self.start_direction, self.end, self.end_direction
        );

        self.map.print(highlight);
    }

    pub fn rotate_left(self) -> Self {
        Board {
            start: self.start.rotate_left(self.map.get_height()),
            end: self.end.rotate_left(self.map.get_height()),
            reset_pos: self.reset_pos.rotate_left(self.map.get_height()),
            start_direction: self.start_direction.left(),
            end_direction: self.end_direction.left(),
            map: self.map.rotate_left(),
        }
    }
}
