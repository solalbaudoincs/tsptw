
use super::Initializer;
use crate::problem::{Instance, Solution};

use rand::seq::SliceRandom;

pub struct RandomInitializer;


impl Initializer for RandomInitializer {
    fn initialize(&mut self, problem: &Instance) -> Solution {
        let node_number = problem.graph.len();
        let mut rng = rand::rng();
        let mut solution: Solution = (0..node_number as u32).collect();
        solution.shuffle(&mut rng);
        solution
    }
}