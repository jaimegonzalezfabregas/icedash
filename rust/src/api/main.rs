use std::ops::{Add, AddAssign, Div, Mul, Sub, SubAssign};

use crate::logic::{
    board::Board,
    creature::Creature,
    tile_map::TileMap,
    worker_pool::{get_new_room, start_search, worker_halt},
};
#[derive(Clone)]
pub struct Neighbour {
    pub center: Tile,
    pub north: Tile,
    pub south: Tile,
    pub east: Tile,
    pub west: Tile,
    pub northwest: Tile,
    pub northeast: Tile,
    pub southwest: Tile,
    pub southeast: Tile,
}

pub struct NeighbourBool {
    pub center: bool,
    pub north: bool,
    pub south: bool,
    pub east: bool,
    pub west: bool,
    pub northwest: bool,
    pub northeast: bool,
    pub southwest: bool,
    pub southeast: bool,
}

impl Neighbour {
    pub fn mask_center(&self, tile: Tile) -> Self {
        let mut ret = self.clone();
        ret.center = tile;
        ret
    }

    pub fn to_stops_player_during_gameplay(&self) -> NeighbourBool {
        NeighbourBool {
            center: self.center.stops_player_during_gameplay(),
            north: self.north.stops_player_during_gameplay(),
            south: self.south.stops_player_during_gameplay(),
            east: self.east.stops_player_during_gameplay(),
            west: self.west.stops_player_during_gameplay(),
            northwest: self.northwest.stops_player_during_gameplay(),
            northeast: self.northeast.stops_player_during_gameplay(),
            southwest: self.southwest.stops_player_during_gameplay(),
            southeast: self.southeast.stops_player_during_gameplay(),
        }
    }

    pub fn get_asset(&self) -> Option<String> {
        match self.center {
            Tile::Entrance => Some("ice.png".into()),
            Tile::Gate => Some("ice.png".into()),
            Tile::Ice => Some("ice.png".into()),
            Tile::WeakWall => Some("ice.png".into()),
            Tile::Box => Some("ice.png".into()),
            Tile::Outside => None,
            Tile::Wall => match self.to_stops_player_during_gameplay() {
                NeighbourBool {
                    north: true,
                    south: true,
                    east: true,
                    west: true,
                    northwest: true,
                    northeast: true,
                    southwest: true,
                    southeast: true,
                    ..
                } => None,
                NeighbourBool {
                    north: false,
                    south: false,
                    ..
                } => Some("wall_s.png".into()),
                NeighbourBool {
                    north: false,
                    south: _,
                    east: false,
                    west: false,
                    ..
                } => Some("wall_new.png".into()),
                NeighbourBool {
                    north: false,
                    east: false,
                    ..
                } => Some("wall_ne.png".into()),
                NeighbourBool {
                    north: false,
                    west: false,
                    ..
                } => Some("wall_nw.png".into()),
                NeighbourBool { north: false, .. } => Some("wall_n.png".into()),
                NeighbourBool { south: false, .. } => Some("wall_s.png".into()),
                NeighbourBool { east: false, .. } => Some("wall_e.png".into()),
                NeighbourBool { west: false, .. } => Some("wall_w.png".into()),
                NeighbourBool {
                    south: true,
                    east: true,
                    southeast: false,
                    ..
                } => Some("wall_e.png".into()),
                NeighbourBool {
                    south: true,
                    west: true,
                    southwest: false,
                    ..
                } => Some("wall_w.png".into()),
                NeighbourBool {
                    north: true,
                    east: true,
                    west: true,
                    northwest: false,
                    northeast: false,
                    ..
                } => Some("wall__ne-nw.png".into()),
                NeighbourBool {
                    north: true,
                    west: true,
                    northwest: false,
                    ..
                } => Some("wall__nw.png".into()),
                NeighbourBool {
                    north: true,
                    east: true,
                    northeast: false,
                    ..
                } => Some("wall__ne.png".into()),
            }
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Copy)]
pub struct Pos {
    pub x: isize,
    pub y: isize,
}

impl Pos {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    pub fn dart_vector(&self) -> Vec<f32> {
        vec![self.x as f32, self.y as f32]
    }

    pub(crate) fn rotate_left(self, width: isize) -> Pos {
        // (1,1), (-1,1)
        Self {
            x: self.y,
            y: -self.x + width - 1,
        }
    }
}

impl Add<Pos> for Pos {
    type Output = Self;

