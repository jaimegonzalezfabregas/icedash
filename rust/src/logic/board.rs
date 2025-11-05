use std::ops::{Deref, DerefMut};

use rand::{random, seq::IteratorRandom};

use crate::{
    api::main::{BoardDescription, Direction, GateDestination, GateMetadata, Tile},
    logic::{
        gate::GateEntry,
        matrix::{Matrix, TileMap},
        noise_reduction::asthetic_filter,
        pos::Pos,
    },
};

#[derive(Clone, Debug)]
pub struct Board {
    pub map: TileMap,
    pub gates: Vec<GateEntry>,
}

impl Deref for Board {
    type Target = TileMap;

    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

impl DerefMut for Board {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.map
    }
}

impl Board {
    pub fn print(&self, highlight: Vec<Pos>) {
        self.map.print(highlight);
    }

    pub fn get_gate_direction(&self, gate_id: usize) -> Direction {
        self.gates[gate_id].inwards_direction
    }

    pub fn get_gate_position(&self, gate_id: usize) -> Pos {
        self.gates[gate_id].pos
    }

    pub fn get_gate_destination(&self, gate_id: usize) -> Option<GateDestination> {
        if let Some(gate_entry) = &self.gates.get(gate_id) {
            if let Tile::Gate(metadata) = self.at(&gate_entry.pos) {
                match metadata {
                    GateMetadata::Exit { destination, .. } => Some(destination),
                    GateMetadata::EntryOnly => None,
                }
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn get_gate_label(&self, gate_id: usize) -> Option<String> {
        if let Some(gate_entry) = &self.gates.get(gate_id) {
            if let Tile::Gate(metadata) = self.at(&gate_entry.pos) {
                match metadata {
                    GateMetadata::Exit { label, .. } => label,
                    GateMetadata::EntryOnly => None,
                }
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn get_gate_id_by_pos(&self, p: Pos) -> Option<usize> {
        self.gates.iter().position(|gate| gate.pos == p)
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

    pub fn new_random(desc: &BoardDescription) -> Result<Self, String> {
        let mut rng = rand::rng();
        let width = (desc.size_range_min..=desc.size_range_max)
            .choose(&mut rng)
            .unwrap_or(10);
        let height = (desc.size_range_min..=desc.size_range_max)
            .clone()
            .into_iter()
            .choose(&mut rng)
            .unwrap_or(10);

        let gate_range_horizontal = &(3..height - 3);
        let gate_range_vertical = &(3..width - 3);

        let (start, start_direction) = (
            Pos::new(0, gate_range_horizontal.clone().choose(&mut rng).unwrap()),
            Direction::East,
        );
        let (end, end_direction) = match (1..3).choose(&mut rng).unwrap() {
            0 => (
                Pos::new(0, gate_range_horizontal.clone().choose(&mut rng).unwrap()),
                Direction::East,
            ),

            1 => (
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

        let mut map = vec![vec![Tile::Wall; width as usize]; height as usize];

        for x in 1..width - 1 {
            for y in 1..height - 1 {
                map[y as usize][x as usize] = Tile::Ice;
            }
        }

        for (percentage, tile) in [
            (
                ((desc.weak_walls_percentage_min..=desc.weak_walls_percentage_max)
                    .choose(&mut rng)
                    .unwrap_or(0)),
                Tile::WeakWall,
            ),
            (
                ((desc.pilars_percentage_min..=desc.pilars_percentage_max)
                    .choose(&mut rng)
                    .unwrap_or(0)),
                Tile::Wall,
            ),
            (
                ((desc.box_percentage_min..=desc.box_percentage_max)
                    .clone()
                    .into_iter()
                    .choose(&mut rng)
                    .unwrap_or(0)),
                Tile::Box,
            ),
        ] {
            for _ in 0..((width - 2) * (height - 2)) * percentage / 100 {
                let x = (1..(width - 1) as usize).choose(&mut rng).unwrap();
                let y = (1..(height - 1) as usize).choose(&mut rng).unwrap();

                map[y][x] = tile.clone();
            }
        }

        let vignet = (width * height)
            * ((desc.vignet_percentage_min..=desc.vignet_percentage_max)
                .choose(&mut rng)
                .unwrap_or(0))
            / 100;

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

        asthetic_filter(
            &mut map,
            &mut start,
            start_direction,
            &mut end,
            end_direction,
        );

        map.set(
            &end,
            Tile::Gate(GateMetadata::Exit {
                destination: GateDestination::NextAutoGen,
                label: None,
            }),
        );
        map.set(&start, Tile::Gate(GateMetadata::EntryOnly));

        let ret = Board {
            map,
            gates: vec![GateEntry::new(start, width), GateEntry::new(end, width)],
        };

        Ok(ret)
    }

    pub fn rotate_left(self) -> Self {
        let ret = Board {
            gates: self
                .gates
                .iter()
                .map(|e| e.rotate_left(self.get_width()))
                .collect(),
            map: self.map.rotate_left(),
        };
        return ret;
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
