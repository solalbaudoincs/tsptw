use std::cmp::Ordering;

use super::Evaluation;
use super::utils::run_solution;

use crate::shared::{Instance, Solution};

#[derive(Clone)]
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
        let eval_a = run_solution(problem, a);
        let eval_b = run_solution(problem, b);

        let ((a_primary, a_secondary), (b_primary, b_secondary)) = if self.distance_first {
            (
                (eval_a.total_distance, eval_a.violation_time),
                (eval_b.total_distance, eval_b.violation_time),
            )
        } else {
            (
                (eval_a.violation_time, eval_a.total_distance),
                (eval_b.violation_time, eval_b.total_distance),
            )
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

    fn score(&self, problem: &Instance, solution: &Solution) -> f32 {
        let eval = run_solution(problem, solution);
        let (distance, violation) = (eval.total_distance, eval.violation_time);
        if violation > 0.0f32 {
            violation
        } else {
            distance
        }
    }
}
