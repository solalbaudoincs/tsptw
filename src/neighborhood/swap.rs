use super::NeighborFn;
use crate::shared::{Solution, Instance, Fitness};
use crate::algorithms::LocalSearch;
use crate::eval::Evaluation;
use crate::neighborhood::Neighborhood;

use rand::SeedableRng;
use rand::prelude::*;
use rand::rngs::StdRng;

#[derive(Clone)]
pub struct Swap {
    rand: StdRng,
    buffer: Solution,
}

impl Swap {
    pub fn new(instance: &Instance) -> Self {
        Swap {
            rand: StdRng::from_os_rng(),
            buffer: vec![0; instance.size()],
        }
    }

    pub fn new_with_seed(instance: &Instance, seed: u64) -> Self {
        Swap {
            rand: StdRng::seed_from_u64(seed),
            buffer: vec![0; instance.size()],
        }
    }
}

impl NeighborFn for Swap {
    fn get_neighbor(&mut self, solution: &Solution) -> &Solution {
        // Implementation of the swap neighbor generation
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
        self.buffer.swap(i, j);
        &self.buffer
    }
}

impl<Eval: Evaluation> LocalSearch<Eval> for Swap {
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
        // Nothing to reset for Swap
    }

    fn change_neighborhood(&mut self, _neighborhood: Neighborhood) {
        // Swap doesn't support changing neighborhood
    }
}
