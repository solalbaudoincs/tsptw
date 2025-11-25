use super::Metaheuristic;

use crate::neighborhood::{NeighborFn};
use crate::initializer::{RandomInitializer, Initializer};
use crate::shared::{Fitness, Instance, Solution};
use crate::eval::Evaluation;

use std::collections::HashMap;
use rand::Rng;
use rand::rngs::StdRng;
use rand::SeedableRng;

/// Algorithme de Recuit Simulé pour le TSPTW, utilisant des voisins générés par des opérations de swap et de 2-opt.
pub struct SimulatedAnnealing {
    initial_temperature: f32,
    cooling_rate: f32,
    stopping_temperature: f32,

    rng: StdRng,

    neighbor_buffer: Solution,
    avg_acceptance_rate: Option<f32>,
    current_fitness_avg: Option<f32>,
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
            avg_acceptance_rate: None,
            current_fitness_avg: None,
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
    fn acceptance_probability(&mut self, current_fitness: f32, neighbor_fitness: f32, temperature: f32) -> f32 {
        if neighbor_fitness < current_fitness {
            1.0
        } else {
            let proba = ((current_fitness - neighbor_fitness) / temperature).exp();
            match self.avg_acceptance_rate {
                None => self.avg_acceptance_rate = Some(proba),
                Some(_) => {}
            }
            self.avg_acceptance_rate = Some(0.9 * self.avg_acceptance_rate.unwrap() + 0.1 * proba);
            proba
        }
    }

    fn single_step<E: Evaluation, Neighborhood: NeighborFn>(
        &mut self,
        solution: &mut Solution,
        fitness: &mut Fitness,
        instance: &Instance,
        neighborhood: &mut Neighborhood,
        evaluation: &E,
    ) {

        // ensure neighbor buffer has correct length
        if self.neighbor_buffer.len() != solution.len() {
            self.neighbor_buffer.resize(solution.len(), 0);
        }

        neighborhood.get_neighbor(solution, &mut self.neighbor_buffer);


        let neighbor_fitness = evaluation.score(instance, &self.neighbor_buffer);
        let accept_prob = self.acceptance_probability(*fitness, neighbor_fitness, self.initial_temperature);
        let u = self.rng.random_range(0.0..1.0);
        let u = self.rng.random_range(0.0..1.0);

        if u < accept_prob {
            solution.clone_from_slice(&self.neighbor_buffer[..]);
            *fitness = neighbor_fitness;
            match self.current_fitness_avg {
                None => self.current_fitness_avg = Some(*fitness),
                Some(_) => {}
            } self.current_fitness_avg = Some(0.9 * self.current_fitness_avg.unwrap_or(0.0) + 0.1 * (*fitness));
        } 
        
        self.initial_temperature *= self.cooling_rate;
    }
}

impl Metaheuristic for SimulatedAnnealing {
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
                instance,
                neighborhood,
                evaluation,
            );
        }
    }


    fn get_metrics(&self) -> HashMap<String, f32> {
        let mut metrics = HashMap::new();
        metrics.insert("temperature".to_string(), self.initial_temperature);
        metrics.insert("acceptance_probability_avg".to_string(), self.avg_acceptance_rate.unwrap_or(0.0));
        metrics.insert("fitness_avg".to_string(), self.current_fitness_avg.unwrap_or(self.current_fitness_avg.unwrap_or(0.0))); // Placeholder, replace with actual average fitness calculation
        metrics
    }

    fn get_metric_names(&self) -> Vec<String> {
        vec!["temperature".to_string(), "acceptance_probability_avg".to_string(), "fitness_avg".to_string()]
    }
}