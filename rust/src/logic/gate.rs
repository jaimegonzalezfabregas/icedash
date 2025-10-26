use crate::api::main::{Direction, Pos};

#[derive(Clone, Debug)]
pub struct Gate {
    pub destination_room_and_gate_id: Option<(String, usize)>,
    pub pos: Pos,
    pub inwards_direction: Direction,
}

impl Gate {
    pub fn rotate_left(&self, width: isize) -> Gate {
        Gate {
            destination_room_and_gate_id: self.destination_room_and_gate_id.clone(),
            pos: self.pos.rotate_left(width),
            inwards_direction: self.inwards_direction.left(),
        }
    }

    pub fn new(dest: Option<(String, usize)>, p: Pos, width: isize) -> Gate{
        Gate {
            destination_room_and_gate_id: dest,
            pos: p,
            inwards_direction: if p.x == 0 {
                Direction::East
            } else if p.y == 0 {
                Direction::South
            } else if p.x == width - 1 {
                Direction::West
            } else {
                Direction::North
            },
        }
    }
}
