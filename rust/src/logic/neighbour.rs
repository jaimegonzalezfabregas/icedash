use crate::api::main::Tile;


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

    pub fn to_stops_player_during_gameplay(&self) -> Neighbour<bool> {
        Neighbour {
            center: self.center.stops_player_during_gameplay(),
            north: self.north.stops_player_during_gameplay(),
            south: self.south.stops_player_during_gameplay(),
            east: self.east.stops_player_during_gameplay(),
            west: self.west.stops_player_during_gameplay(),
            northwest: self.northwest.stops_player_during_gameplay(),
            northeast: self.northeast.stops_player_during_gameplay(),
            southwest: self.southwest.stops_player_during_gameplay(),
            southeast: self.southeast.stops_player_during_gameplay(),
        }
    }

    pub fn get_asset(&self) -> Option<(String, isize)> {
        match self.center {
            Tile::Gate(Some((_, _))) | Tile::Ice | Tile::WeakWall | Tile::Box => {
                Some(("ice.png".into(), 0))
            }
            Tile::Outside => None,
            Tile::Stop => Some(("stop.png".into(), 0)),
            Tile::Wall | Tile::Gate(None) => {
                let mut rotator = self.clone();
                let mut ret = None;
                let mut ret_priority = 0;

                for i in 0..4 {
                    let (priority, new_ret) = match rotator.to_stops_player_during_gameplay() {
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
                        } => (100, Some(("wall.png".into(), i))),

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