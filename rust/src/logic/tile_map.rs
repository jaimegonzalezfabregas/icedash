use crate::api::main::{Pos, Tile};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct TileMap(pub Vec<Vec<Tile>>);

impl TileMap {
    pub fn at_mut(&mut self, p: Pos) -> &mut Tile {
        self.0
            .get_mut(p.y as usize)
            .unwrap()
            .get_mut(p.x as usize)
            .unwrap()
    }

    pub fn at(&self, p: Pos) -> Tile {
        if (self.in_bounds(p)) {
            self.0
                .get(p.y as usize)
                .unwrap()
                .get(p.x as usize)
                .unwrap()
                .clone()
        } else {
            Tile::Outside
        }
    }

    pub fn atxy(&self, x: isize, y: isize) -> Tile {
        self.0
            .get(y as usize)
            .unwrap()
            .get(x as usize)
            .unwrap()
            .clone()
    }

    pub fn set(&mut self, p: Pos, val: Tile) {
        *self.at_mut(p) = val;
    }

    pub fn get_width(&self) -> isize {
        self.0[0].len() as isize
    }

    pub fn get_height(&self) -> isize {
        self.0.len() as isize
    }

    pub fn all_inner_pos<'a>(&'a self) -> impl Iterator<Item = Pos> + use<'a> {
        (1..self.get_width() - 1).into_iter().flat_map(|x| {
            (1..self.get_height() - 1)
                .into_iter()
                .map(move |y| Pos::new(x as isize, y as isize))
        })
    }

    pub fn all_pos<'a>(&'a self) -> impl Iterator<Item = Pos> + use<'a> {
        (0..self.get_width()).into_iter().flat_map(|x| {
            (0..self.get_height())
                .into_iter()
                .map(move |y| Pos::new(x as isize, y as isize))
        })
    }

    pub fn print(&self, highlight: Vec<Pos>) {
        for (y, row) in self.0.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                if highlight.contains(&Pos::new(x as isize, y as isize)) {
                    print!(". ");
                } else {
                    print!("{} ", tile.simbol());
                }
            }
            println!("");
        }
    }

    pub fn in_bounds(&self, p: Pos) -> bool {
        p.x >= 0 && p.y >= 0 && p.x < self.get_width() && p.y < self.get_height()
    }

    pub(crate) fn rotate_left(self) -> TileMap {
        let mut ret = TileMap(vec![
            vec![Tile::Wall; self.get_height() as usize];
            self.get_width() as usize
        ]);

        for p in self.all_pos() {
            ret.set(p.rotate_left(self.get_height()), self.at(p));
        }

        // ret.print(vec![]);

        ret
    }
}
