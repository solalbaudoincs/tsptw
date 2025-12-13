use crate::algorithms::{HillClimbing, Metaheuristic};
use crate::neighborhood::{Neighborhood, NeighborhoodType};
use crate::shared::Instance;
use crate::eval::Evaluation;
use super::Factory;

#[derive(Clone)]
pub struct HCConfig {
    pub step: usize,
    pub max_steps: usize,
    pub neighborhood_type: NeighborhoodType,
}

#[derive(Clone)]
pub struct HCFactory {
    pub config: HCConfig,
}

impl<Eval: Evaluation> Factory<Eval> for HCFactory {
    fn build(&self, instance: &Instance) -> Box<dyn Metaheuristic<Eval>> {
        let neighborhood = Neighborhood::from_type(self.config.neighborhood_type, instance);
        let hc = HillClimbing::new(
            self.config.step,
            self.config.max_steps,
            neighborhood,
        );
        Box::new(hc)
    }
}
