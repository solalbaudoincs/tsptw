use super::Metaheuristic;

use crate::eval::Evaluation;
use crate::shared::{Fitness, Instance, Solution};
use crate::neighborhood::{NeighborFn};

use rand::Rng;
use rand::rngs::StdRng;
use rand::SeedableRng;

/// Algorithme de Hill Climbing pour le TSPTW, utilisant des voisins générés par des opérations de swap et de 2-opt.
pub struct HillClimbing {
    nb_neighbors: usize,
    neighbor_buffer: Solution,
    rng: rand::rngs::StdRng,
}

impl HillClimbing {
    pub fn new(nb_neighbors: usize, instance: &Instance) -> Self {

        let neighbor_buffer = vec![0; instance.size()];
        HillClimbing {
            nb_neighbors,
            neighbor_buffer,
            rng: StdRng::from_os_rng(),
        }
    }
}

impl HillClimbing {
    fn single_step<E: Evaluation, Neighborhood: NeighborFn>(
        &mut self,
        solution: &mut Solution,
        fitness: &mut Fitness,
        neighborhood: &mut Neighborhood,
        instance: &Instance,
        evaluation: &E,
    ) -> () {

        for _ in 0..self.nb_neighbors {
            neighborhood.get_neighbor(solution, &mut self.neighbor_buffer);

            let neighbor_fitness = evaluation.score(instance, &self.neighbor_buffer);
            if neighbor_fitness < *fitness {
                solution.clone_from_slice(&self.neighbor_buffer[..]);
                *fitness = neighbor_fitness;
            }
        }
    }
}

impl Metaheuristic for HillClimbing {
    fn step<Eval: Evaluation, Neighborhood: NeighborFn>(
        &mut self,
        population: &mut [Solution],
        fitness: &mut [Fitness],
        neighborhood: &mut Neighborhood,
        instance: &Instance,
        evaluation: &Eval,
    ) {
        for i in 0..population.len() {
            self.single_step(
                &mut population[i],
                &mut fitness[i],
                neighborhood,
                instance,
                evaluation,
            );
        }
    }

    fn get_metrics(&self) -> std::collections::HashMap<String, f32> {
        std::collections::HashMap::new()
    }
    fn get_metric_names(&self) -> Vec<String> {
        vec![]
    }
}
