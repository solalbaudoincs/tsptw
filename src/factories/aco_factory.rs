use crate::algorithms::{ACO, Metaheuristic};
use crate::shared::Instance;
use crate::eval::Evaluation;
use crate::neighborhood::{LocalSearchType, LocalSearchImpl};
use super::Factory;

pub struct ACOConfig {
    pub evaporation_rate: f32,
    pub alpha: f32,
    pub beta: f32,
    pub pheromone_deposit: f32,
    pub max_iter: usize,
    pub local_search_type: LocalSearchType,
}

pub struct ACOFactory {
    pub config: ACOConfig,
}

impl<Eval: Evaluation> Factory<Eval> for ACOFactory {
    fn build(&self, instance: &Instance) -> Box<dyn Metaheuristic<Eval>> {
        let local_search = LocalSearchImpl::from_type(self.config.local_search_type, instance);
        let aco = ACO::new(
            instance,
            self.config.evaporation_rate,
            self.config.alpha,
            self.config.beta,
            self.config.pheromone_deposit,
            self.config.max_iter,
            local_search,
        );
        Box::new(aco)
    }
}
