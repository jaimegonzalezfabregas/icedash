use std::ops::{Add, Div, Mul, Sub};


#[derive(Clone, Debug, Eq, PartialEq, Hash, Copy)]
pub struct Pos {
    pub x: isize,
    pub y: isize,
}

impl Pos {
    pub fn parity(&self) -> bool{
        (self.x + self.y) % 2 == 0
    }

    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    pub(crate) fn rotate_left(self, width: isize) -> Pos {
        Self {
            x: self.y,
            y: -self.x + width - 1,
        }
    }
}

impl Add<Pos> for Pos {
    type Output = Self;

    fn add(self, rhs: Pos) -> Self::Output {
        Pos::new(self.x + rhs.x, self.y + rhs.y)
    }
}


impl Sub<Pos> for Pos {
    type Output = Self;

    fn sub(mut self, rhs: Pos) -> Self::Output {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self
    }
}

impl Div<isize> for Pos {
    type Output = Pos;

    fn div(self, rhs: isize) -> Self::Output {
        Pos::new(self.x / rhs, self.y / rhs)
    }
}

impl Mul<isize> for Pos {
    type Output = Pos;

    fn mul(self, rhs: isize) -> Self::Output {
        Pos::new(self.x * rhs, self.y * rhs)
    }
}