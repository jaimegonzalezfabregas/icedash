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
    pub size_range_min: isize,
    pub size_range_max: isize,
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
            size_range_min: data[0],
            size_range_max: data[1],
            weak_walls_percentage_min: data[2],
            weak_walls_percentage_max: data[3],
            pilars_percentage_min: data[4],
            pilars_percentage_max: data[5],
            box_percentage_min: data[6],
            box_percentage_max: data[7],
            vignet_percentage_min: data[8],
            vignet_percentage_max: data[9],
            game_mode: data[10].into(),
        }
    }

    pub fn as_list(&self) -> Vec<isize> {
        vec![
            self.size_range_min,
            self.size_range_max,
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
