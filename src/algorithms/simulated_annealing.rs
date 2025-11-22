use crate::shared_types::{Fitness, Instance, Population, Solution, Fitnesses};

use super::Metaheuristic;
use crate::problem::{
    evaluation::Evaluation,
};
use crate::neighbourhood::{Swap, TwoOpt};


use rand::Rng;

pub struct SimulatedAnnealing {
    initial_temperature: f64,
    cooling_rate: f64,
    stopping_temperature: f64,
    iters_per_temp: usize,
    two_opt_rate: f64,
}

impl SimulatedAnnealing {
    fn acceptance_probability(&self, current_fitness: f64, neighbor_fitness: f64, temperature: f64) -> f64 {
        if neighbor_fitness < current_fitness {
            1.0
        } else {
            ((current_fitness - neighbor_fitness) / temperature).exp()
        }
    }
}

impl SimulatedAnnealing {
    fn single_step<E: Evaluation>(
        &mut self,
        solution: &mut Solution,
        fitness: &mut Fitness,
        instance: &Instance,
        evaluation: &E
    ) -> () {

        let mut rng = rand::rng();

        for _ in 0..self.iters_per_temp {

            let neighbor;
            if rng.random_range(0.0..1.0) < self.two_opt_rate {
                neighbor = TwoOpt.get_neighbor(&solution);
            } else {
                neighbor = Swap.get_neighbor(&solution);
            }
            let neighbor_fitness = evaluation.score(instance, &neighbor);
            let rand = rng.random_range(0.0..1.0);
            if rand < self.acceptance_probability(*fitness, neighbor_fitness, self.initial_temperature) {
                *solution = neighbor;
                *fitness = neighbor_fitness;
            }
        } self.initial_temperature *= self.cooling_rate;
    }
}

impl Metaheuristic for SimulatedAnnealing {
    fn step<Eval: Evaluation>(
        &mut self,
        population: &mut Population,
        fitness: &mut Fitnesses,
        instance: &Instance,
        evaluation: &Eval,
    ) {
        for i in 0..population.len() {
            self.single_step(
                &mut population[i],
                &mut fitness[i],
                instance,
                evaluation
            );
        }
    }
}