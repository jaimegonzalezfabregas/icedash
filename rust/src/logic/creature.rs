use crate::logic::{board::Board, solver::{solve, Analysis}};

#[derive(Clone)]
pub struct Creature {
    pub board: Board,
    pub fitness: f32,
    pub analysis: Vec<Analysis>,
    pub mutation_count: usize,
}

impl Creature {
    pub fn board_to_creature(b: Option<Board>) -> Option<Self> {
        let b = b?;

        let analysis = solve(&b);
        if analysis.len() != 0 {
            let mut fitness = analysis[0].fitness();

            for analysis in analysis.iter() {
                fitness = fitness.min(analysis.fitness())
            }

            fitness /= analysis.len().pow(2) as f32;

            Some(Self {
                board: b,
                fitness,
                analysis,
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
