use super::NeighborFn;
use crate::problem::Solution;

use rand::Rng;

pub struct Swap {
    rng: rand::rngs::ThreadRng,
}

impl Swap {
    pub fn new() -> Self {
        Swap {
            rng: rand::rngs::ThreadRng::default(),
        }
    }
}

impl NeighborFn for Swap {
    fn get_neighbor(&mut self, solution: &Solution) -> Solution {
        // Implementation of the swap neighbor generation
        let len = solution.len();
        let mut i = self.rng.random_range(0..len);
        let mut j;
        if i == len - 1 {
            j = self.rng.random_range(0..len - 1);
            (i, j) = (j, i);
        } else {
            j = self.rng.random_range(i + 1..len);
        }
        let mut new_route: Solution = solution.clone();
        new_route.swap(i, j);
        new_route
    }
}
