use std::cmp::Ordering;

use super::Metaheuristic;
use crate::neighbourhood::NeighbourhoodGenerator;
use crate::problem::{evaluation::Evaluation, instance::Instance, solution::Population};

pub struct HillClimbing {
    pub max_iterations: u32,
}

impl HillClimbing {
    pub fn new(max_iterations: u32) -> Self {
        Self { max_iterations }
    }
}

impl Metaheuristic for HillClimbing {
    fn step(
        &mut self,
        population: &mut Population,
        best: usize,
        instance: &Instance,
        neighbourhood: &NeighbourhoodGenerator,
        evaluation: &dyn Evaluation,
    ) {
        if population.is_empty() || best >= population.len() {
            return;
        }

        let mut neighbours = neighbourhood(&population[best]);
        for neighbour in neighbours.by_ref().take(self.max_iterations as usize) {
            if evaluation.compare(instance, &neighbour, &population[best]) == Ordering::Less {
                population[best] = neighbour;
                break;
            }
        }
    }
}