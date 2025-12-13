use crate::algorithms::{SimulatedAnnealing, Metaheuristic};
use crate::neighborhood::{Neighborhood, NeighborhoodType};
use crate::shared::{Instance};
use crate::eval::Evaluation;

use super::Factory;

#[derive(Clone)]
pub struct SAConfig {
    pub initial_temperature: f32,
    pub cooling_rate: f32,
    pub stopping_temperature: f32,
    pub acceptance_smoothing_factor: f32,
    pub initial_acceptance_rate: f32,
    pub delta_fitness_smoothing_factor: f32,
    pub neighborhood_type: NeighborhoodType,
}

#[derive(Clone)]
pub struct SAFactory {
    pub config: SAConfig,
}

impl<Eval: Evaluation> Factory<Eval> for SAFactory {
    fn build(&self, instance: &Instance) -> Box<dyn Metaheuristic<Eval>> {
        let neighborhood = Neighborhood::from_type(self.config.neighborhood_type, instance);
        let sa  = SimulatedAnnealing::new(
            self.config.initial_temperature,
            self.config.cooling_rate,
            self.config.stopping_temperature,
            self.config.acceptance_smoothing_factor,
            self.config.initial_acceptance_rate,
            self.config.delta_fitness_smoothing_factor,
            neighborhood,
        );
        Box::new(sa)
    }
}