use crate::{api::{pos::Pos, tile::Tile}, logic::matrix::TileMap};

pub struct TileMapWrap<'a> {
    pub base: &'a TileMap,
    pub p: Pos,
    pub tile: Tile,
}

impl<'a> TileMapWrap<'a> {
    pub fn at(&self, p: &Pos) -> Tile {
        if *p == self.p {
            self.tile.clone()
        } else {
            self.base.at(p)
        }
    }
}
