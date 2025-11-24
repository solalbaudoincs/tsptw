use crate::shared::Solution;

mod swap;
mod twoopt;
mod utils;

pub use swap::Swap;
pub use twoopt::TwoOpt;
//pub use utils::NeighborFnMixer;

pub trait NeighborFn {
    fn get_neighbor(&mut self, solution: &Solution, buffer: &mut [u32]) -> ();
}

#[derive(PartialEq, Clone, Copy)]
pub enum NeighborhoodType {
    Swap,
    TwoOpt,
}