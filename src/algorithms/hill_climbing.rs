use rand::Rng;

use crate::shared_types::{Fitness, Instance, Solution, Population, Fitnesses};
use crate::neighbourhood::{Swap, TwoOpt};
use crate::shared_types::Evaluation;
use super::Metaheuristic;

pub struct HillClimbing {
    nb_neighboors: usize,
    two_opt_rate: f32,
}

impl  HillClimbing {
    fn single_step<E: Evaluation>(
        &self,
        solution: &mut Solution,
        fitness: &mut Fitness,
        instance: &Instance,
        evaluation: &E
    ) -> () {

        let mut rng = rand::rng();

        for _ in 0..self.nb_neighboors {

            let neighbor;
            if rng.random_range(0.0..1.0) < self.two_opt_rate {
                neighbor = TwoOpt.get_neighbor(&solution);
            } else {
                neighbor = Swap.get_neighbor(&solution);
            }
            let neighbor_fitness = evaluation.score(&instance, &neighbor);
            if neighbor_fitness < *fitness {
                *solution = neighbor;
                *fitness = neighbor_fitness;
            }
        }
    }
}

impl Metaheuristic for HillClimbing {
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