use std::{collections::HashMap, ops::Deref};

use crate::{
    api::{
        asset_map::AssetMap,
        direction::Direction,
        main::{GateDestination, GateMetadata, LeftRotatable},
        pos::Pos,
        tile::Tile,
    },
    logic::{board::Board, gate::GateEntry, matrix::Matrix, solver::Analysis},
};

#[derive(Clone)]
pub struct DartBoard {
    pub board: Board,
    pub asset_map: AssetMap,
    pub analysis: Option<Analysis>,
    pub gate_destinations: Vec<Option<GateDestination>>,
    pub gate_positions: Vec<Pos>,
    pub gate_directions: Vec<Direction>,
    pub gate_lables: Vec<Option<String>>,
    pub gate_pos_to_id: HashMap<Pos, isize>,
    pub width: isize,
    pub height: isize,
    pub max_movement_count: Option<isize>,
}

impl Deref for DartBoard {
    type Target = Board;

    fn deref(&self) -> &Self::Target {
        &self.board
    }
}

impl DartBoard {
    pub(crate) fn new(board: Board, analysis: Option<Analysis>) -> Self {
        Self {
            asset_map: AssetMap::from_tilemap(&board.map),
            max_movement_count: analysis
                .clone()
                .map(|analysis| analysis.optimal_movement_count as isize),
            gate_directions: board.gates.iter().map(|g| g.inwards_direction).collect(),
            gate_positions: board.gates.iter().map(|g| g.pos).collect(),
            gate_destinations: board
                .gates
                .iter()
                .enumerate()
                .map(|(id, _)| board.get_gate_destination(id))
                .collect(),
            gate_lables: board
                .gates
                .iter()
                .enumerate()
                .map(|(id, _)| board.get_gate_label(id))
                .collect(),
            gate_pos_to_id: board
                .gates
                .iter()
                .enumerate()
                .map(|(id, g)| (g.pos, id as isize))
                .collect(),
            width: board.get_width(),
            height: board.get_height(),
            analysis: analysis,
            board,
        }
    }

    pub fn new_lobby(
        serialized: String,
        gate_metadata: HashMap<u8, GateMetadata>,
        mut sign_text: Vec<(String, isize, isize)>,
        entrance_direction: Option<(usize, Direction)>,
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

        let mut board = Board {
            map: Matrix(map.clone()),
            gates,
        };

        if let Some((entrance_gate, entrance_direction)) = entrance_direction {
            while board.gates[entrance_gate].inwards_direction != entrance_direction {
                board = board.rotate_left()
            }
        }

        Self::new(board, None)
    }

    pub fn rotate_left(&self) -> Self {
        Self::new(self.board.rotate_left(), self.analysis.clone())
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
