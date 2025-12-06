use super::Metaheuristic;

use crate::neighborhood::{NeighborFn};
use crate::shared::{Fitness, Instance, Solution};
use crate::eval::Evaluation;

use std::collections::HashMap;
use rand::Rng;
use rand::rngs::StdRng;
use rand::SeedableRng;

/// Algorithme de Recuit Simulé pour le TSPTW, utilisant des voisins générés par des opérations de swap et de 2-opt.
pub struct SimulatedAnnealing {

    temperature: f32,
    cooling_rate: f32,
    stopping_temperature: f32,

    rng: StdRng,

    neighbor_buffer: Solution,

    // Average acceptance rate of worse solutions
    avg_acceptance_rate: Option<f32>,
    acceptance_smoothing_factor: f32,

    // Desired initial acceptance rate
    initial_acceptance_rate: f32,
    is_attained_initial_rate: bool,

    // Average change in fitness when accepting a worse solution
    avg_delta_fitness: Option<f32>,
    delta_fitness_smoothing_factor: f32,


    // Avg fitness of the current solutions
    current_fitness_avg: Option<f32>,
}

impl SimulatedAnnealing {

    pub fn new(
        initial_temperature: f32,
        cooling_rate: f32,
        stopping_temperature: f32,
        acceptance_smoothing_factor: f32,
        initial_acceptance_rate: f32,
        delta_fitness_smoothing_factor: f32,
        instance: &Instance
    ) -> Self {
        let solution_size = instance.size();

        SimulatedAnnealing {
            temperature: initial_temperature,
            cooling_rate,
            stopping_temperature,
            rng: StdRng::from_os_rng(),
            neighbor_buffer: vec![0; solution_size],
            avg_acceptance_rate: None,
            acceptance_smoothing_factor,
            initial_acceptance_rate,
            is_attained_initial_rate: false,
            avg_delta_fitness: None,
            delta_fitness_smoothing_factor,
            current_fitness_avg: None,
        }
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
            self.avg_acceptance_rate = Some(
                self.acceptance_smoothing_factor * self.avg_acceptance_rate.unwrap() + (1.0 - self.acceptance_smoothing_factor) * proba
            );
            proba
        }
    }

    fn update_avg_delta_fitness(&mut self, delta: f32) {
        match self.avg_delta_fitness {
            None => self.avg_delta_fitness = Some(delta),
            Some(_) => {}
        }
        self.avg_delta_fitness = Some(
            self.delta_fitness_smoothing_factor * self.avg_delta_fitness.unwrap() + (1.0 - self.delta_fitness_smoothing_factor) * delta
        );
    }

    fn start_condition_met(&mut self) -> bool {

        if self.is_attained_initial_rate {
            return true;
        }

        if self.avg_acceptance_rate.unwrap_or(0.0) > self.initial_acceptance_rate {
            self.is_attained_initial_rate = true;
            return true;
        }

        false
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
        let accept_prob = self.acceptance_probability(*fitness, neighbor_fitness, self.temperature);
        let u = self.rng.random_range(0.0..1.0);

        if u < accept_prob {
            solution.clone_from_slice(&self.neighbor_buffer[..]);
            *fitness = neighbor_fitness;
            match self.current_fitness_avg {
                None => self.current_fitness_avg = Some(*fitness),
                Some(_) => {}
            } self.current_fitness_avg = Some(0.9 * self.current_fitness_avg.unwrap_or(0.0) + 0.1 * (*fitness));
        } 
        
        if self.start_condition_met() {
            self.temperature *= self.cooling_rate;
        }

        else { 
            let delta = (neighbor_fitness - *fitness).abs();
            self.update_avg_delta_fitness(delta);
            let new_temp = -self.avg_delta_fitness.unwrap_or(1.0) / self.avg_acceptance_rate.unwrap_or(0.1).ln();
            self.temperature = new_temp.max(self.temperature);
        }
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
        metrics.insert("temperature".to_string(), self.temperature);
        metrics.insert("acceptance_probability_avg".to_string(), self.avg_acceptance_rate.unwrap_or(0.0));
        metrics.insert("fitness_avg".to_string(), self.current_fitness_avg.unwrap_or(self.current_fitness_avg.unwrap_or(0.0))); // Placeholder, replace with actual average fitness calculation
        metrics
    }

    fn get_metric_names(&self) -> Vec<String> {
        vec!["temperature".to_string(), "acceptance_probability_avg".to_string(), "fitness_avg".to_string()]
    }
}