use crate::shared::Instance;
use crate::shared::Solution;
use crate::shared::Fitness;
use crate::eval::Evaluation;
use crate::algorithms::{LocalSearch, SimulatedAnnealing, HillClimbing};

mod swap;
mod twoopt;

pub use swap::Swap;
pub use twoopt::TwoOpt;
//pub use utils::NeighborFnMixer;


#[derive(PartialEq, Clone, Copy)]
pub enum NeighborhoodType {
    Swap,
    TwoOpt,
}

impl Default for NeighborhoodType {
    fn default() -> Self {
        NeighborhoodType::Swap
    }
}

#[derive(Clone)]
pub enum Neighborhood {
    Swap(Swap),
    TwoOpt(TwoOpt),
}

impl Neighborhood {
    pub fn from_type(neighborhood_type: NeighborhoodType, instance: &Instance) -> Self {
        match neighborhood_type {
            NeighborhoodType::Swap => Neighborhood::Swap(Swap::new(instance)),
            NeighborhoodType::TwoOpt => Neighborhood::TwoOpt(TwoOpt::new(instance)),
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
pub enum LocalSearchType {
    Swap,
    TwoOpt,
    SimulatedAnnealing,
    HillClimbing,
}

impl Default for LocalSearchType {
    fn default() -> Self {
        LocalSearchType::Swap
    }
}

#[derive(Clone)]
pub enum LocalSearchImpl {
    Swap(Swap),
    TwoOpt(TwoOpt),
    SimulatedAnnealing(SimulatedAnnealing),
    HillClimbing(HillClimbing),
}

impl LocalSearchImpl {
    pub fn from_type(ls_type: LocalSearchType, instance: &Instance) -> Self {
        match ls_type {
            LocalSearchType::Swap => LocalSearchImpl::Swap(Swap::new(instance)),
            LocalSearchType::TwoOpt => LocalSearchImpl::TwoOpt(TwoOpt::new(instance)),
            LocalSearchType::SimulatedAnnealing => {
                // Default parameters for SA as local search
                let neighborhood = Neighborhood::from_type(NeighborhoodType::Swap, instance);
                LocalSearchImpl::SimulatedAnnealing(SimulatedAnnealing::new(
                    100.0,  // initial_temperature
                    0.95,   // cooling_rate
                    0.01,   // stopping_temperature
                    0.9,    // acceptance_smoothing_factor
                    0.8,    // initial_acceptance_rate
                    0.9,    // delta_fitness_smoothing_factor
                    neighborhood,
                    0,      // backtracking_interval
                ))
            },
            LocalSearchType::HillClimbing => {
                // Default parameters for HC as local search
                let neighborhood = Neighborhood::from_type(NeighborhoodType::Swap, instance);
                LocalSearchImpl::HillClimbing(HillClimbing::new(
                    10,     // step
                    100,    // max_steps
                    neighborhood,
                ))
            },
        }
    }
}

impl<Eval: Evaluation> LocalSearch<Eval> for LocalSearchImpl {
    fn search(
        &mut self,
        solution: &mut Solution,
        fitness: &mut Fitness,
        instance: &Instance,
        evaluation: &Eval,
    ) {
        match self {
            LocalSearchImpl::Swap(ls) => ls.search(solution, fitness, instance, evaluation),
            LocalSearchImpl::TwoOpt(ls) => ls.search(solution, fitness, instance, evaluation),
            LocalSearchImpl::SimulatedAnnealing(ls) => ls.search(solution, fitness, instance, evaluation),
            LocalSearchImpl::HillClimbing(ls) => ls.search(solution, fitness, instance, evaluation),
        }
    }

    fn reset(&mut self) {
        // Only SA and HC have meaningful reset logic
        // But we can't call it here without Eval type
        // So we leave it empty for now
    }

    fn change_neighborhood(&mut self, _neighborhood: Neighborhood) {
        // Only SA and HC support changing neighborhood
        // But we can't call it here without Eval type
        // So we leave it empty for now
    }
}


pub trait NeighborFn : Send + Sync {
    fn get_neighbor(&mut self, solution: &Solution) -> &Solution;
}

impl NeighborFn for Neighborhood {
    fn get_neighbor(&mut self, solution: &Solution) -> &Solution {
        match self {
            Neighborhood::Swap(n) => n.get_neighbor(solution),
            Neighborhood::TwoOpt(n) => n.get_neighbor(solution),
        }
    }
}
