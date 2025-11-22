use super::Metaheuristic;
use crate::neighbourhood::{self, NeighborFn, Swap, TwoOpt};
use crate::problem::evaluation::{Evaluation, Fitness, Fitnesses};
use crate::problem::{Instance, Population, Solution};

use rand::Rng;

pub struct SimulatedAnnealing {
    initial_temperature: f64,
    cooling_rate: f64,
    stopping_temperature: f64,
    rng: rand::rngs::ThreadRng,
}

impl SimulatedAnnealing {
    fn acceptance_probability(
        &self,
        current_fitness: f64,
        neighbor_fitness: f64,
        temperature: f64,
    ) -> f64 {
        if neighbor_fitness < current_fitness {
            1.0
        } else {
            ((current_fitness - neighbor_fitness) / temperature).exp()
        }
    }
    pub fn new(initial_temperature: f64, cooling_rate: f64, stopping_temperature: f64) -> Self {
        SimulatedAnnealing {
            initial_temperature,
            cooling_rate,
            stopping_temperature,
            rng: rand::rngs::ThreadRng::default(),
        }
    }
}

impl SimulatedAnnealing {
    fn single_step<E: Evaluation, N: NeighborFn>(
        &mut self,
        solution: &mut Solution,
        fitness: &mut Fitness,
        instance: &Instance,
        neighbourhood: &mut N,
        evaluation: &E,
    ) -> () {
        let neighbor = neighbourhood.get_neighbor(&solution);
        let neighbor_fitness = evaluation.score(instance, &neighbor);
        let rand = self.rng.random_range(0.0..1.0);
        let p: f64 =
            self.acceptance_probability(*fitness, neighbor_fitness, self.initial_temperature);
        if rand < p {
            *solution = neighbor;
            *fitness = neighbor_fitness;
        }
        self.initial_temperature *= self.cooling_rate;
    }
}

impl Metaheuristic for SimulatedAnnealing {
    fn step<Eval: Evaluation, N: NeighborFn>(
        &mut self,
        population: &mut Population,
        fitness: &mut Fitnesses,
        neighbourhood: &mut N,
        instance: &Instance,
        evaluation: &Eval,
    ) {
        for i in 0..population.len() {
            self.single_step(
                &mut population[i],
                &mut fitness[i],
                instance,
                neighbourhood,
                evaluation,
            );
        }
    }
}
