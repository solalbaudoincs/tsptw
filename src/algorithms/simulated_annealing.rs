use super::Metaheuristic;

use crate::neighborhood::NeighborFn;
use crate::initializer::{RandomInitializer, Initializer};
use crate::shared::{Fitness, Instance, Solution};
use crate::eval::Evaluation;

use std::collections::HashMap;
use rand::Rng;
use rand::rngs::StdRng;
use rand::SeedableRng;

pub struct SimulatedAnnealing {
    initial_temperature: f32,
    cooling_rate: f32,
    stopping_temperature: f32,
    rng: StdRng,

    neighbor_buffer: Solution,
}

impl SimulatedAnnealing {

    
    pub fn new(initial_temperature: f32, cooling_rate: f32, stopping_temperature: f32, instance: &Instance) -> Self {

            let solution_size = instance.size();

            SimulatedAnnealing {
            initial_temperature,
            cooling_rate,
            stopping_temperature,
            rng: StdRng::from_os_rng(),
            neighbor_buffer: vec![0; solution_size],
        }
    }

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
    
    pub fn estimate_initial_temperature<E: Evaluation, N: NeighborFn>(
        &mut self,
        instance: &Instance,
        evaluation: &E,
        neighborhood: &mut N,
        sample_size: usize,
        desired_acceptance_rate: f32,
    ) -> f32 {
        let mut deltas = Vec::new();
        // Start from a random initial solution
        let mut rd_initializer = RandomInitializer{};
        let mut current_solution = rd_initializer.initialize(instance);
        let mut current_fitness = evaluation.score(instance, &current_solution);

        for _ in 0..sample_size {

            // Generate neighbor and puts it in buffer
            neighborhood.get_neighbor(&current_solution, &mut self.neighbor_buffer);
            let neighbor_fitness = evaluation.score(instance, &self.neighbor_buffer);

            let delta_energy = neighbor_fitness - current_fitness;
            deltas.push(delta_energy);

            current_solution = self.neighbor_buffer.clone();
            current_fitness = neighbor_fitness;
        }
        // Calculate average increase in energy from sampled uphill moves
        let average_delta = deltas.iter().sum::<f32>() / deltas.len() as f32;
        let delta_variance = deltas.iter().map(|d| (d - average_delta).powi(2)).sum::<f32>() / deltas.len() as f32;

        // Compute initial temperature for the desired acceptance probability
        -(3.0 * delta_variance.sqrt()) / desired_acceptance_rate.ln()
    }
}

impl SimulatedAnnealing {
    fn single_step<E: Evaluation, N: NeighborFn>(
        &mut self,
        solution: &mut Solution,
        fitness: &mut Fitness,
        instance: &Instance,
        neighborhood: &mut N,
        evaluation: &E,
    ) -> () {

        // Generate neighbor and puts it in buffer
        neighborhood.get_neighbor(&solution, &mut self.neighbor_buffer);

        let neighbor_fitness = evaluation.score(instance, &self.neighbor_buffer);
            let rand = self.rng.random_range(0.0..1.0);
            let p = self.acceptance_probability(*fitness , neighbor_fitness, self.initial_temperature);
        if rand < p {
            solution.copy_from_slice(&self.neighbor_buffer);
            *fitness = neighbor_fitness;
        }
        self.initial_temperature *= self.cooling_rate;
    }
}

impl Metaheuristic for SimulatedAnnealing {
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

    fn get_metrics(&self) -> HashMap<String, f32> {
        let mut metrics = HashMap::new();
        metrics.insert("temperature".to_string(), self.initial_temperature);
        metrics
    }

    fn get_metric_names(&self) -> Vec<String> {
        vec!["temperature".to_string()]
    }
}
