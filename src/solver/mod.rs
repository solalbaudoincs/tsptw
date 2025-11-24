mod ga_sa;

use crate::shared::{Instance};
use crate::eval::Evaluation;

pub trait Solver {
    fn solve<E: Evaluation>(
        &mut self, 
        max_iterations: usize, 
        instance: &Instance, 
        evaluation: &E,
    ) -> ();
}