use crate::shared::{Solution, Instance, Fitness};
use crate::algorithms::LocalSearch;
use crate::eval::Evaluation;
use crate::neighborhood::Neighborhood;

use super::NeighborFn;

use rand::prelude::*;
use rand::rngs::StdRng;

#[derive(Clone)]
pub struct TwoOpt {
    rand: StdRng,
    buffer: Solution,
}

impl TwoOpt {
    pub fn new(instance: &Instance) -> Self {
        TwoOpt {
            rand: StdRng::from_os_rng(),
            buffer: vec![0; instance.size()],
        }
    }
}

impl NeighborFn for TwoOpt {
    fn get_neighbor(&mut self, solution: &Solution) -> &Solution {
        // Implementation of the 2-opt neighbor generation

        let len = solution.len();
        let mut i = self.rand.random_range(0..len);
        let mut j;

        if i == len - 1 {
            j = self.rand.random_range(0..len - 1);
            (i, j) = (j, i);
        } else {
            j = self.rand.random_range(i + 1..len);
        }

        if self.buffer.len() != len {
            self.buffer.resize(len, 0);
        }

        self.buffer.clone_from_slice(&solution[..]);
        self.buffer[i..=j].reverse();
        &self.buffer
    }
}

impl<Eval: Evaluation> LocalSearch<Eval> for TwoOpt {
    fn search(
        &mut self,
        solution: &mut Solution,
        fitness: &mut Fitness,
        instance: &Instance,
        evaluation: &Eval,
    ) {
        let neighbor = self.get_neighbor(solution);
        let neighbor_fitness = evaluation.score(instance, neighbor);
        
        if neighbor_fitness < *fitness {
            solution.clone_from_slice(neighbor);
            *fitness = neighbor_fitness;
        }
    }

    fn reset(&mut self) {
        // Nothing to reset for TwoOpt
    }

    fn change_neighborhood(&mut self, _neighborhood: Neighborhood) {
        // TwoOpt doesn't support changing neighborhood
    }
}
