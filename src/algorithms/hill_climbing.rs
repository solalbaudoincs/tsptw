use super::LocalSearch;
use super::Metaheuristic;

use crate::eval::Evaluation;
use crate::neighborhood::{NeighborFn, Neighborhood};
use crate::shared::{Fitness, Instance, Solution};

/// Algorithme de Hill Climbing pour le TSPTW, utilisant des voisins générés par des opérations de swap et de 2-opt.
#[derive(Clone)]
pub struct HillClimbing {
    step: usize,
    max_steps: usize,
    neighborhood: Neighborhood,
    // rng: StdRng,
    iteration: usize,
}

impl HillClimbing {
    pub fn new(step: usize, max_steps: usize, neighborhood: Neighborhood) -> Self {
        Self {
            step,
            max_steps,
            neighborhood,
            iteration: 0,
        }
    }
}

impl HillClimbing {
    fn single_step<E: Evaluation>(
        &mut self,
        solution: &mut Solution,
        fitness: &mut Fitness,
        instance: &Instance,
        evaluation: &E,
    ) -> () {
        for _ in 0..self.step {
            let neighbor_buffer = self.neighborhood.get_neighbor(solution);

            let neighbor_fitness = evaluation.score(instance, neighbor_buffer);
            if neighbor_fitness < *fitness {
                solution.clone_from_slice(neighbor_buffer);
                *fitness = neighbor_fitness;
            }
        }
    }
    fn _stop_condition_met(&self) -> bool {
        self.iteration >= self.max_steps
    }
    fn _reset(&mut self) {
        self.iteration = 0;
    }
}

impl<Eval: Evaluation> Metaheuristic<Eval> for HillClimbing {
    fn step(
        &mut self,
        population: &mut [Solution],
        fitness: &mut [Fitness],
        instance: &Instance,
        evaluation: &Eval,
    ) {
        for i in 0..population.len() {
            self.single_step(&mut population[i], &mut fitness[i], instance, evaluation);
        }
        self.iteration += 1;
    }

    fn get_metrics(&self) -> std::collections::HashMap<String, f32> {
        std::collections::HashMap::new()
    }
    fn get_metric_names(&self) -> Vec<String> {
        vec![]
    }

    fn stop_condition_met(&self) -> bool {
        self._stop_condition_met()
    }
    fn get_iteration(&self) -> usize {
        self.iteration
    }
}

impl<Eval: Evaluation> LocalSearch<Eval> for HillClimbing {
    fn search(
        &mut self,
        solution: &mut Solution,
        fitness: &mut Fitness,
        instance: &Instance,
        evaluation: &Eval,
    ) {
        while !self._stop_condition_met() {
            self.single_step(solution, fitness, instance, evaluation);
        }
    }

    fn reset(&mut self) {
        self._reset();
    }

    fn change_neighborhood(&mut self, neighborhood: Neighborhood) {
        self._reset();
        self.neighborhood = neighborhood;
    }
}
