use crate::problem::solution::Solution;
mod swap;
mod twoopt;
mod utils;

pub use swap::Swap;
pub use twoopt::TwoOpt;
pub use utils::NeighborFnMixer;

pub trait NeighborFn {
    fn get_neighbor(&mut self, solution: &Solution) -> Solution;
}
