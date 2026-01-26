
use super::Initializer;
use crate::shared::{Instance, Solution};

use rand::seq::SliceRandom;
use rand::SeedableRng;
use rand::rngs::StdRng;

pub struct RandomInitializer;


impl Initializer for RandomInitializer {
    fn initialize(&mut self, instance: &Instance) -> Solution {
        let node_number = instance.size();
        let mut rng = rand::rng();
        let mut solution: Solution = (0..node_number as u32).collect();
        solution.shuffle(&mut rng);
        solution
    }
}

pub struct SeededRandomInitializer {
    rng: StdRng,
}

impl SeededRandomInitializer {
    pub fn new(seed: u64) -> Self {
        SeededRandomInitializer {
            rng: StdRng::seed_from_u64(seed),
        }
    }
}

impl Initializer for SeededRandomInitializer {
    fn initialize(&mut self, instance: &Instance) -> Solution {
        let node_number = instance.size();
        let mut solution: Solution = (0..node_number as u32).collect();
        solution.shuffle(&mut self.rng);
        solution
    }
}