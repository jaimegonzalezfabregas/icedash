#[derive(Clone, PartialEq, Debug, Eq, Hash)]
pub enum GameMode {
    FindExit,
    FindPerfectPath,
}

impl From<isize> for GameMode {
    fn from(value: isize) -> Self {
        if value == 0 {
            Self::FindExit
        } else {
            Self::FindPerfectPath
        }
    }
}

impl From<&GameMode> for isize {
    fn from(value: &GameMode) -> Self {
        if let GameMode::FindExit = value {
            0
        } else {
            1
        }
    }
}

#[derive(Clone, PartialEq, Debug, Eq, Hash)]
pub struct BoardDescription {
    pub area: isize,
    pub weak_walls_percentage_min: isize,
    pub weak_walls_percentage_max: isize,
    pub pilars_percentage_min: isize,
    pub pilars_percentage_max: isize,
    pub box_percentage_min: isize,
    pub box_percentage_max: isize,
    pub vignet_percentage_min: isize,
    pub vignet_percentage_max: isize,
    pub game_mode: GameMode,
}

impl BoardDescription {
    pub fn from_list(data: Vec<isize>) -> BoardDescription {
        BoardDescription {
            area: data[0],
            weak_walls_percentage_min: data[1],
            weak_walls_percentage_max: data[2],
            pilars_percentage_min: data[3],
            pilars_percentage_max: data[4],
            box_percentage_min: data[5],
            box_percentage_max: data[6],
            vignet_percentage_min: data[7],
            vignet_percentage_max: data[8],
            game_mode: data[9].into(),
        }
    }

    pub fn as_list(&self) -> Vec<isize> {
        vec![
            self.area,
            self.weak_walls_percentage_min,
            self.weak_walls_percentage_max,
            self.pilars_percentage_min,
            self.pilars_percentage_max,
            self.box_percentage_min,
            self.box_percentage_max,
            self.vignet_percentage_min,
            self.vignet_percentage_max,
            (&self.game_mode).into(),
        ]
    }
}
