use std::{collections::HashMap, ops::Deref};

use crate::logic::{
    board::Board,
    gate::GateEntry,
    matrix::{Matrix, TileMap},
    pos::Pos,
    solver::Analysis,
    worker_pool::{get_new_room, load_board_description_stack, worker_halt},
};

#[frb(sync)]
pub fn pos2dart_vector(p: Pos) -> Vec<f32> {
    p.dart_vector()
}

#[derive(Clone, PartialEq, Copy, Debug)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    pub fn icon(&self) -> &str {
        match self {
            Direction::North => "^",
            Direction::South => "v",
            Direction::East => ">",
            Direction::West => "<",
        }
    }

    pub(crate) fn vector(&self) -> Pos {
        match self {
            Direction::North => Pos::new(0, -1),
            Direction::South => Pos::new(0, 1),
            Direction::East => Pos::new(1, 0),
            Direction::West => Pos::new(-1, 0),
        }
    }

    #[frb(sync)]
    pub fn dart_vector(&self) -> Vec<f32> {
        match self {
            Direction::North => vec![0., -1.],
            Direction::South => vec![0., 1.],
            Direction::East => vec![1., 0.],
            Direction::West => vec![-1., 0.],
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

    pub fn left(&self) -> Self {
        match self {
            Direction::North => Direction::West,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
            Direction::East => Direction::North,
        }
    }

    pub fn right(&self) -> Self {
        self.left().reverse()
    }

    pub(crate) fn all() -> Vec<Direction> {
        vec![
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ]
    }
}

#[derive(Clone, PartialEq, Debug, Eq, Hash)]
pub struct BoardDescription {
    pub size_range_min: isize,
    pub size_range_max: isize,
    pub weak_walls_percentage_min: isize,
    pub weak_walls_percentage_max: isize,
    pub pilars_percentage_min: isize,
    pub pilars_percentage_max: isize,
    pub box_percentage_min: isize,
    pub box_percentage_max: isize,
    pub vignet_percentage_min: isize,
    pub vignet_percentage_max: isize,
}

#[derive(Clone, PartialEq, Debug, Eq, Hash)]
pub enum GateDestination {
    NextAutoGen,
    FirstAutogen { board_description_stack: Vec<BoardDescription> },
    RoomIdWithGate { room_id: String, gate_id: usize },
}

impl GateDestination {
    pub fn get_gate_id(&self) -> usize {
        match self {
            GateDestination::NextAutoGen => 0,
            GateDestination::FirstAutogen { .. } => 0,
            GateDestination::RoomIdWithGate { gate_id, .. } => *gate_id,
        }
    }
}

#[derive(Clone, PartialEq, Debug, Eq, Hash)]
pub enum GateMetadata {
    Exit {
        destination: GateDestination,
        label: Option<String>,
    },
    EntryOnly,
}

#[derive(Clone, PartialEq, Debug, Eq, Hash)]
pub enum Tile {
    Gate(GateMetadata),
    Wall,
    Ice,
    Stop,
    WeakWall,
    Box,
    Outside,
}

impl Default for Tile {
    fn default() -> Self {
        Tile::Outside
    }
}

impl Tile {
    pub fn symbol(&self) -> &str {
        match self {
            Tile::Gate(GateMetadata::EntryOnly) => "E",
            Tile::Gate(_) => "G",
            Tile::Wall => "#",
            Tile::Stop => "s",
            Tile::Ice => " ",
            Tile::WeakWall => "w",
            Tile::Outside => " ",
            Tile::Box => "b",
        }
    }

    pub fn stops_player_during_sim(&self) -> bool {
        match self {
            Tile::Gate(GateMetadata::EntryOnly) => true,
            Tile::Gate(_) => false,
            Tile::Wall => true,
            Tile::Stop => false,
            Tile::Ice => false,
            Tile::WeakWall => true,
            Tile::Outside => true,
            Tile::Box => true,
        }
    }

    pub fn stops_player_during_gameplay(&self) -> bool {
        match self {
            Tile::Gate(GateMetadata::EntryOnly) => true,
            Tile::Gate(_) => true,
            Tile::Wall => true,
            Tile::Stop => false,
            Tile::Ice => false,
            Tile::WeakWall => false,
            Tile::Outside => true,
            Tile::Box => false,
        }
    }

    pub fn is_a_wall_for_texturing(&self) -> bool {
        match self {
            Tile::Gate(GateMetadata::EntryOnly) => true,
            Tile::Gate(_) => false,
            Tile::Wall => true,
            Tile::Stop => false,
            Tile::Ice => false,
            Tile::WeakWall => false,
            Tile::Outside => true,
            Tile::Box => false,
        }
    }

    pub fn stops_box_during_gameplay(&self) -> bool {
        match self {
            Tile::Gate(_) => true,
            Tile::Wall => true,
            Tile::Stop => false,
            Tile::Ice => false,
            Tile::WeakWall => false,
            Tile::Outside => true,
            Tile::Box => false,
        }
    }

    pub(crate) fn from_symbol(symbol: u8, gate_metadata: &HashMap<u8, GateMetadata>) -> Tile {
        match symbol {
            b'#' => Tile::Wall,
            b' ' => Tile::Ice,
            b'w' => Tile::WeakWall,
            b'b' => Tile::Box,
            b's' => Tile::Stop,
            e => {
                let metadata = gate_metadata.get(&e).cloned();
                Tile::Gate(metadata.unwrap_or(GateMetadata::EntryOnly))
            }
        }
    }
}

type AssetMap = Matrix<Option<(String, isize)>>;

impl Matrix<Option<(String, isize)>> {
    fn from_tilemap(tilemap: &TileMap) -> Self {
        let mut ret = Matrix::new(tilemap.get_width(), tilemap.get_height());
        let mut wip_tilemap = tilemap.clone();

        let mut rep = true;

        while rep {
            rep = false;

            // Decorate Interior
            for p in wip_tilemap.all_pos().collect::<Vec<_>>() {
                let tile = wip_tilemap.at(&p);

                if let Tile::Wall = tile {
                    let mut neigh_count = 0;
                    for delta in Direction::all() {
                        if wip_tilemap
                            .at(&(p + delta.vector()))
                            .is_a_wall_for_texturing()
                        {
                            neigh_count += 1;
                        }
                    }
                    if neigh_count < 2 {
                        ret.set(&p, Some(("1x1_obstacle.png".into(), 0)));
                        wip_tilemap.set(&p, Tile::Ice);
                        rep = true;
                    }

                    if !wip_tilemap
                        .at(&(p + Pos::new(1, 0)))
                        .is_a_wall_for_texturing()
                        && !wip_tilemap
                            .at(&(p + Pos::new(-1, 0)))
                            .is_a_wall_for_texturing()
                    {
                        ret.set(&p, Some(("1x1_obstacle.png".into(), 0)));
                        wip_tilemap.set(&p, Tile::Ice);
                        rep = true;
                    }

                    if !wip_tilemap
                        .at(&(p + Pos::new(0, 1)))
                        .is_a_wall_for_texturing()
                        && !wip_tilemap
                            .at(&(p + Pos::new(0, -1)))
                            .is_a_wall_for_texturing()
                    {
                        ret.set(&p, Some(("1x1_obstacle.png".into(), 0)));
                        wip_tilemap.set(&p, Tile::Ice);
                        rep = true;
                    }
                }
            }
        }

        for p in wip_tilemap.all_pos() {
            if let None = ret.at(&p) {
                ret.set(&p, wip_tilemap.neighbour_at(&p).get_asset());
            }
        }

        ret
    }
}

#[derive(Clone)]
pub struct DartBoard {
    pub board: Board,
    pub asset_map: AssetMap,
    pub analysis: Option<Analysis>,
}

impl Deref for DartBoard {
    type Target = Board;

    fn deref(&self) -> &Self::Target {
        &self.board
    }
}

impl DartBoard {
    #[frb(sync)]
    pub fn get_gate_direction(&self, gate_id: usize) -> Direction {
        self.board.get_gate_direction(gate_id)
    }

    #[frb(sync)]
    pub fn get_gate_position(&self, gate_id: usize) -> Pos {
        self.board.get_gate_position(gate_id)
    }

    pub fn get_gate_destination(&self, gate_id: usize) -> Option<GateDestination> {
        self.board.get_gate_destination(gate_id)
    }

    pub fn get_gate_label(&self, gate_id: usize) -> Option<String> {
        self.board.get_gate_label(gate_id)
    }

    pub fn get_gate_id_by_pos(&self, p: Pos) -> Option<usize> {
        self.board.get_gate_id_by_pos(p)
    }
    pub(crate) fn new(board: Board, analysis: Analysis) -> Self {
        Self {
            asset_map: AssetMap::from_tilemap(&board.map),
            board,
            analysis: Some(analysis),
        }
    }

    pub fn new_lobby(serialized: String, gate_metadata: HashMap<u8, GateMetadata>) -> Self {
        let mut map: Vec<Vec<Tile>> = vec![];
        let mut gates = vec![];
        let mut x: usize;
        let mut y: usize = 0;

        for line in serialized.split("\n") {
            let mut line = line.as_bytes();
            let mut row = vec![];
            x = 0;

            while line.len() != 0 {
                let tile = Tile::from_symbol(line[0], &gate_metadata);

                if let Tile::Gate(_) = tile {
                    gates.push(GateEntry::new(
                        Pos::new(x as isize, y as isize),
                        map.get(0).unwrap_or(&vec![]).len() as isize,
                    ));
                }

                x += 1;

                row.push(tile.clone());

                line = &line[2..];
            }
            if row.len() != 0 {
                y += 1;
                map.push(row)
            }
        }

        Matrix(map.clone()).print(vec![]);

        Self {
            asset_map: AssetMap::from_tilemap(&Matrix(map.clone())),
            board: Board {
                map: Matrix(map),
                gates,
            },
            analysis: None,
        }
    }

    #[frb(sync)]
    pub fn rotate_left(&self) -> Self {
        Self {
            board: self.board.clone().rotate_left(),
            asset_map: self
                .asset_map
                .clone()
                .rotate_left()
                .map(|asset| match asset {
                    Some((asset, rotation)) => Some((asset, rotation - 1)),
                    None => None,
                }),
            analysis: self.analysis.clone(),
        }
    }

    pub fn get_width(&self) -> isize {
        self.map.get_width()
    }

    pub fn get_height(&self) -> isize {
        self.map.get_height()
    }

    pub fn get_max_movement_count(&self) -> Option<isize> {
        self.analysis
            .clone()
            .map(|analysis| analysis.optimal_movement_count as isize)
    }

    pub fn at(&self, p: &Pos) -> Tile {
        self.map.at(p)
    }

    pub fn asset_at(&self, p: &Pos) -> Option<(String, isize)> {
        self.asset_map.at(p)
    }

    pub fn get_all_positions(&self) -> Vec<Pos> {
        self.map.all_pos().collect()
    }

    pub fn print(&self) {
        self.board.print(vec![]);
    }
}

#[frb(non_opaque)]
pub enum AutoGenOutput {
    NoMoreDescriptionsLoaded,
    NotReady,
    Ok(DartBoard),
}

pub fn dart_get_new_board() -> AutoGenOutput {
    get_new_room()
}

pub fn dart_worker_halt(millis: usize) {
    worker_halt(millis)
}

pub fn dart_load_board_description_stack(board_desc_stack: Vec<BoardDescription>) {
    load_board_description_stack(board_desc_stack)
}
// use cap::Cap;
use flutter_rust_bridge::frb;
// use std::alloc;

// #[global_allocator]
// static ALLOCATOR: Cap<alloc::System> = Cap::new(alloc::System, usize::max_value());

#[frb(init)]
pub fn init_app() {
    // ALLOCATOR.set_limit(5 * 1024 * 1024 * 1024).unwrap();

    flutter_rust_bridge::setup_default_user_utils();
}
