use crate::api::main::{Direction, Pos};

#[derive(Clone, Debug)]
pub struct Gate {
    pub destination_room_and_gate_id: Option<(String, usize)>,
    pub pos: Pos,
    pub entry_direction: Direction,
}

impl Gate {
    pub fn rotate_left(&self, width: isize) -> Gate {
        Gate {
            destination_room_and_gate_id: self.destination_room_and_gate_id.clone(),
            pos: self.pos.rotate_left(width),
            entry_direction: self.entry_direction.left(),
        }
    }
}
