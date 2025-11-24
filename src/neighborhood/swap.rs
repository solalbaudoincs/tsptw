use super::NeighborFn;
use crate::shared::Solution;

use rand::prelude::*;
use rand::rngs::StdRng;
use rand::SeedableRng;

pub struct Swap {
    rand: StdRng,
}

impl Swap {
    pub fn new() -> Self {
        Swap {
            rand: StdRng::from_os_rng(),
        }
    }
}
    
impl NeighborFn for Swap {
    fn get_neighbor(&mut self, solution: &Solution, buffer: &mut [u32]) -> () {
        // Implementation of the swap neighbor generation
        let len = solution.len();
        let mut i = self.rand.random_range(0..len);
        let mut j;
        if i==len-1 {
            j = self.rand.random_range(0..len-1);
            (i, j) = (j, i);
        }
        else {
            j = self.rand.random_range(i+1..len);
        }
        buffer.clone_from_slice(&solution[..]);
        buffer.swap(i, j);
    }
}