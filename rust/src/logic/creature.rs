use crate::{
    api::main::Board,
    logic::solver::{solve, Analysis},
};

#[derive(Clone)]
pub struct Creature {
    pub board: Board,
    pub analisis: Analysis,
    pub fitness: f32,
    pub mutation_count: usize,
}

impl Creature {
    pub fn board_to_creature(b: Option<Board>) -> Option<Self> {
        let b = b?;
        let analysis = solve(&b);
        if let Some(analysis) = analysis {
            Some(Self {
                board: b,
                fitness: analysis.fitness(),
                analisis: analysis,
                mutation_count: 0,
            })
        } else {
            None
        }
    }

    pub fn mutate(&self, factor: f32) -> Option<Self> {
        Self::board_to_creature(self.board.mutate(factor)).map(|mut ret| {
            ret.mutation_count = self.mutation_count + 1;
            ret
        })
    }
}

impl PartialOrd for Creature {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.fitness.partial_cmp(&other.fitness)
    }
}

impl PartialEq for Creature {
    fn eq(&self, other: &Self) -> bool {
        self.fitness == other.fitness
    }
}
