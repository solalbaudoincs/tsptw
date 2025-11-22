use super::Metaheuristic;
use crate::neighbourhood::NeighborFn;
use crate::problem::evaluation::{Evaluation, Fitness, Fitnesses};
use crate::problem::{Instance, Population, Solution};
use crate::initializer::{RandomInitializer, Initializer};

use rand::Rng;

pub struct SimulatedAnnealing {
    initial_temperature: f32,
    cooling_rate: f32,
    stopping_temperature: f32,
    rng: rand::rngs::ThreadRng,
}

impl SimulatedAnnealing {
    fn acceptance_probability(
        &self,
        current_fitness: f32,
        neighbor_fitness: f32,
        temperature: f32,
    ) -> f32 {
        if neighbor_fitness < current_fitness {
            1.0
        } else {
            ((current_fitness - neighbor_fitness) / temperature).exp()
        }
    }
    pub fn new(initial_temperature: f32, cooling_rate: f32, stopping_temperature: f32) -> Self {
            SimulatedAnnealing {
            initial_temperature,
            cooling_rate,
            stopping_temperature,
            rng: rand::rngs::ThreadRng::default(),
        }
    }
    fn estimate_initial_temperature<E: Evaluation, N: NeighborFn>(
        instance: &Instance,
        evaluation: &E,
        neighbourhood: &mut N,
        sample_size: usize,
        desired_acceptance_rate: f32,
    ) -> f32 {
        let mut energy_increases = Vec::new();
        // Start from a random initial solution
        let mut rd_initializer = RandomInitializer{};
        let mut current_solution = rd_initializer.initialize(instance);
        let mut current_fitness = evaluation.score(instance, &current_solution);

        for _ in 0..sample_size {
            let neighbor = neighbourhood.get_neighbor(&current_solution);
            let neighbor_fitness = evaluation.score(instance, &neighbor);

            let delta_energy = neighbor_fitness - current_fitness;
            if delta_energy > 0.0 {
                energy_increases.push(delta_energy);
            }

            current_solution = neighbor;
            current_fitness = neighbor_fitness;
        }
        // Calculate average increase in energy from sampled uphill moves
        let average_delta_energy = if energy_increases.is_empty() {
            // If no uphill moves, fallback to a small positive number to avoid division by zero
            1e-6
        } else {
            energy_increases.iter().sum::<f32>() / energy_increases.len() as f32
        };
        // Compute initial temperature for the desired acceptance probability
        -average_delta_energy / desired_acceptance_rate.ln()
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
            let rand = self.rng.random_range(0.0f32..1.0f32);
            let p: f32 = self.acceptance_probability(*fitness , neighbor_fitness, self.initial_temperature);
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
