use flutter_rust_bridge::frb;

use crate::{
    api::main::{Direction, LeftRotatable, Tile},
    logic::{ neighbour::Neighbour, pos::Pos},
};

pub(crate) type TileMap = Matrix<Tile>;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[frb(opaque)]
pub struct Matrix<T: Clone>(pub Vec<Vec<T>>);

impl<T: Clone + LeftRotatable + Default> LeftRotatable for Matrix<T> {
    fn rotate_left(&self) -> Matrix<T> {
        let mut ret = Matrix(vec![
            vec![T::default(); self.get_height() as usize];
            self.get_width() as usize
        ]);

        for p in self.all_pos() {
            ret.set(&p.rotate_left(self.get_width()), self.at(&p).rotate_left());
        }

        // ret.print(vec![]);

        ret
    }
}

impl<T: Clone> Matrix<T> {

    pub fn rotate_left_keeping_elements(&self) -> Matrix<T>
    where
        T: Default,
    {
        let mut ret = Matrix(vec![
            vec![T::default(); self.get_height() as usize];
            self.get_width() as usize
        ]);

        for p in self.all_pos() {
            ret.set(&p.rotate_left(self.get_width()), self.at(&p));
        }

        ret
    }

    pub fn at(&self, p: &Pos) -> T
    where
        T: Default,
    {
        if self.in_bounds(p) {
            self.0[p.y as usize][p.x as usize].clone()
        } else {
            T::default()
        }
    }


    pub fn new(width: isize, height: isize) -> Matrix<T>
    where
        T: Default,
    {
        Matrix(vec![vec![T::default(); width as usize]; height as usize])
    }

    pub fn neighbour_at(&self, p: &Pos) -> Neighbour<T>
    where
        T: Default,
    {
        use Direction::*;
        Neighbour {
            center: self.at(p),
            north: self.at(&(*p + North.vector())),
            south: self.at(&(*p + South.vector())),
            east: self.at(&(*p + East.vector())),
            west: self.at(&(*p + West.vector())),
            northwest: self.at(&(*p + North.vector() + West.vector())),
            northeast: self.at(&(*p + North.vector() + East.vector())),
            southwest: self.at(&(*p + South.vector() + West.vector())),
            southeast: self.at(&(*p + South.vector() + East.vector())),
        }
    }

    pub fn atxy(&self, x: isize, y: isize) -> T {
        self.0
            .get(y as usize)
            .unwrap()
            .get(x as usize)
            .unwrap()
            .clone()
    }

    pub fn set(&mut self, p: &Pos, val: T) {
        self.0[p.y as usize][p.x as usize] = val;
    }

    pub fn get_width(&self) -> isize {
        self.0[0].len() as isize
    }

    pub fn get_height(&self) -> isize {
        self.0.len() as isize
    }

    pub fn all_inner_pos<'a>(&'a self) -> impl Iterator<Item = Pos> + use<'a, T> {
        (1..self.get_width() - 1).into_iter().flat_map(|x| {
            (1..self.get_height() - 1)
                .into_iter()
                .map(move |y| Pos::new(x as isize, y as isize))
        })
    }

    pub fn all_pos<'a>(&'a self) -> impl Iterator<Item = Pos> + use<'a, T> {
        (0..self.get_width()).into_iter().flat_map(|x| {
            (0..self.get_height())
                .into_iter()
                .map(move |y| Pos::new(x as isize, y as isize))
        })
    }

    pub fn in_bounds(&self, p: &Pos) -> bool {
        p.x >= 0 && p.y >= 0 && p.x < self.get_width() && p.y < self.get_height()
    }

    pub fn map<B: Clone>(self, f: fn(T) -> B) -> Matrix<B> {
        Matrix(
            self.0
                .iter()
                .cloned()
                .map(|e| e.iter().cloned().map(|e| f(e)).collect())
                .collect(),
        )
    }
}

impl TileMap {
    pub fn print(&self, highlight: Vec<Pos>) {
        for (y, row) in self.0.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                if highlight.contains(&Pos::new(x as isize, y as isize)) {
                    print!(". ");
                } else {
                    print!("{} ", tile.symbol());
                }
            }
            println!("");
        }
    }
}
