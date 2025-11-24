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

enum NeighborType {
    Swap,
    TwoOpt,
}

struct NeighborPool {
    swap_fn: Swap,
    twoopt_fn: TwoOpt,
    size: usize,
}

impl NeighborPool {
    pub fn new() -> Self {
    NeighborPool {
            swap_fn: Swap::new(),
            twoopt_fn: TwoOpt::new(),
            size: 2,
        }
    }

    pub fn get(&mut self, n_type: &NeighborType) -> &mut dyn NeighborFn {
        match n_type {
            NeighborType::Swap => &mut self.swap_fn,
            NeighborType::TwoOpt => &mut self.twoopt_fn,
        }
    }
}