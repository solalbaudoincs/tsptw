mod lexicographic;
mod random;
pub mod utils;
mod weighted;

use std::cmp::Ordering;

pub use lexicographic::Lexicographic;
pub use weighted::Weighted;

use super::instance::Instance;
use super::solution::Solution;

pub type Fitness = f32;
pub type Fitnesses = Vec<Fitness>;

pub trait Evaluation {
    fn compare(&self, problem: &Instance, s1: &Solution, s2: &Solution) -> Ordering; // returns Ordering::Greater if s1 is better than s2
    fn score(&self, problem: &Instance, solution: &Solution) -> Fitness;
}
