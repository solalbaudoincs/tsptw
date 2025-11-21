mod lexicographic;
mod weighted;
mod random;
pub mod utils;

use std::cmp::Ordering;

pub use lexicographic::Lexicographic;
pub use weighted::Weighted;
pub use random::Random;

use super::solution::Solution;
use super::instance::Instance;

pub trait Evaluation {
    fn compare(&self, problem: &Instance, a: &Solution, b: &Solution) -> Ordering;
    fn score(&self, problem: &Instance, solution: &Solution) -> f64;
}
