use rand::Rng;

use crate::shared_types::{Fitness, Instance, Solution, Population, Fitnesses};
use crate::neighbourhood::{Swap, TwoOpt, NeighborFn};
use crate::shared_types::Evaluation;
use super::Metaheuristic;

pub struct HillClimbing {
    nb_neighboors: usize,
    two_opt_rate: f64,
}

impl  HillClimbing {
    fn single_step<E: Evaluation, N: NeighborFn>(
        &self,
        solution: &mut Solution,
        fitness: &mut Fitness,
        instance: &Instance,
        neighbourhood: &N,
        evaluation: &E
    ) -> () {

        for _ in 0..self.nb_neighboors {

            let neighbor = neighbourhood.get_neighbor(&solution);
            let neighbor_fitness = evaluation.score(&instance, &neighbor);
            if neighbor_fitness < *fitness {
                *solution = neighbor;
                *fitness = neighbor_fitness;
            }
        }
    }
}

impl Metaheuristic for HillClimbing {
    fn step<Eval: Evaluation, N: NeighborFn>(
        &mut self,
        population: &mut Population,
        fitness: &mut Fitnesses,
        neighbourhood: &N,
        instance: &Instance,
        evaluation: &Eval,
    ) {
        for i in 0..population.len() {
            self.single_step(
                &mut population[i],
                &mut fitness[i],
                instance,
                neighbourhood,
                evaluation
            );
        }
    }
}