use crate::api::main::Pos;

#[derive(Debug, Clone)]
pub struct Visitations(Vec<u128>, isize);

impl Visitations {
    pub fn new(width: isize, height: isize) -> Self {
        Visitations(vec![0; ((height * width) / 128 +1 ) as usize], width)
    }

    pub fn contains(&self, p: &Pos) -> bool {
        let index = p.x + p.y * self.1;

        (self.0[(index /128) as usize] >> (index % 128) ) & 0x1 == 0x1
    }

    pub fn insert(&mut self, p: &Pos) {
                let index = p.x + p.y * self.1;

        self.0[(index /128) as usize] |= 0x1 << (index % 128);
    }
}