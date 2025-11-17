use crate::{api::{direction::Direction, pos::Pos, tile::Tile}, logic::matrix::{Matrix, TileMap}};

pub type AssetMap = Matrix<Option<(String, isize)>>;

impl Matrix<Option<(String, isize)>> {
   pub fn from_tilemap(tilemap: &TileMap) -> Self {
        let mut ret = Matrix::new(tilemap.get_width(), tilemap.get_height());
        let mut wip_tilemap = tilemap.clone();

        let mut rep = true;

        while rep {
            rep = false;

            // Decorate Interior
            for p in wip_tilemap.all_pos().collect::<Vec<_>>() {
                let tile = wip_tilemap.at(&p);

                if let Tile::Wall = tile {
                    let mut neigh_count = 0;
                    for delta in Direction::all() {
                        if wip_tilemap
                            .at(&(p + delta.vector()))
                            .is_a_wall_for_texturing()
                        {
                            neigh_count += 1;
                        }
                    }
                    if neigh_count < 2 {
                        ret.set(&p, Some(("1x1_obstacle.png".into(), 0)));
                        wip_tilemap.set(&p, Tile::Ice);
                        rep = true;
                    }

                    if !wip_tilemap
                        .at(&(p + Pos::new(1, 0)))
                        .is_a_wall_for_texturing()
                        && !wip_tilemap
                            .at(&(p + Pos::new(-1, 0)))
                            .is_a_wall_for_texturing()
                    {
                        ret.set(&p, Some(("1x1_obstacle.png".into(), 0)));
                        wip_tilemap.set(&p, Tile::Ice);
                        rep = true;
                    }

                    if !wip_tilemap
                        .at(&(p + Pos::new(0, 1)))
                        .is_a_wall_for_texturing()
                        && !wip_tilemap
                            .at(&(p + Pos::new(0, -1)))
                            .is_a_wall_for_texturing()
                    {
                        ret.set(&p, Some(("1x1_obstacle.png".into(), 0)));
                        wip_tilemap.set(&p, Tile::Ice);
                        rep = true;
                    }
                }
            }
        }

        for p in wip_tilemap.all_pos() {
            if let None = ret.at(&p) {
                ret.set(&p, wip_tilemap.neighbour_at(&p).get_asset());
            }
        }

        ret
    }
}
