use super::Metaheuristic;

use crate::neighborhood::{NeighborFn, TwoOpt, Swap};
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
    two_opt_rate: f32,

    rng: StdRng,

    neighbor_buffer: Solution,
}

impl SimulatedAnnealing {

    pub fn new(initial_temperature: f32, two_opt_rate: f32, cooling_rate: f32, stopping_temperature: f32, instance: &Instance) -> Self {

            let solution_size = instance.size();

            SimulatedAnnealing {
            initial_temperature,
            two_opt_rate,
            cooling_rate,
            stopping_temperature,
            rng: StdRng::from_os_rng(),
            neighbor_buffer: vec![0; solution_size],
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
    fn acceptance_probability(&self, current_fitness: f32, neighbor_fitness: f32, temperature: f32) -> f32 {
        if neighbor_fitness < current_fitness {
            1.0
        } else {
            ((current_fitness - neighbor_fitness) / temperature).exp()
        }
    }

    fn single_step<E: Evaluation>(
        &mut self,
        solution: &mut Solution,
        fitness: &mut Fitness,
        instance: &Instance,
        evaluation: &E,
    ) {
        let mut rng = rand::rng();

        // ensure neighbor buffer has correct length
        if self.neighbor_buffer.len() != solution.len() {
            self.neighbor_buffer.resize(solution.len(), 0);
        }

        let r = self.rng.random_range(0.0..1.0) ;

        if r < self.two_opt_rate {
            TwoOpt::new().get_neighbor(solution, &mut self.neighbor_buffer[..]);
        } else {
            Swap::new().get_neighbor(solution, &mut self.neighbor_buffer[..]);
        }

        let neighbor_fitness = evaluation.score(instance, &self.neighbor_buffer);
        let accept_prob = self.acceptance_probability(*fitness, neighbor_fitness, self.initial_temperature);
        let u = rng.random_range(0.0..1.0);
        if u < accept_prob {
            solution.clone_from_slice(&self.neighbor_buffer[..]);
            *fitness = neighbor_fitness;
        }
    }
}

impl Metaheuristic for SimulatedAnnealing {
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
        } self.initial_temperature *= self.cooling_rate;
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
