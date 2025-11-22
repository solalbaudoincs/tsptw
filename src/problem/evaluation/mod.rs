mod lexicographic;
mod random;
pub mod utils;
mod weighted;

use std::cmp::Ordering;

pub use lexicographic::Lexicographic;
pub use weighted::Weighted;

use super::instance::Instance;
use super::solution::Solution;

pub type Fitness = f64;
pub type Fitnesses = Vec<Fitness>;

pub trait Evaluation {
    fn compare(&self, problem: &Instance, a: &Solution, b: &Solution) -> Ordering;
    fn score(&self, problem: &Instance, solution: &Solution) -> Fitness;
}
