use super::NeighborFn;
use crate::problem::Solution;

use rand::Rng;

pub struct NeighborMixer {
    neighborhood_fns: Vec<Box<dyn NeighborFn>>,
    weights: Vec<f64>,
}

impl NeighborFn for NeighborMixer {
    fn get_neighbor(&self, solution: &Solution) -> Solution {
        let mut rng = rand::rng();
        let r: f64 = rng.random_range(0.0..1.0);
        let mut cumulative_weight = 0.0;

        for (i, weight) in self.weights.iter().enumerate() {
            cumulative_weight += *weight;
            if r <= cumulative_weight {
                return self.neighborhood_fns[i].get_neighbor(solution);
            }
        }

        // Fallback in case of rounding errors
        return self.neighborhood_fns.last().unwrap().get_neighbor(solution);
    }
}
