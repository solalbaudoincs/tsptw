use crate::shared::{Instance, Solution, Fitness};
use crate::eval::Evaluation;

use std::collections::HashMap;

mod hill_climbing;
pub use hill_climbing::HillClimbing;

mod simulated_annealing;
pub use simulated_annealing::SimulatedAnnealing;

mod ga;
pub use ga::GeneticAlgorithm;   
pub use ga::{CrossoverType, CompetitionType};

mod aco;
pub use aco::ACO;

pub trait Metaheuristic {
    fn step<Eval: Evaluation>(
        &mut self,
        population: &mut [Solution],
        fitnesses: &mut [Fitness],
        instance: &Instance,
        evaluation: &Eval,
    );
    // a metaheuristic makes a step in the optimization process, it modifies the population and the best solution index

    fn get_metrics(&self) -> HashMap<String, f32>;

    fn get_metric_names(&self) -> Vec<String>;
}