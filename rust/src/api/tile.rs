use std::collections::HashMap;

use crate::api::main::{GateMetadata, LeftRotatable};

#[derive(Clone, PartialEq, Debug, Eq, Hash)]
pub enum Tile {
    Gate(GateMetadata),
    Wall,
    Ice,
    Stop,
    WeakWall,
    Box,
    Outside,
    Sign {
        text: String,
        width: isize,
        height: isize,
    },
}

impl LeftRotatable for Tile {
    fn rotate_left(&self) -> Self {
        match self {
            Tile::Gate(metadata) => Tile::Gate(metadata.clone()),
            Tile::Wall => Tile::Wall,
            Tile::Ice => Tile::Ice,
            Tile::Stop => Tile::Stop,
            Tile::WeakWall => Tile::WeakWall,
            Tile::Outside => Tile::Outside,
            Tile::Box => Tile::Box,
            Tile::Sign { text, width, height } => Tile::Sign {
                text: text.clone(),
                width: *height,
                height: *width,
            },
        }
    }
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
            Tile::Sign { .. } => "S",
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
            Tile::Sign { .. } => false,
        }
    }

    pub fn stops_player_during_gameplay(&self, predicting: bool) -> bool {
        match self {
            Tile::Gate(GateMetadata::EntryOnly) => true,
            Tile::Gate(_) => predicting,
            Tile::Wall => true,
            Tile::Stop => false,
            Tile::Ice => false,
            Tile::WeakWall => false,
            Tile::Outside => true,
            Tile::Box => false,
            Tile::Sign { .. } => false,
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
            Tile::Sign { .. } => false,
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
            Tile::Sign { .. } => false,
        }
    }

    pub(crate) fn from_symbol(
        symbol: u8,
        gate_metadata: &HashMap<u8, GateMetadata>,
        sign_metadata: &mut Vec<(String, isize, isize)>,
    ) -> Tile {
        match symbol {
            b'#' => Tile::Wall,
            b' ' => Tile::Ice,
            b'w' => Tile::WeakWall,
            b'b' => Tile::Box,
            b's' => Tile::Stop,
            b'S' => {
                let metadata = sign_metadata.remove(0);
                Tile::Sign {
                    text: metadata.0,
                    width: metadata.1,
                    height: metadata.2,
                }
            }
            e => {
                let metadata = gate_metadata.get(&e).cloned();
                Tile::Gate(metadata.unwrap_or(GateMetadata::EntryOnly))
            }
        }
    }
}
