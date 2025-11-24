use super::Metaheuristic;

use crate::neighborhood::{NeighborFn};
use crate::eval::Evaluation;
use crate::shared::{Fitness, Instance, Solution};

pub struct HillClimbing {
    nb_neighbors: usize,
    neighbor_buffer: Solution,
}

impl HillClimbing {
    pub fn new(nb_neighbors: usize, instance: &Instance) -> Self {

        let neighbor_buffer = vec![0; instance.size()];
        HillClimbing {
            nb_neighbors,
            neighbor_buffer,
        }
    }
}

impl HillClimbing {
    fn single_step<E: Evaluation, N: NeighborFn>(
        &mut self,
        solution: &mut Solution,
        fitness: &mut Fitness,
        instance: &Instance,
        neighborhood: &mut N,
        evaluation: &E,
    ) -> () {

        for _ in 0..self.nb_neighbors {

            // Generate a neighbor and puts it in the neighbor_buffer
            neighborhood.get_neighbor(&solution, &mut self.neighbor_buffer);

            let neighbor_fitness = evaluation.score(&instance, &self.neighbor_buffer);
            if neighbor_fitness < *fitness {
                solution.clone_from_slice(&self.neighbor_buffer[..]);
                *fitness = neighbor_fitness;
            }
        }
    }
}

impl Metaheuristic for HillClimbing {
    fn step<Eval: Evaluation, N: NeighborFn>(
        &mut self,
        population: &mut [Solution],
        fitness: &mut [Fitness],
        neighborhood: &mut N,
        instance: &Instance,
        evaluation: &Eval,
    ) {
        for i in 0..population.len() {
            self.single_step(
                &mut population[i],
                &mut fitness[i],
                instance,
                neighborhood,
                evaluation,
            );
        }
    }
}
