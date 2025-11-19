use crate::neighbourhood::NeighbourhoodGenerator;
use crate::problem::{evaluation::Evaluation, instance::Instance, solution::Population};

mod hill_climbing;
pub use hill_climbing::HillClimbing;

pub trait Metaheuristic {
    fn step(&mut self, 
            population: &mut Population, 
            best: usize,
            instance: &Instance, 
            neighbourhood: &NeighbourhoodGenerator, 
            evaluation: &dyn Evaluation
        );
        // a metaheuristic makes a step in the optimization process, it modifies the population and the best solution index
}
