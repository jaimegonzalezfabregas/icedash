use crate::{
    api::main::{Direction},
    logic::pos::Pos,
};

#[derive(Clone, Debug)]
pub struct GateEntry {
    pub pos: Pos,
    pub inwards_direction: Direction,
}

impl GateEntry {
    pub fn new(p: Pos, width: isize) -> GateEntry {
        GateEntry {
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

    pub fn rotate_left(&self, width: isize) -> GateEntry {
        GateEntry {
            pos: self.pos.rotate_left(width), // Width is not needed here
            inwards_direction: self.inwards_direction.left(),
        }
    }
}
