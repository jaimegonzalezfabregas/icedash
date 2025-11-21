use crate::{
    api::{board_description::BoardDescription, dart_board::DartBoard, direction::Direction},
    logic::worker_pool::{get_new_room, halt_search, start_search, stop_search},
};

pub trait LeftRotatable {
    fn rotate_left(&self) -> Self;
}

#[derive(Clone, PartialEq, Debug, Eq, Hash)]
pub struct EndOfGameMetadata {
    pub level: isize,
    pub gamemode_desc: String,
    pub return_gate: RoomIdAndGate,
    pub best_score_id: String,
}

#[derive(Clone, PartialEq, Debug, Eq, Hash)]
pub struct RoomIdAndGate {
    pub room_id: String,
    pub gate_id: isize,
}

#[derive(Clone, PartialEq, Debug, Eq, Hash)]
pub enum GateDestination {
    NextAutoGen,
    FirstAutogen {
        board_description: BoardDescription,
        board_count: isize,
        end_of_game_metadata: EndOfGameMetadata,
    },
    RoomIdWithGate(RoomIdAndGate),
}

impl GateDestination {
    pub fn get_gate_id(&self) -> isize {
        match self {
            GateDestination::NextAutoGen => 0,
            GateDestination::FirstAutogen { .. } => 0,
            GateDestination::RoomIdWithGate(RoomIdAndGate { gate_id, .. }) => *gate_id,
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
