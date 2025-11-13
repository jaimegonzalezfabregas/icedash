use rand::seq::IteratorRandom;

use crate::api::{main::GateMetadata, tile::Tile};

#[derive(Clone)]
pub struct Neighbour<T> {
    pub center: T,
    pub north: T,
    pub south: T,
    pub east: T,
    pub west: T,
    pub northwest: T,
    pub northeast: T,
    pub southwest: T,
    pub southeast: T,
}

impl Neighbour<Tile> {
    pub fn mask_center(&self, tile: Tile) -> Self {
        let mut ret = self.clone();
        ret.center = tile;
        ret
    }

    pub fn to_is_a_wall_for_texturing(&self) -> Neighbour<bool> {
        Neighbour {
            center: self.center.is_a_wall_for_texturing(),
            north: self.north.is_a_wall_for_texturing(),
            south: self.south.is_a_wall_for_texturing(),
            east: self.east.is_a_wall_for_texturing(),
            west: self.west.is_a_wall_for_texturing(),
            northwest: self.northwest.is_a_wall_for_texturing(),
            northeast: self.northeast.is_a_wall_for_texturing(),
            southwest: self.southwest.is_a_wall_for_texturing(),
            southeast: self.southeast.is_a_wall_for_texturing(),
        }
    }

    pub fn get_asset(&self) -> Option<(String, isize)> {
        let mut rng = rand::rng();
        rng.reseed().expect("random initialization failed");

        match self.center {
            Tile::Gate(GateMetadata::Exit { .. })
            | Tile::Ice
            | Tile::WeakWall
            | Tile::Sign { .. }
            | Tile::Box => Some(("ice.png".into(), 0)),
            Tile::Outside => None,
            Tile::Stop => Some(("stop.png".into(), 0)),
            Tile::Wall | Tile::Gate(_) => {
                let mut rotator = self.clone();
                let mut ret = None;
                let mut ret_priority = 0;

                for i in 0..4 {
                    let (priority, new_ret) = match rotator.to_is_a_wall_for_texturing() {
                        Neighbour {
                            southwest: true,
                            northwest: false,
                            southeast: false,
                            northeast: true,
                            south: true,
                            west: true,
                            north: true,
                            east: true,
                            ..
                        } => (500, Some(("wall_double_corner.png".into(), i))),

                        Neighbour {
                            southwest: false,
                            south: true,
                            west: true,
                            north: true,
                            east: true,
                            ..
                        } => (300, Some(("wall_corner_in.png".into(), i))),
                        Neighbour {
                            south: false,
                            west: false,
                            north: true,
                            east: true,
                            northeast: true,
                            ..
                        } => (200, Some(("wall_corner_out.png".into(), i))),
                        Neighbour {
                            south: false,
                            north: true,
                            ..
                        } => (
                            100,
                            Some((
                                (format!("wall/{}.png", (1..=8).choose(&mut rng).unwrap())).into(),
                                i,
                            )),
                        ),

                        _ => (0, None),
                    };
                    if priority > ret_priority {
                        ret_priority = priority;
                        ret = new_ret;
                    }

                    rotator = rotator.rotate_left();
                }

                return ret;
            }
        }
    }
}

impl<T: Clone> Neighbour<T> {
    fn rotate_left(&self) -> Neighbour<T> {
        Neighbour {
            center: self.center.clone(),
            north: self.east.clone(),
            south: self.west.clone(),
            east: self.south.clone(),
            west: self.north.clone(),
            northeast: self.southeast.clone(),
            southeast: self.southwest.clone(),
            northwest: self.northeast.clone(),
            southwest: self.northwest.clone(),
        }
    }
}
