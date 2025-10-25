use std::ops::Deref;

use rand::{random, seq::IteratorRandom};

use crate::{
    api::main::{Direction, Pos, Tile},
    logic::{
        gate::Gate,
        matrix::{Matrix, TileMap},
        noise_reduction::map_noise_cleanup,
    },
};

#[derive(Clone, Debug)]
pub struct Board {
    pub map: TileMap,
    pub gates: Vec<Gate>,
}

impl Deref for Board {
    type Target = TileMap;

    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

impl Board {
    pub fn get_gate_direction(&self, gate_id: usize) -> Direction {
        self.gates[gate_id].entry_direction
    }

    pub fn get_gate_position(&self, gate_id: usize) -> Pos {
        self.gates[gate_id].pos
    }

    pub fn get_gate_destination(&self, gate_id: usize) -> Option<(String, usize)> {
        self.gates[gate_id]
            .destination_room_and_gate_id
            .clone()
    }

    pub fn get_gate_id_by_pos(&self, p: Pos) -> Option<usize> {
        self
            .gates
            .iter()
            .position(|gate| gate.pos == p)
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

        let gate_range_horizontal = &(3..height - 3);
        let gate_range_vertical = &(3..width - 3);

        let (start, start_direction) = (
            Pos::new(0, gate_range_horizontal.clone().choose(&mut rng).unwrap()),
            Direction::East,
        );
        let (end, end_direction) = match (1..3).choose(&mut rng).unwrap() {
            0 => (
                Pos::new(0, gate_range_horizontal.clone().choose(&mut rng).unwrap()),
                Direction::West,
            ),

            1 => (
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

        let boxes = ((width * height) / 40..=(width * height) / 30)
            .choose(&mut rng)
            .unwrap();

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

        let mut map = Matrix(map);

        map_noise_cleanup(
            &mut map,
            &mut start,
            start_direction,
            &mut end,
            end_direction,
        );

        let ret = Board {
            map,
            gates: vec![
                Gate {
                    destination_room_and_gate_id: None,
                    pos: start,
                    entry_direction: start_direction,
                },
                Gate {
                    destination_room_and_gate_id: Some(("\\next_autogen".into(), 0)),
                    pos: end,
                    entry_direction: end_direction,
                },
            ],
        };

        Ok(ret)
    }

    pub fn rotate_left(self) -> Self {
        Board {
            gates: self
                .gates
                .iter()
                .map(|e| e.rotate_left(self.get_width()))
                .collect(),
            map: self.map.rotate_left(),
        }
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
            self.tile.clone()
        } else {
            self.base.at(p)
        }
    }
}
