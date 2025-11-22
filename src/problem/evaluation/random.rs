use rand::Rng;
use std::cmp::Ordering;

use super::Evaluation;
use crate::problem::{instance::Instance, solution::Solution};

use super::utils::run_solution;

// pub struct Random {
//     pub violation_prob: f64,
//     rng : rand::rngs::ThreadRng,
// }

// impl Evaluation for Random {
//     fn score(&self, problem: &Instance, solution: &Solution) -> f64 {
//         let (distance, violation) = run_solution(problem, solution);
//         let toss = self.rng.gen();
//         if toss < self.violation_prob {
//             violation
//         } else {
//             distance
//         }
//     }
//     fn compare(&self, problem: &Instance, a: &Solution, b: &Solution) -> Ordering {
//         let a_score = self.score(problem, a);
//         let b_score = self.score(problem, b);

//         if a_score < b_score {
//             Ordering::Less
//         } else if a_score > b_score {
//             Ordering::Greater
//         } else {
//             Ordering::Equal
//         }
//     }
// }
// impl Random {
//     pub fn new(distance_prob: f64) -> Self {
//         Self { violation_prob: distance_prob, rng: rand::rng() }
//     }
// }
