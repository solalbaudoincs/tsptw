use std::cmp::Ordering;

use super::Evaluation;
use super::utils::run_solution;

use crate::shared::{Instance, Solution};

pub struct Weighted {
    pub violation_coefficient: f32,
}

impl Evaluation for Weighted {

    fn score(&self, problem: &Instance, solution: &Solution) -> f32 {
        let eval = run_solution(problem, solution);
        let (distance, violation) = (eval.total_distance, eval.violation_time);
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


pub struct GeneralWeighted {
    pub total_distance_weight: f32,
    pub violation_time_weight: f32,
    pub total_time_weight: f32,
    pub delay_weight: f32,
}

impl Evaluation for GeneralWeighted {
    fn score(&self, problem: &Instance, solution: &Solution) -> f32 {
        let eval = run_solution(problem, solution);

        self.total_distance_weight * eval.total_distance +
        self.violation_time_weight * eval.violation_time +
        self.total_time_weight * eval.total_time +
        self.delay_weight * eval.delay
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