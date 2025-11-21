use crate::shared_types::{Instance, Population, Fitnesses};
use crate::problem::evaluation::Evaluation;

mod hill_climbing;
pub use hill_climbing::HillClimbing;

mod simulated_annealing;
pub use simulated_annealing::SimulatedAnnealing;

mod ga;

pub trait Metaheuristic {
    fn step<Eval: Evaluation>(
        &mut self,
        population: &mut Population,
        fitness: &mut Fitnesses,
        instance: &Instance,
        evaluation: &Eval,
    );
    // a metaheuristic makes a step in the optimization process, it modifies the population and the best solution index
}
