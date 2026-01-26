use super::{NeighborFn, Swap, TwoOpt};
use crate::shared::{Solution, Instance, Fitness};
use crate::algorithms::LocalSearch;
use crate::eval::Evaluation;
use crate::neighborhood::Neighborhood;

#[derive(Clone)]
pub struct Alternating {
    swap: Swap,
    twoopt: TwoOpt,
    use_swap: bool,
}

impl Alternating {
    pub fn new(instance: &Instance) -> Self {
        Alternating {
            swap: Swap::new(instance),
            twoopt: TwoOpt::new(instance),
            use_swap: true,
        }
    }

    pub fn new_with_seed(instance: &Instance, seed: u64) -> Self {
        Alternating {
            swap: Swap::new_with_seed(instance, seed),
            twoopt: TwoOpt::new_with_seed(instance, seed.wrapping_add(1)),
            use_swap: true,
        }
    }
}

impl NeighborFn for Alternating {
    fn get_neighbor(&mut self, solution: &Solution) -> &Solution {
        let neighbor = if self.use_swap {
            self.swap.get_neighbor(solution)
        } else {
            self.twoopt.get_neighbor(solution)
        };
        self.use_swap = !self.use_swap;  // Toggle for next call
        neighbor
    }
}

impl<Eval: Evaluation> LocalSearch<Eval> for Alternating {
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
        // Nothing to reset for Alternating
    }

    fn change_neighborhood(&mut self, _neighborhood: Neighborhood) {
        // Alternating manages its own neighborhoods
    }
}
