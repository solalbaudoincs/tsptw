use crate::eval::Evaluation;
use crate::neighborhood::Neighborhood;
use crate::shared::{Fitness, Instance, Solution};

use std::collections::HashMap;

mod hill_climbing;
pub use hill_climbing::HillClimbing;

mod simulated_annealing;
mod vns; // Added VNS module
pub use simulated_annealing::SimulatedAnnealing;
pub use vns::VNS; // Added VNS public export

mod ga;
pub use ga::GeneticAlgorithm;
pub use ga::{CompetitionType, CrossoverType};

mod aco;
pub use aco::ACO;



pub trait Metaheuristic<Eval: Evaluation>: Send + Sync {
    fn step(
        &mut self,
        population: &mut [Solution],
        fitnesses: &mut [Fitness],
        instance: &Instance,
        evaluation: &Eval,
    );
    // a metaheuristic makes a step in the optimization process, it modifies the population and the best solution index

    fn get_metrics(&self) -> HashMap<String, f32> {
        HashMap::new()
    }

    fn get_metric_names(&self) -> Vec<String> {
        Vec::new()
    }

    fn stop_condition_met(&self) -> bool;
    
    fn get_iteration(&self) -> usize;

    fn get_best_solution(
        &self,
        population: &[Solution],
        fitnesses: &[Fitness],
    ) -> Option<(Vec<u32>, f32, f32)> {
        if population.is_empty() || fitnesses.is_empty() {
            return None;
        }

        let (best_index, _) = fitnesses
            .iter()
            .enumerate()
            .min_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .unwrap();

        let best_solution = population[best_index].clone();
        let best_fitness = fitnesses[best_index];

        // Placeholder for violation calculation
        let violation = 0.0;

        Some((best_solution, best_fitness, violation))
    }
}

pub trait LocalSearch<Eval: Evaluation>: Send + Sync {
    fn search(
        &mut self,
        solution: &mut Solution,
        fitness: &mut Fitness,
        instance: &Instance,
        evaluation: &Eval,
    );

    fn reset(&mut self);

    fn change_neighborhood(&mut self, neighborhood: Neighborhood);
}
