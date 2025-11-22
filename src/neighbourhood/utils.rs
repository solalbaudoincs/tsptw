use super::NeighborFn;
use crate::problem::Solution;

use rand::Rng;

pub struct NeighborFnMixer {
    neighborhood_fns: Vec<Box<dyn NeighborFn>>,
    weights: Vec<f32>,
    rng: rand::rngs::ThreadRng,
}

impl NeighborFnMixer {
    pub fn new(
        neighborhood_fns: Vec<Box<dyn NeighborFn>>,
        weights: Vec<f32>,
    ) -> Self {
        let weight_sum: f32 = weights.iter().sum();
        let normalized_weights: Vec<f32> = weights
            .iter()
            .map(|w| *w / weight_sum)
            .collect();

        NeighborFnMixer {
            neighborhood_fns,
            weights: normalized_weights,
            rng: rand::rngs::ThreadRng::default(),
        }
    }
}

impl NeighborFn for NeighborFnMixer {
    fn get_neighbor(&mut self, solution: &Solution) -> Solution {
        let r: f32 = self.rng.random_range(0.0f32..1.0f32);
        let mut cumulative_weight: f32 = 0.0f32;

        for (i, weight) in self.weights.iter().enumerate() {
            cumulative_weight += *weight;
            if r <= cumulative_weight {
                return self.neighborhood_fns[i].get_neighbor(solution);
            }
        }
        // Fallback in case of rounding errors
        let n = self.neighborhood_fns.len();
       return self.neighborhood_fns[n - 1].get_neighbor(solution);
    }
}
