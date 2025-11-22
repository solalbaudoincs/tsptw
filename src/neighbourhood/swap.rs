use super::NeighborFn;
use crate::problem::Solution;

use rand::Rng;

pub struct Swap;

impl NeighborFn for Swap {
    fn get_neighbor(&self, solution: &Solution) -> Solution {
        // Implementation of the swap neighbor generation
        let mut rng = rand::rng();
        let len = solution.len();
        let mut i = rng.random_range(0..len);
        let mut j;
        if i==len-1 {
            j = rng.random_range(0..len-1);
            (i, j) = (j, i);
        }
        else {
            j = rng.random_range(i+1..len);
        }
        let mut new_route: Solution = solution.clone();
        new_route.swap(i, j);
        new_route
    }
}