use rand::{random, seq::IteratorRandom, Rng};

use crate::{
    api::main::{Direction, Pos, Tile},
    logic::{noise_reduction::map_noise_cleanup, tile_map::TileMap},
};

#[derive(Clone, Debug)]
pub struct Board {
    pub map: TileMap,
    pub start: Pos,
    pub end: Pos,
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

    pub fn mutate(&self, factor: f32) -> Self {
        let mut rng = rand::rng();

        let mut ret = self.clone();

        for pos in self.map.all_inner_pos() {
            if rng.random::<f32>() < factor {
                match ret.map.at(&pos) {
                    Tile::Wall => ret.map.set(&pos, Tile::Ice),
                    Tile::Ice => ret.map.set(&pos, Tile::Wall),
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

        ret
    }

    pub fn box_cascade(&mut self, moved_ice_cube: &Pos, direction: &Direction) {
        assert!(self.map.at(moved_ice_cube) == Tile::Box);

        let next_pos = *moved_ice_cube + direction.vector();

        if self.map.at(&next_pos) == Tile::Ice {
            self.map.set(&next_pos, Tile::Box);
            self.map.set(moved_ice_cube, Tile::Ice);
            self.box_cascade(&next_pos, direction);
        }

        if self.map.at(&next_pos) == Tile::Box {
            self.box_cascade(&next_pos, direction);
        }
    }

    pub fn new_random() -> Result<Self, String> {
        let mut rng = rand::rng();
        let width = (7..15).choose(&mut rng).unwrap();
        let height = (7..15).choose(&mut rng).unwrap();

        let start_side = (0..3).choose(&mut rng).unwrap();
        let end_side = ((1..3).choose(&mut rng).unwrap() + start_side) % 4;

        let gate_range_horizontal = &(3..height - 3);
        let gate_range_vertical = &(3..width - 3);

        let (start, start_direction) = match start_side {
            0 => (
                Pos::new(0, gate_range_horizontal.clone().choose(&mut rng).unwrap()),
                Direction::East,
            ),
            1 => (
                Pos::new(
                    width - 1,
                    gate_range_horizontal.clone().choose(&mut rng).unwrap(),
                ),
                Direction::West,
            ),
            2 => (
                Pos::new(gate_range_vertical.clone().choose(&mut rng).unwrap(), 0),
                Direction::South,
            ),
            _ => (
                Pos::new(
                    gate_range_vertical.clone().choose(&mut rng).unwrap(),
                    height - 1,
                ),
                Direction::North,
            ),
        };

        let (end, end_direction) = match end_side {
            0 => (
                Pos::new(0, gate_range_horizontal.clone().choose(&mut rng).unwrap()),
                Direction::West,
            ),
            1 => (
                Pos::new(
                    width - 1,
                    gate_range_horizontal.clone().choose(&mut rng).unwrap(),
                ),
                Direction::East,
            ),
            2 => (
                Pos::new(gate_range_vertical.clone().choose(&mut rng).unwrap(), 0),
                Direction::North,
            ),
            _ => (
                Pos::new(
                    gate_range_vertical.clone().choose(&mut rng).unwrap(),
                    height - 1,
                ),
                Direction::South,
            ),
        };

        let mut map = vec![vec![Tile::Wall; width as usize]; height as usize];

        for x in 1..width - 1 {
            for y in 1..height - 1 {
                map[y as usize][x as usize] = Tile::Ice;
            }
        }

        let weak_walls = ((width * height) / 40..=(width * height) / 30)
            .choose(&mut rng)
            .unwrap();

        for _ in 0..weak_walls {
            let x = (1..(width - 1) as usize).choose(&mut rng).unwrap();
            let y = (1..(height - 1) as usize).choose(&mut rng).unwrap();

            map[y][x] = Tile::WeakWall;
        }

        let pilars = ((width * height) / 20..(width * height) / 10)
            .choose(&mut rng)
            .unwrap();

        for _ in 0..pilars {
            let x = (1..(width - 1) as usize).choose(&mut rng).unwrap();
            let y = (1..(height - 1) as usize).choose(&mut rng).unwrap();

            map[y][x] = Tile::Wall;
        }

        let boxes = ((width * height) / 40..=(width * height) / 30).choose(&mut rng).unwrap();

        for _ in 0..boxes {
            let x = (1..(width - 1) as usize).choose(&mut rng).unwrap();
            let y = (1..(height - 1) as usize).choose(&mut rng).unwrap();

            map[y][x] = Tile::Box;
        }

        let vignet = ((width * height) / 10..(width * height) / 5)
            .choose(&mut rng)
            .unwrap();

        for _ in 0..vignet {
            let x = (1..(width - 1) as usize).choose(&mut rng).unwrap();
            let y = (1..(height - 1) as usize).choose(&mut rng).unwrap();

            let normal_x = (x as f32 / width as f32) - 0.5;
            let normal_y = (y as f32 / height as f32) - 0.5;

            let normal_d = normal_x * normal_x + normal_y * normal_y;

            if random::<f32>() > normal_d * 2. {
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
            map,
            start,
            start_direction,
            end,
            end_direction,
        };

        Ok(ret)
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
            start: self.start.rotate_left(self.map.get_width()),
            end: self.end.rotate_left(self.map.get_width()),
            start_direction: self.start_direction.left(),
            end_direction: self.end_direction.left(),
            map: self.map.rotate_left(),
        }
    }

    pub fn at(&self, p: &Pos) -> Tile {
        self.map.at(p)
    }
}

pub struct TileMapWrap<'a> {
    pub base: &'a TileMap,
    pub p: Pos,
    pub tile: Tile,
}

impl<'a> TileMapWrap<'a> {
    pub fn at(&self, p: &Pos) -> Tile {
        if *p == self.p {
            self.tile
        } else {
            self.base.at(p)
        }
    }
}
