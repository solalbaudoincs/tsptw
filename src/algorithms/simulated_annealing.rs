use super::LocalSearch;
use super::Metaheuristic;

use crate::eval::Evaluation;
use crate::neighborhood::{NeighborFn, Neighborhood};
use crate::shared::{Fitness, Instance, Solution};

use rand::Rng;
use rand::SeedableRng;
use rand::rngs::StdRng;
use std::collections::HashMap;

/// Algorithme de Recuit Simulé pour le TSPTW, utilisant des voisins générés par des opérations de swap et de 2-opt.
#[derive(Clone)]
pub struct SimulatedAnnealing {
    neighborhood: Neighborhood, // Changed to Enum
    temperature: f32,
    initial_temperature: f32,
    cooling_rate: f32,
    current_avg_fitness: Option<f32>,
    stopping_temperature: f32,

    rng: StdRng,

    // Average acceptance rate of worse solutions
    pub avg_acceptance_rate: Option<f32>,
    pub acceptance_smoothing_factor: f32,

    // Desired initial acceptance rate
    pub initial_acceptance_rate: f32,
    pub is_attained_initial_rate: bool,

    // Average change in fitness when accepting a worse solution
    pub avg_delta_fitness: Option<f32>,
    pub delta_fitness_smoothing_factor: f32,

    // Avg fitness of the current solutions
    pub current_fitness_avg: Option<f32>,

    iteration: usize,
}

impl SimulatedAnnealing {
    pub fn new(
        initial_temperature: f32,
        cooling_rate: f32,
        stopping_temperature: f32,
        acceptance_smoothing_factor: f32,
        initial_acceptance_rate: f32,
        delta_fitness_smoothing_factor: f32,
        neighborhood: Neighborhood, // Changed to Enum
    ) -> Self {
        SimulatedAnnealing {
            neighborhood,
            temperature: initial_temperature,
            initial_temperature,
            cooling_rate,
            stopping_temperature,
            rng: StdRng::from_os_rng(),

            avg_acceptance_rate: None,
            
            
            acceptance_smoothing_factor,
            initial_acceptance_rate,
            is_attained_initial_rate: false,
            avg_delta_fitness: None,
            delta_fitness_smoothing_factor,
            current_fitness_avg: None,

            iteration: 0,
        }
    }

    pub fn update_avg_acceptance_rate(
        accepted: bool, probability: f32, avg_acceptance_rate: &mut Option<f32>, acceptance_smoothing_factor: f32
    ) {
        let current_rate = if accepted { 1.0 } else { probability };

        let avg = avg_acceptance_rate.get_or_insert(current_rate);
        *avg = acceptance_smoothing_factor * (*avg)
            + (1.0 - acceptance_smoothing_factor) * current_rate;
    }

    pub fn update_temperature(&mut self, current_fitness: &f32, neighbor_fitness: f32) {
        let delta = (neighbor_fitness - *current_fitness).abs();
        
        // Update average delta fitness
        let avg = self.avg_delta_fitness.get_or_insert(delta);
        *avg = self.delta_fitness_smoothing_factor * (*avg) + (1.0 - self.delta_fitness_smoothing_factor) * delta;

        // Update temperature based on average delta fitness and average acceptance probability
        if let (Some(avg_delta), Some(avg_rate)) =
            (self.avg_delta_fitness, self.avg_acceptance_rate)
        {
            let new_temp = -avg_delta / avg_rate.ln();
            self.temperature = new_temp.max(self.temperature);
        }
    }

    
    pub fn start_condition_met(&mut self) -> bool {
        if self.is_attained_initial_rate {
            return true;
        }

        if self.avg_acceptance_rate.unwrap_or(0.0) > self.initial_acceptance_rate {
            self.is_attained_initial_rate = true;
            return true;
        }

        false
    }

    
    pub fn acceptance_probability(
        current_fitness: f32,
        neighbor_fitness: f32,
        temperature: f32,
    ) -> f32 {

        if neighbor_fitness < current_fitness {
            1.0
        } 
        
        else {
            ((current_fitness - neighbor_fitness) / temperature).exp()
        }
    }

    fn single_step<Eval: Evaluation>(
        &mut self,
        solution: &mut Solution,
        fitness: &mut Fitness,
        instance: &Instance,
        evaluation: &Eval,
    ) {
        let neighbor = self.neighborhood.get_neighbor(solution);

        let neighbor_fitness = evaluation.score(instance, neighbor);
        let accept_prob = Self::acceptance_probability(*fitness, neighbor_fitness, self.temperature);
        Self::update_avg_acceptance_rate(
            neighbor_fitness < *fitness, 
            accept_prob, 
            &mut self.avg_acceptance_rate, 
            self.acceptance_smoothing_factor
        );
        
        let u = self.rng.random_range(0.0..1.0);

        if u < accept_prob {
            solution.clone_from_slice(neighbor);
            *fitness = neighbor_fitness;

            let current_avg_fitness = self.current_fitness_avg.get_or_insert(*fitness);
            *current_avg_fitness = 0.9 * (*current_avg_fitness) + 0.1 * (*fitness);
        }

        // Update temperature, either by cooling or by updating the temperature until the initial acceptance rate is reached
        if self.start_condition_met() {
            self.temperature *= self.cooling_rate;
        } else {
            self.update_temperature(fitness, neighbor_fitness);
        }
    }

    fn _stop_condition_met(&self) -> bool {
        self.temperature < self.stopping_temperature
    }

    fn _reset(&mut self) {
        self.temperature = self.initial_temperature;
        self.avg_acceptance_rate = None;
        self.is_attained_initial_rate = false;
        self.avg_delta_fitness = None;
        self.current_fitness_avg = None;
    }
}

impl<Eval: Evaluation> Metaheuristic<Eval> for SimulatedAnnealing {
    fn step(
        &mut self,
        population: &mut [Solution],
        fitness: &mut [Fitness],
        instance: &Instance,
        evaluation: &Eval,
    ) {
        for i in 0..population.len() {
            self.single_step::<Eval>(&mut population[i], &mut fitness[i], instance, evaluation);
        }

        self.iteration += 1;
    }

    fn get_metrics(&self) -> HashMap<String, f32> {
        let mut metrics = HashMap::new();
        metrics.insert("temperature".to_string(), self.temperature);
        metrics.insert(
            "acceptance_probability_avg".to_string(),
            self.avg_acceptance_rate.unwrap_or(0.0),
        );
        metrics.insert(
            "fitness_avg".to_string(),
            self.current_fitness_avg.unwrap_or(0.0),
        );
        metrics
    }

    fn get_metric_names(&self) -> Vec<String> {
        vec![
            "temperature".to_string(),
            "acceptance_probability_avg".to_string(),
            "fitness_avg".to_string(),
        ]
    }

    fn stop_condition_met(&self) -> bool {
        self._stop_condition_met()
    }

    fn get_iteration(&self) -> usize {
        // Placeholder implementation, replace with actual iteration tracking if needed
        self.iteration
    }
}

impl<Eval: Evaluation> LocalSearch<Eval> for SimulatedAnnealing {
    fn search(
        &mut self,
        solution: &mut Solution,
        fitness: &mut Fitness,
        instance: &Instance,
        evaluation: &Eval,
    ) {
        while !self._stop_condition_met() {
            self.single_step::<Eval>(solution, fitness, instance, evaluation);
        }
    }

    fn reset(&mut self) {
        self._reset();
    }

    fn change_neighborhood(&mut self, neighborhood: Neighborhood) {
        self._reset();
        self.neighborhood = neighborhood;
    }
}
