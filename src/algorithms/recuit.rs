use crate::shared_types::{Fitness, Instance, Solution};
use crate::neighbor_fn::{NeighborFn, Swap, TwoOpt};


use super::Metaheuristic;
use crate::neighbourhood::NeighbourhoodGenerator;
use crate::problem::{
    evaluation::Evaluation,
    instance::Instance,
    solution::{Population, Solution},
};


use rand::Rng;

struct Recuit {
    initial_temperature: f32,
    cooling_rate: f32,
    stopping_temperature: f32,
    iters_per_temp: usize,
    two_opt_rate: f32,
}

impl Recuit {
    fn acceptance_probability(&self, current_fitness: f32, neighbor_fitness: f32, temperature: f32) -> f32 {
        if neighbor_fitness < current_fitness {
            1.0
        } else {
            ((current_fitness - neighbor_fitness) / temperature).exp()
        }
    }
}

impl MetaHeuristic<Solution, Fitness> for Recuit {
    fn step<F: metric::Metric>(
        &mut self,
        solution: &mut Vec<Solution>,
        fitness: &mut Vec<Fitness>,
        instance: &Instance,
        metric: &F
    ) -> () {

        let mut rng = rand::rng();

        for _ in 0..self.iters_per_temp {

            let neighbor;
            if rng.random_range(0.0..1.0) < self.two_opt_rate {
                neighbor = TwoOpt.get_neighbor(&solution);
            } else {
                neighbor = Swap.get_neighbor(&solution);
            }
            let neighbor_fitness = metric.evaluate(&neighbor, instance);
            let rand = rng.random_range(0.0..1.0);
            if rand < self.acceptance_probability(*fitness, neighbor_fitness, self.initial_temperature) {
                *solution = neighbor;
                *fitness = neighbor_fitness;
            }
        } self.initial_temperature *= self.cooling_rate;
    }
}