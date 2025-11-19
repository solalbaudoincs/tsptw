mod lexicographic;
pub mod utils;

use std::cmp::Ordering;

pub use lexicographic::Lexicographic;

use super::solution::Solution;
use super::instance::Instance;

pub trait Evaluation {
    fn compare(&self, problem: &Instance, a: &Solution, b: &Solution) -> Ordering;
    fn score(&self, problem: &Instance, solution: &Solution) -> f64;
}
