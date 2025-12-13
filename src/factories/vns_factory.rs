use crate::algorithms::{VNS, Metaheuristic, SimulatedAnnealing, HillClimbing};
use crate::neighborhood::{Neighborhood, NeighborhoodType};
use crate::shared::Instance;
use crate::eval::Evaluation;
use super::{Factory, LocalSearchConfig};

pub struct VNSConfig {
    pub neighborhoods: Vec<NeighborhoodType>,
    pub local_search: LocalSearchConfig,
}

pub struct VNSFactory {
    pub config: VNSConfig,
}

impl<Eval: Evaluation> Factory<Eval> for VNSFactory {
    fn build(&self, instance: &Instance) -> Box<dyn Metaheuristic<Eval>> {
        let neighborhoods: Vec<Neighborhood> = self.config.neighborhoods.iter()
            .map(|&t| Neighborhood::from_type(t, instance))
            .collect();

        match &self.config.local_search {
            LocalSearchConfig::HillClimbing(hc_config) => {
                 let neighborhood = Neighborhood::from_type(hc_config.neighborhood_type, instance);
                 let hc = HillClimbing::new(
                    hc_config.step,
                    hc_config.max_steps,
                    neighborhood,
                );
                let vns = VNS::new(neighborhoods, hc);
                Box::new(vns)
            },
            LocalSearchConfig::SimulatedAnnealing(sa_config) => {
                let neighborhood = Neighborhood::from_type(sa_config.neighborhood_type, instance);
                let sa = SimulatedAnnealing::new(
                    sa_config.initial_temperature,
                    sa_config.cooling_rate,
                    sa_config.stopping_temperature,
                    sa_config.acceptance_smoothing_factor,
                    sa_config.initial_acceptance_rate,
                    sa_config.delta_fitness_smoothing_factor,
                    neighborhood,
                );
                let vns = VNS::new(neighborhoods, sa);
                Box::new(vns)
            }
        }
    }
}
