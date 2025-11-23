use crate::neighbourhood::NeighborFn;
use crate::problem::evaluation::Fitnesses;
use crate::problem::{Evaluation, Instance, Population};
use std::collections::HashMap;

mod hill_climbing;
pub use hill_climbing::HillClimbing;

mod simulated_annealing;
pub use simulated_annealing::SimulatedAnnealing;

// mod ga;
// pub use ga::GeneticAlgorithm;   

//mod ga;

pub trait Metaheuristic {
    fn step<Eval: Evaluation, N: NeighborFn>(
        &mut self,
        population: &mut Population,
        fitnesses: &mut Fitnesses,
        neighbourhood: &mut N,
        instance: &Instance,
        evaluation: &Eval,
    );
    // a metaheuristic makes a step in the optimization process, it modifies the population and the best solution index

    fn get_metrics(&self) -> HashMap<String, f32> {
        HashMap::new()
    }

    fn get_metric_names(&self) -> Vec<String> {
        vec![]
    }
}