    fn add(self, rhs: Pos) -> Self::Output {
        Pos::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl AddAssign for Pos {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl SubAssign for Pos {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Sub<Pos> for Pos {
    type Output = Self;

    fn sub(mut self, rhs: Pos) -> Self::Output {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self
    }
}

impl Div<isize> for Pos {
    type Output = Pos;

    fn div(self, rhs: isize) -> Self::Output {
        Pos::new(self.x / rhs, self.y / rhs)
    }
}

impl Mul<isize> for Pos {
    type Output = Pos;

    fn mul(self, rhs: isize) -> Self::Output {
        Pos::new(self.x * rhs, self.y * rhs)
    }
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

#[derive(Clone, PartialEq, Copy, Debug, Eq, Hash)]
pub enum Tile {
    Entrance,
    Gate,
    Wall,
    Ice,
    WeakWall,
    Box,
    Outside,
}

impl Tile {
    pub fn symbol(&self) -> &str {
        match self {
            Tile::Entrance => "E",
            Tile::Gate => "G",
            Tile::Wall => "#",
            Tile::Ice => " ",
            Tile::WeakWall => "w",
            Tile::Outside => " ",
            Tile::Box => "b",
        }
    }

    pub fn stops_player_during_sim(&self) -> bool {
        match self {
            Tile::Entrance => true,
            Tile::Gate => false,
            Tile::Wall => true,
            Tile::Ice => false,
            Tile::WeakWall => true,
            Tile::Outside => true,
            Tile::Box => true,
        }
    }

    pub fn stops_player_during_gameplay(&self) -> bool {
        match self {
            Tile::Entrance => true,
            Tile::Gate => false,
            Tile::Wall => true,
            Tile::Ice => false,
            Tile::WeakWall => false,
            Tile::Outside => true,
            Tile::Box => false,
        }
    }

    pub fn stops_box_during_gameplay(&self) -> bool{
        match self{
            Tile::Entrance => true,
            Tile::Gate => true,
            Tile::Wall => true,
            Tile::Ice => false,
            Tile::WeakWall => false,
            Tile::Outside => true,
            Tile::Box => false,
        }
    }

    pub(crate) fn from_symbol(symbol: u8) -> Tile {
        match symbol {
            b'E' => Tile::Entrance,
            b'G' => Tile::Gate,
            b'#' => Tile::Wall,
            b' ' => Tile::Ice,
            b'w' => Tile::WeakWall,
            b'b' => Tile::Box,
            _ => Tile::Outside,
        }
    }
}

#[derive(Clone)]
pub enum Room {
    Lobby(Board),
    Trial(Creature),
}

impl Room {
    pub fn get_start_direction(&self) -> Direction {
        match self {
            Room::Lobby(board) => board.start_direction,
            Room::Trial(creature) => creature.board.start_direction,
        }
    }

    pub fn rotate_left(self) -> Self {
        match self {
            Room::Lobby(board) => Room::Lobby(board.rotate_left()),
            Room::Trial(mut creature) => {
                creature.board = creature.board.rotate_left();
                creature.board.print(vec![]);
                Room::Trial(creature)
            }
        }
    }

    pub fn get_board(&self) -> Board {
        match self {
            Room::Lobby(board) => board,
            Room::Trial(creature) => &creature.board,
        }
        .to_owned()
    }

    pub fn get_width(&self) -> isize {
        self.get_board().get_width()
    }

    pub fn get_max_movement_count(&self) -> Option<isize> {
        match self {
            Room::Lobby(_) => None,
            Room::Trial(creature) => Some(creature.analysis.optimal_movement_count as isize),
        }
    }

    pub fn get_map(&self) -> TileMap {
        self.get_board().map.clone()
    }

    pub fn get_height(&self) -> isize {
        self.get_board().get_height()
    }

    pub fn get_start(&self) -> Pos {
        self.get_board().start
    }

    pub fn get_end(&self) -> Pos {
        self.get_board().end
    }

    pub fn get_all_positions(&self) -> Vec<Pos> {
        self.get_board().map.all_pos().collect()
    }

    pub fn at(&self, pos: &Pos) -> Tile {
        self.get_map().at(pos)
    }

    pub fn neighbour_at(&self, pos: &Pos) -> Neighbour {
        self.get_map().neighbour_at(pos)
    }

    pub fn set_tile_at(&self, pos: &Pos, tile: Tile) -> Self {
        let mut ret = self.to_owned();
        match ret {
            Room::Lobby(ref mut board) => {
                board.map.set(pos, tile);
            }
            Room::Trial(ref mut creature) => {
                creature.board.map.set(pos, tile);
            }
        }
        ret
    }
}

pub fn dart_get_new_board() -> Room {
    get_new_room()
}

pub fn dart_worker_halt(millis: usize) {
    worker_halt(millis)
}

use cap::Cap;
use std::alloc;

#[global_allocator]
static ALLOCATOR: Cap<alloc::System> = Cap::new(alloc::System, usize::max_value());

#[flutter_rust_bridge::frb(init)]
pub fn init_app() {
    ALLOCATOR.set_limit(5 * 1024 * 1024 * 1024).unwrap();

    flutter_rust_bridge::setup_default_user_utils();
    start_search();
}
