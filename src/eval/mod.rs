mod lexicographic;
mod random;
pub mod utils;
mod weighted;

use std::cmp::Ordering;

pub use lexicographic::Lexicographic;
pub use weighted::Weighted;
pub use weighted::GeneralWeighted;

pub type Fitness = f32;
pub type Fitnesses = Vec<Fitness>;

#[derive(PartialEq, Clone, Copy)]
pub enum EvaluationType {
    Weighted,
    Lexicographic,
}

use crate::shared::{Instance, Solution};

pub trait Evaluation {
    fn compare(&self, instance: &Instance, s1: &Solution, s2: &Solution) -> Ordering; // returns Ordering::Greater if s1 is better than s2
    fn score(&self, instance: &Instance, solution: &Solution) -> Fitness;
}
