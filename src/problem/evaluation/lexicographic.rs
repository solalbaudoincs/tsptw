use std::cmp::Ordering;

use super::Evaluation;
use crate::problem::{instance::Instance, solution::Solution};

use super::utils::run_solution;

pub struct Lexicographic {
    distance_first: bool,
}

impl Lexicographic {
    pub fn new(distance_first: bool) -> Self {
        Self { distance_first }
    }
}

impl Evaluation for Lexicographic {
    fn compare(&self, problem: &Instance, a: &Solution, b: &Solution) -> Ordering {
        let (a_distance, a_violation) = run_solution(problem, a);
        let (b_distance, b_violation) = run_solution(problem, b);

        let ((a_primary, a_secondary), (b_primary, b_secondary)) = if self.distance_first {
            ((a_distance, a_violation), (b_distance, b_violation))
        } else {
            ((a_violation, a_distance), (b_violation, b_distance))
        };

        if a_primary < b_primary {
            Ordering::Less
        } else if a_primary > b_primary {
            Ordering::Greater
        } else if a_secondary < b_secondary {
            Ordering::Less
        } else if a_secondary > b_secondary {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
    fn score(&self, problem: &Instance, solution: &Solution) -> f64 {
        let (distance, violation) = run_solution(problem, solution);
        if violation > 0.0 {
            violation
        } else {
            distance
        }
    }
}
