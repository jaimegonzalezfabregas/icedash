use crate::logic::{
    board::Board,
    solver::{analyze, Analysis},
};

#[derive(Clone)]
pub struct Creature {
    pub board: Board,
    pub fitness: f32,
    pub analysis: Analysis,
    pub mutation_count: usize,
}

impl Creature {
    

    pub fn board_to_creature(b: Board) -> Result<Self, String> {

        let analysis = analyze(&b)?;

        Ok(Self {
            board: b,
            fitness : analysis.compute_fitness(),
            analysis,
            mutation_count: 0,
        })
    }

    pub fn mutate(&self, factor: f32) -> Result<Self, String> {
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
