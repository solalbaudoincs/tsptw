use crate::algorithms::{GeneticAlgorithm, Metaheuristic, CompetitionType, CrossoverType};
use crate::shared::Instance;
use crate::eval::Evaluation;
use crate::neighborhood::{LocalSearchType, LocalSearchImpl};
use super::Factory;

pub struct GAConfig {
    pub crossover_rate: f32,
    pub crossover_type: CrossoverType,
    pub elitism_rate: f32,
    pub competition_participation_rate: f32,
    pub competition_type: CompetitionType,
    pub max_iter: usize,
    pub population_size: usize,
    pub mutation_rate: f32,
    pub local_search_type: LocalSearchType,
}

pub struct GAFactory {
    pub config: GAConfig,
}

impl<Eval: Evaluation> Factory<Eval> for GAFactory {
    fn build(&self, instance: &Instance) -> Box<dyn Metaheuristic<Eval>> {
        let local_search = LocalSearchImpl::from_type(self.config.local_search_type, instance);
        let ga = GeneticAlgorithm::new(
            instance,
            self.config.crossover_rate,
            self.config.crossover_type,
            self.config.elitism_rate,
            self.config.competition_participation_rate,
            self.config.competition_type,
            self.config.max_iter,
            self.config.population_size,
            self.config.mutation_rate,
            local_search,
        );
        Box::new(ga)
    }
}
