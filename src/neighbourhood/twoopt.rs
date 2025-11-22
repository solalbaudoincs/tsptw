use super::NeighborFn;
use crate::problem::Solution;

use rand::Rng;

pub struct TwoOpt {
    rng: rand::rngs::ThreadRng,
}
impl TwoOpt {
    pub fn new() -> Self {
        TwoOpt {
            rng: rand::rngs::ThreadRng::default(),
        }
    }
}

impl NeighborFn for TwoOpt {
    fn get_neighbor(&mut self, solution: &Solution) -> Solution {
        // Implementation of the 2-opt neighbor generation

        let mut rng = rand::rng();
        let len = solution.len();
        let mut i = rng.random_range(0..len);
        let mut j;

        if i == len - 1 {
            j = rng.random_range(0..len - 1);
            (i, j) = (j, i);
        } else {
            j = rng.random_range(i + 1..len);
        }
        let mut new_route: Solution = solution.clone();
        new_route[i..=j].reverse();

        return new_route;
    }
}
