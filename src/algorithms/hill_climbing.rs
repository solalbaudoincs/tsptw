use super::Metaheuristic;

use crate::eval::Evaluation;
use crate::shared::{Fitness, Instance, Solution};
use crate::neighborhood::{Swap, TwoOpt, NeighborFn};

use rand::Rng;
use rand::rngs::StdRng;
use rand::SeedableRng;

pub struct HillClimbing {
    nb_neighbors: usize,
    neighbor_buffer: Solution,
    two_opt_rate: f32,
    rng: rand::rngs::StdRng,
}

impl HillClimbing {
    pub fn new(nb_neighbors: usize, two_opt_rate: f32, instance: &Instance) -> Self {

        let neighbor_buffer = vec![0; instance.size()];
        HillClimbing {
            nb_neighbors,
            neighbor_buffer,
            two_opt_rate,
            rng: StdRng::from_os_rng(),
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

        for _ in 0..self.nb_neighbors {
            let r = self.rng.random_range(0.0..1.0);

            if r < self.two_opt_rate {
                TwoOpt::new().get_neighbor(&solution, &mut self.neighbor_buffer[..])
            } else {
                Swap::new().get_neighbor(&solution, &mut self.neighbor_buffer[..])
            };

            let neighbor_fitness = evaluation.score(instance, &self.neighbor_buffer);
            if neighbor_fitness < *fitness {
                solution.clone_from_slice(&self.neighbor_buffer[..]);
                *fitness = neighbor_fitness;
            }
        }
    }
}

impl Metaheuristic for HillClimbing {
    fn step<Eval: Evaluation>(
        &mut self,
        population: &mut [Solution],
        fitness: &mut [Fitness],
        instance: &Instance,
        evaluation: &Eval,
    ) {
        for i in 0..population.len() {
            self.single_step(
                &mut population[i],
                &mut fitness[i],
                instance,
                evaluation,
            );
        }
    }
}
