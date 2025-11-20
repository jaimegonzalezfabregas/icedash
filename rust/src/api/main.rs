use crate::{
    api::{dart_board::DartBoard, direction::Direction},
    logic::worker_pool::{get_new_room, halt_search, start_search, stop_search},
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
        game_mode: Option<(String, String)>,
    },
    RoomIdWithGate {
        room_id: String,
        gate_id: isize,
    },
}

impl GateDestination {
    pub fn get_gate_id(&self) -> isize {
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
