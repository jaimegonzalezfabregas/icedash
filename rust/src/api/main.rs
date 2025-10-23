use std::ops::{Add, AddAssign, Deref, Div, Mul, Sub, SubAssign};

use crate::logic::{
    board::Board,
    solver::Analysis,
    matrix::{Matrix, TileMap},
    worker_pool::{get_new_room, start_search, worker_halt},
};
#[derive(Clone)]
pub struct Neighbour<T> {
    pub center: T,
    pub north: T,
    pub south: T,
    pub east: T,
    pub west: T,
    pub northwest: T,
    pub northeast: T,
    pub southwest: T,
    pub southeast: T,
}

impl Neighbour<Tile> {
    pub fn mask_center(&self, tile: Tile) -> Self {
        let mut ret = self.clone();
        ret.center = tile;
        ret
    }

    pub fn to_stops_player_during_gameplay(&self) -> Neighbour<bool> {
        Neighbour {
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

    pub fn get_asset(&self) -> Option<(String, isize)> {
        match self.center {
            Tile::Entrance => Some(("ice.png".into(), 0)),
            Tile::Gate => Some(("ice.png".into(), 0)),
            Tile::Ice => Some(("ice.png".into(), 0)),
            Tile::WeakWall => Some(("ice.png".into(), 0)),
            Tile::Box => Some(("ice.png".into(), 0)),
            Tile::Outside => None,
            Tile::Wall => {
                let mut rotator = self.clone();
                let mut ret = None;
                let mut ret_priority = 0;

                for i in 0..4 {
                    let (priority, new_ret) = match rotator.to_stops_player_during_gameplay() {
                        Neighbour {
                            southwest: false,
                            south: true,
                            west: true,
                            north: true,
                            east: true,
                            ..
                        } => (300, Some(("wall_corner_in.png".into(), i))),
                        Neighbour {
                            south: false,
                            west: false,
                            north: true,
                            east: true,
                            northeast: true,
                            ..
                        } => (200, Some(("wall_corner_out.png".into(), i))),
                        Neighbour {
                            south: false,
                            north: true,
                            ..
                        } => (100, Some(("wall.png".into(), i))),

                        _ => (0, None),
                    };
                    if priority > ret_priority {
                        ret_priority = priority;
                        ret = new_ret;
                    }

                    rotator = rotator.rotate_left();
                }

                return ret;
            }
        }
    }
}

impl<T: Copy> Neighbour<T> {
    fn rotate_left(&self) -> Neighbour<T> {
        Neighbour {
            center: self.center,
            north: self.east,
            south: self.west,
            east: self.south,
            west: self.north,
            northeast: self.southeast,
            southeast: self.southwest,
            northwest: self.northeast,
            southwest: self.northwest,
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

impl Default for Tile {
    fn default() -> Self {
        Tile::Outside
    }
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

    pub fn stops_box_during_gameplay(&self) -> bool {
        match self {
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

type AssetMap = Matrix<Option<(String, isize)>>;

impl Matrix<Option<(String, isize)>> {
    fn from_tilemap(tilemap: &TileMap) -> Self {
        let mut ret = Matrix::new(tilemap.get_width(), tilemap.get_height());

        for p in tilemap.all_pos() {
            ret.set(&p, tilemap.neighbour_at(&p).get_asset());
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
    pub(crate) fn new(board: Board, analysis: Analysis) -> Self {
        Self {
            asset_map: AssetMap::from_tilemap(&board.map),
            board,
            analysis: Some(analysis),
        }
    }

    pub fn new_lobby(
        serialized: String,
        start: Pos,
        end: Pos,
        start_direction: Direction,
        end_direction: Direction,
    ) -> Self {
        Self {
            asset_map: AssetMap::from_tilemap(&TileMap::from_print(&serialized)),
            board: Board {
                map: TileMap::from_print(&serialized),
                start,
                end,
                start_direction,
                end_direction,
            },
            analysis: None,
        }
    }

    pub fn get_start_direction(&self) -> Direction {
        self.start_direction
    }

    pub fn get_end_direction(&self) -> Direction {
        self.start_direction
    }

    pub fn rotate_left(&self) -> Self {
        Self {
            board: self.board.clone().rotate_left(),
            asset_map: self.asset_map.clone().rotate_left(),
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

    pub fn get_end(&self) -> Pos {
        self.end
    }

    pub fn get_start(&self) -> Pos {
        self.end
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
}

pub fn dart_get_new_board() -> DartBoard {
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
