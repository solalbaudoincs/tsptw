use crate::shared::Instance;
use crate::shared::Solution;
use crate::shared::Fitness;
use crate::eval::Evaluation;
use crate::algorithms::{LocalSearch, SimulatedAnnealing, HillClimbing};
use serde::Serialize;

mod swap;
mod twoopt;
mod bandit;
mod alternating;

pub use swap::Swap;
pub use twoopt::TwoOpt;
pub use bandit::Bandit;
pub use alternating::Alternating;

#[derive(Clone, Serialize, Debug)]
pub struct BanditStats {
    pub swap_selections: usize,
    pub twoopt_selections: usize,
    pub swap_avg_reward: f64,
    pub twoopt_avg_reward: f64,
}


#[derive(PartialEq, Clone, Copy, Serialize, serde::Deserialize, Debug)]
pub enum NeighborhoodType {
    Swap,
    TwoOpt,
    Alternating,
    Bandit,
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
    Alternating(Alternating),
    Bandit(Bandit),
}

impl Neighborhood {
    pub fn from_type(neighborhood_type: NeighborhoodType, instance: &Instance) -> Self {
        match neighborhood_type {
            NeighborhoodType::Swap => Neighborhood::Swap(Swap::new(instance)),
            NeighborhoodType::TwoOpt => Neighborhood::TwoOpt(TwoOpt::new(instance)),
            NeighborhoodType::Alternating => Neighborhood::Alternating(Alternating::new(instance)),
            NeighborhoodType::Bandit => {
                let arms = vec![
                    Neighborhood::Swap(Swap::new(instance)),
                    Neighborhood::TwoOpt(TwoOpt::new(instance)),
                ];
                Neighborhood::Bandit(Bandit::new(arms, 0.9))
            }
        }
    }

    pub fn from_type_with_seed(neighborhood_type: NeighborhoodType, instance: &Instance, seed: u64) -> Self {
        match neighborhood_type {
            NeighborhoodType::Swap => Neighborhood::Swap(Swap::new_with_seed(instance, seed)),
            NeighborhoodType::TwoOpt => Neighborhood::TwoOpt(TwoOpt::new_with_seed(instance, seed)),
            NeighborhoodType::Alternating => Neighborhood::Alternating(Alternating::new_with_seed(instance, seed)),
            NeighborhoodType::Bandit => {
                let arms = vec![
                    Neighborhood::Swap(Swap::new_with_seed(instance, seed)),
                    Neighborhood::TwoOpt(TwoOpt::new_with_seed(instance, seed.wrapping_add(1))),
                ];
                Neighborhood::Bandit(Bandit::new(arms, 0.9))
            }
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
                    0,      // warmup_steps
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
    fn update_reward(&mut self, _reward: f64) {}
    fn get_bandit_stats(&self) -> Option<BanditStats> { None }
}

impl NeighborFn for Neighborhood {
    fn get_neighbor(&mut self, solution: &Solution) -> &Solution {
        match self {
            Neighborhood::Swap(n) => n.get_neighbor(solution),
            Neighborhood::TwoOpt(n) => n.get_neighbor(solution),
            Neighborhood::Alternating(n) => n.get_neighbor(solution),
            Neighborhood::Bandit(n) => n.get_neighbor(solution),
        }
    }

    fn update_reward(&mut self, reward: f64) {
        match self {
            Neighborhood::Bandit(n) => n.update_reward(reward),
            _ => {}
        }
    }

    fn get_bandit_stats(&self) -> Option<BanditStats> {
        match self {
            Neighborhood::Bandit(n) => Some(n.get_bandit_stats()),
            _ => None
        }
    }
}
