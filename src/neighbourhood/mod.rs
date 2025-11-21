use crate::problem::solution::Solution;
mod swap;
mod twoopt;

pub use swap::Swap;
pub use twoopt::TwoOpt;


pub trait NeighborFn {
    fn get_neighbor(&self, solution: &Solution) -> Solution;
}