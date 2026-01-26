use super::{NeighborFn, BanditStats};
use crate::shared::{Solution, Instance, Fitness};
use crate::algorithms::LocalSearch;
use crate::eval::Evaluation;
use crate::neighborhood::Neighborhood;


#[derive(Clone)]
pub struct Bandit {
    arms: Vec<Neighborhood>,
    rewards: Vec<f64>,
    counts: Vec<f64>,
    total_pulls: f64,
    decay: f64,
    last_selected: usize,
}

impl Bandit {
    pub fn new(arms: Vec<Neighborhood>, decay: f64) -> Self {
        let n = arms.len();
        Bandit {
            arms,
            rewards: vec![0.0; n],
            counts: vec![0.0; n],
            total_pulls: 0.0,
            decay,
            last_selected: 0,
        }
    }

    pub fn new_with_seed(_instance: &Instance, arms: Vec<Neighborhood>, decay: f64, _seed: u64) -> Self {
        // Arms are already seeded during construction, so this is just a convenience constructor
        Self::new(arms, decay)
    }

    fn select_arm(&mut self) -> usize {
        let n = self.arms.len();

        for i in 0..n {
            if self.counts[i] < 1.0 {
                return i;
            }
        }

        let mut best_arm = 0;
        let mut best_ucb = f64::NEG_INFINITY;
        let ln_total = self.total_pulls.ln();

        for i in 0..n {
            let avg_reward = self.rewards[i] / self.counts[i];
            let exploration = (2.0 * ln_total / self.counts[i]).sqrt();
            let ucb = avg_reward + exploration;
            if ucb > best_ucb {
                best_ucb = ucb;
                best_arm = i;
            }
        }

        best_arm
    }

    fn update(&mut self, arm: usize, reward: f64) {
        for i in 0..self.arms.len() {
            self.rewards[i] *= self.decay;
            self.counts[i] *= self.decay;
        }

        self.rewards[arm] += reward;
        self.counts[arm] += 1.0;
        self.total_pulls = self.counts.iter().sum();
    }

    pub fn update_reward(&mut self, reward: f64) {
        self.update(self.last_selected, reward);
    }

    pub fn get_bandit_stats(&self) -> BanditStats {
        BanditStats {
            swap_selections: self.counts.get(0).map(|&c| c as usize).unwrap_or(0),
            twoopt_selections: self.counts.get(1).map(|&c| c as usize).unwrap_or(0),
            swap_avg_reward: if self.counts.get(0).map(|&c| c > 0.0).unwrap_or(false) {
                self.rewards.get(0).copied().unwrap_or(0.0) / self.counts.get(0).copied().unwrap_or(1.0)
            } else {
                0.0
            },
            twoopt_avg_reward: if self.counts.get(1).map(|&c| c > 0.0).unwrap_or(false) {
                self.rewards.get(1).copied().unwrap_or(0.0) / self.counts.get(1).copied().unwrap_or(1.0)
            } else {
                0.0
            },
        }
    }
}

impl NeighborFn for Bandit {
    fn get_neighbor(&mut self, solution: &Solution) -> &Solution {
        self.last_selected = self.select_arm();
        self.arms[self.last_selected].get_neighbor(solution)
    }
}

impl<Eval: Evaluation> LocalSearch<Eval> for Bandit {
    fn search(
        &mut self,
        solution: &mut Solution,
        fitness: &mut Fitness,
        instance: &Instance,
        evaluation: &Eval,
    ) {
        let arm = self.select_arm();
        self.last_selected = arm;
        let old_fitness = *fitness;

        let neighbor = self.arms[arm].get_neighbor(solution);
        let neighbor_fitness = evaluation.score(instance, neighbor);

        if neighbor_fitness < *fitness {
            solution.clone_from_slice(neighbor);
            *fitness = neighbor_fitness;
        }

        let reward = ((old_fitness - neighbor_fitness) as f64).max(0.0);
        self.update(arm, reward);
    }

    fn reset(&mut self) {
        let n = self.arms.len();
        self.rewards = vec![0.0; n];
        self.counts = vec![0.0; n];
        self.total_pulls = 0.0;
    }

    fn change_neighborhood(&mut self, _neighborhood: Neighborhood) {
        // Bandit manages its own set of neighborhoods
    }
}
