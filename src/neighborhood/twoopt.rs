use crate::shared::Solution;

use super::NeighborFn;

use rand::prelude::*;
use rand::rngs::StdRng;

pub struct TwoOpt {
    rand: StdRng,
}

impl TwoOpt {
    pub fn new() -> Self {
        TwoOpt {
            rand: StdRng::from_os_rng(),
        }
    }
}

impl NeighborFn for TwoOpt {
    fn get_neighbor(&mut self, solution: &Solution, buffer: &mut [u32]) -> () {
        // Implementation of the 2-opt neighbor generation

        let len = solution.len();
        let mut i = self.rand.random_range(0..len);
        let mut j;

        if i == len - 1 {
            j = self.rand.random_range(0..len - 1);
            (i, j) = (j, i);
        } else {
            j = self.rand.random_range(i + 1..len);
        }
        buffer.clone_from_slice(&solution[..]);
        buffer[i..=j].reverse();
    }
}