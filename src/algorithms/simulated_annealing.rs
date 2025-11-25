use super::Metaheuristic;
use crate::neighbourhood::NeighborFn;
use crate::problem::evaluation::{Evaluation, Fitness, Fitnesses};
use crate::problem::{Instance, Population, Solution};
use crate::initializer::{RandomInitializer, Initializer};
use std::collections::HashMap;

use rand::Rng;
use rand::SeedableRng;

pub struct SimulatedAnnealing {
    initial_temperature: f32,
    cooling_rate: f32,
    avg_acceptance_rate: Option<f32>,
    current_avg_fitness: Option<f32>,
    stopping_temperature: f32,
    rng: rand::rngs::StdRng,
}

impl SimulatedAnnealing {
    fn acceptance_probability(
        &mut self,
        current_fitness: f32,
        neighbor_fitness: f32,
        temperature: f32,
    ) -> f32 {
        if neighbor_fitness < current_fitness {
            1.0
        } else {
            let prb = ((current_fitness - neighbor_fitness) / temperature).exp();

            match self.avg_acceptance_rate {
                None => self.avg_acceptance_rate = Some(prb),
                Some(_) => (),
            }
            self.avg_acceptance_rate = Some(0.9 * self.avg_acceptance_rate.unwrap_or(0.0) + 0.1 * prb);
            prb
        }
    }
    pub fn new(initial_temperature: f32, cooling_rate: f32, stopping_temperature: f32) -> Self {
            SimulatedAnnealing {
            initial_temperature,
            cooling_rate,
            stopping_temperature,
            rng: rand::rngs::StdRng::from_os_rng(),
            avg_acceptance_rate: None,
            current_avg_fitness: None,
        }
    }
    pub fn estimate_initial_temperature<E: Evaluation, N: NeighborFn>(
        instance: &Instance,
        evaluation: &E,
        neighbourhood: &mut N,
        sample_size: usize,
        desired_acceptance_rate: f32,
    ) -> f32 {
        let mut deltas = Vec::new();
        // Start from a random initial solution
        let mut rd_initializer = RandomInitializer{};
        let mut current_solution = rd_initializer.initialize(instance);
        let mut current_fitness = evaluation.score(instance, &current_solution);

        for _ in 0..sample_size {
            let neighbor = neighbourhood.get_neighbor(&current_solution);
            let neighbor_fitness = evaluation.score(instance, &neighbor);

            let delta_energy = neighbor_fitness - current_fitness;
            deltas.push(delta_energy);

            current_solution = neighbor;
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
            
            match self.current_avg_fitness {
                None => self.current_avg_fitness = Some(*fitness),
                Some(_) => (),
            }
            self.current_avg_fitness = Some(0.9 * self.current_avg_fitness.unwrap_or(0.0) + 0.1 * *fitness);

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

    fn get_metrics(&self) -> HashMap<String, f32> {
        let mut metrics = HashMap::new();
        metrics.insert("temperature".to_string(), self.initial_temperature);
        metrics.insert("avg_acceptance_rate".to_string(), self.avg_acceptance_rate.unwrap_or(0.0));
        metrics.insert("current_avg_fitness".to_string(), self.current_avg_fitness.unwrap_or(0.0));
        metrics
    }

    fn get_metric_names(&self) -> Vec<String> {
        vec!["temperature".to_string(), "avg_acceptance_rate".to_string(), "current_avg_fitness".to_string()]
    }
}
