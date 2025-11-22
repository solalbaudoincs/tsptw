use std::cmp::Ordering;

use super::Evaluation;
use crate::problem::{instance::Instance, solution::Solution};

use super::utils::run_solution;

pub struct Weighted {
    pub violation_coefficient: f64,
}

impl Evaluation for Weighted {
    fn score(&self, problem: &Instance, solution: &Solution) -> f64 {
        let (distance, violation) = run_solution(problem, solution);
        self.violation_coefficient * violation + distance
    }
    fn compare(&self, problem: &Instance, a: &Solution, b: &Solution) -> Ordering {
        let a_score = self.score(problem, a);
        let b_score = self.score(problem, b);

        if a_score < b_score {
            Ordering::Less
        } else if a_score > b_score {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}
