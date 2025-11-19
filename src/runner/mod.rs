use std::cmp::Ordering;

use crate::algorithms::Metaheuristic;
use crate::neighbourhood::NeighbourhoodGenerator;
use crate::problem::{
    evaluation::Evaluation,
    instance::Instance,
    solution::{Population, Solution},
};

pub struct RunConfig {
    pub max_iterations: u32,
}

pub fn run<T: Metaheuristic>(
    instance: &Instance,
    population: &mut Population,
    algorithm: &mut T,
    neighbourhood: NeighbourhoodGenerator,
    evaluation: &dyn Evaluation,
    config: &RunConfig,
) -> Option<usize> {
    let mut best = best_solution_index(population, instance, evaluation)?;

    for _ in 0..config.max_iterations {
        algorithm.step(population, best, instance, &neighbourhood, evaluation);
        best = best_solution_index(population, instance, evaluation)?;
    }

    Some(best)
}

fn best_solution_index(
    population: &[Solution],
    instance: &Instance,
    evaluation: &dyn Evaluation,
) -> Option<usize> {
    if population.is_empty() {
        return None;
    }

    let mut best = 0;
    for idx in 1..population.len() {
        if evaluation.compare(instance, &population[idx], &population[best]) == Ordering::Less {
            best = idx;
        }
    }
    Some(best)
}