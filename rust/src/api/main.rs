use std::{collections::HashMap, ops::Deref};

use crate::{
    api::{direction::Direction, pos::Pos, tile::Tile},
    logic::{
        board::Board,
        gate::GateEntry,
        matrix::{Matrix, TileMap},
        solver::Analysis,
        worker_pool::{get_new_room, halt_search, start_search, stop_search},
    },
};

pub trait LeftRotatable {
    fn rotate_left(&self) -> Self;
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

impl BoardDescription {
    pub fn from_list(data: Vec<isize>) -> BoardDescription {
        BoardDescription {
            size_range_min: data[0],
            size_range_max: data[1],
            weak_walls_percentage_min: data[2],
            weak_walls_percentage_max: data[3],
            pilars_percentage_min: data[4],
            pilars_percentage_max: data[5],
            box_percentage_min: data[6],
            box_percentage_max: data[7],
            vignet_percentage_min: data[8],
            vignet_percentage_max: data[9],
        }
    }

    pub fn as_list(&self) -> Vec<isize> {
        vec![
            self.size_range_min,
            self.size_range_max,
            self.weak_walls_percentage_min,
            self.weak_walls_percentage_max,
            self.pilars_percentage_min,
            self.pilars_percentage_max,
            self.box_percentage_min,
            self.box_percentage_max,
            self.vignet_percentage_min,
            self.vignet_percentage_max,
        ]
    }
}

#[derive(Clone, PartialEq, Debug, Eq, Hash)]
pub enum GateDestination {
    NextAutoGen,
    FirstAutogen {
        board_description: BoardDescription,
        board_count: isize,
        game_mode: Option<String>,
    },
    RoomIdWithGate {
        room_id: String,
        gate_id: usize,
        game_mode: Option<String>,
    },
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
    pub fn get_gate_direction(&self, gate_id: usize) -> Direction {
        self.board.get_gate_direction(gate_id)
    }

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

    pub fn new_lobby(
        serialized: String,
        gate_metadata: HashMap<u8, GateMetadata>,
        mut sign_text: Vec<(String, isize, isize)>,
    ) -> Self {
        let mut map: Vec<Vec<Tile>> = vec![];
        let mut gates = vec![];
        let mut x: usize;
        let mut y: usize = 0;

        for line in serialized.split("\n") {
            let mut line = line.as_bytes();
            let mut row = vec![];
            x = 0;

            while line.len() != 0 {
                let tile = Tile::from_symbol(line[0], &gate_metadata, &mut sign_text);

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

    pub fn rotate_left(&self) -> Self {
        Self {
            board: self.board.clone().rotate_left(),
            asset_map: self.asset_map.clone().rotate_left_keeping_elements().map(
                |asset| match asset {
                    Some((asset, rotation)) => Some((asset, rotation - 1)),
                    None => None,
                },
            ),
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
    NotReady,
    Ok(DartBoard),
    NoMoreBufferedBoards,
}

pub fn dart_start_search(board_desc: BoardDescription, max_buffered_boards: isize) {
    start_search(board_desc, max_buffered_boards);
}

pub fn dart_get_new_board(entry_direction: Direction) -> AutoGenOutput {
    get_new_room(entry_direction)
}

pub fn dart_worker_halt(millis: usize) {
    halt_search(millis)
}

pub fn dart_stop_search() {
    stop_search()
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
