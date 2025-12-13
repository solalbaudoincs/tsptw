use super::LocalSearch;
use super::Metaheuristic;
use crate::eval::Evaluation;
use crate::neighborhood::Neighborhood; // Use Enum
use crate::shared::{Fitness, Instance, Solution};

pub struct VNS<LS> {
    neighborhoods: Vec<Neighborhood>, // Enum wrap
    local_search: LS,
    indexes: Vec<usize>,
    iteration: usize,
}

impl<LS> VNS<LS> {
    pub fn new(
        neighborhoods: Vec<Neighborhood>, // Enum wrap
        local_search: LS,
    ) -> Self {
        let neighborhoods_len = neighborhoods.len();
        VNS {
            neighborhoods,
            local_search,
            indexes: (0..neighborhoods_len).collect(),
            iteration: 0,
        }
    }
}

impl<Eval: Evaluation,  LS: LocalSearch<Eval>> Metaheuristic<Eval> for VNS<LS> {
    fn step(
        &mut self,
        population: &mut [Solution],
        fitness: &mut [Fitness],
        instance: &Instance,
        evaluation: &Eval,
    ) {
        for i in 0..population.len() {
            let mut k = 0;
            while k < self.neighborhoods.len() {
                let start_fitness = fitness[i];
                let index = self.indexes[k];
                // Clone neighborhood into SA
                let n = self.neighborhoods[index].clone();
                self.local_search.change_neighborhood(n);

                // Run SA
                self.local_search.reset();
                self.local_search
                    .search(&mut population[i], &mut fitness[i], instance, evaluation);

                // Check improvement
                if fitness[i] < start_fitness {
                    // Improved: restart from first neighborhood (VND strategy)
                    k = 0;
                } else {
                    // Not improved: move to next neighborhood
                    k += 1;
                }
            }
        } self.iteration += 1;
    }
    fn stop_condition_met(&self) -> bool {
        // VNS step runs completely. We rely on calling code or SA metrics.
        // Since step runs the whole VNS/VND procedure, we are "done" after one step.
        // Usually Metaheuristic::step is one iteration.
        // But our step runs the WHOLE VNS loop.
        // We could implement shake in the future to have finer control.
        true
    }
    fn get_iteration(&self) -> usize {
        self.iteration
    }
}
