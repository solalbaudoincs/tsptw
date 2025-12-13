use crate::algorithms::{Metaheuristic, CompetitionType, CrossoverType};
use crate::shared::Instance;
use crate::eval::Evaluation;
use crate::neighborhood::{NeighborhoodType, LocalSearchType};

mod sa_factory;
pub use sa_factory::SAFactory;
pub use sa_factory::SAConfig;

mod aco_factory;
pub use aco_factory::ACOFactory;
pub use aco_factory::ACOConfig;

mod ga_factory;
pub use ga_factory::GAFactory;
pub use ga_factory::GAConfig;

mod vns_factory;
pub use vns_factory::VNSFactory;
pub use vns_factory::VNSConfig;

mod hc_factory;
pub use hc_factory::HCFactory;
pub use hc_factory::HCConfig;

pub enum AlgoConfig {
    SimulatedAnnealing(SAConfig),
    GeneticAlgorithm(GAConfig),
    AntColonyOptimization(ACOConfig),
    VariableNeighborhoodSearch(VNSConfig),
    HillClimbing(HCConfig),
}

impl AlgoConfig {
    pub fn into_factory(self) -> AlgoFactories {
        match self {
            AlgoConfig::SimulatedAnnealing(config) => AlgoFactories::SAFactory(SAFactory { config }),
            AlgoConfig::GeneticAlgorithm(config) => AlgoFactories::GAFactory(GAFactory { config }),
            AlgoConfig::AntColonyOptimization(config) => AlgoFactories::ACOFactory(ACOFactory { config }),
            AlgoConfig::VariableNeighborhoodSearch(config) => AlgoFactories::VNSFactory(VNSFactory { config }),
            AlgoConfig::HillClimbing(config) => AlgoFactories::HCFactory(HCFactory { config }),
        }
    }
}

#[derive(Clone)]
pub enum LocalSearchConfig {
    HillClimbing(HCConfig),
    SimulatedAnnealing(SAConfig),
}

pub enum AlgoFactories {
    SAFactory(SAFactory),
    GAFactory(GAFactory),
    ACOFactory(ACOFactory),
    VNSFactory(VNSFactory),
    HCFactory(HCFactory),
}

impl AlgoFactories {
    pub fn name(&self) -> &'static str {
        match self {
            AlgoFactories::SAFactory(_) => "Simulated Annealing",
            AlgoFactories::GAFactory(_) => "Genetic Algorithm",
            AlgoFactories::ACOFactory(_) => "Ant Colony Optimization",
            AlgoFactories::VNSFactory(_) => "Variable Neighborhood Search",
            AlgoFactories::HCFactory(_) => "Hill Climbing",
        }
    }
    pub fn build<Eval: Evaluation>(&self, instance: &Instance) -> Box<dyn Metaheuristic<Eval>> {
        match self {
            AlgoFactories::SAFactory(factory) => factory.build(instance),
            AlgoFactories::GAFactory(factory) => factory.build(instance),
            AlgoFactories::ACOFactory(factory) => factory.build(instance),
            AlgoFactories::VNSFactory(factory) => factory.build(instance),
            AlgoFactories::HCFactory(factory) => factory.build(instance),
        }
    }
}

pub trait Factory<Eval: Evaluation> {
    fn build(&self, instance: &Instance) -> Box<dyn Metaheuristic<Eval>>;
}

// Unified parameters struct with all possible algorithm parameters as Options
#[derive(Default, Clone)]
pub struct AlgoParams {
    // Simulated Annealing
    pub initial_temperature: Option<f32>,
    pub cooling_rate: Option<f32>,
    pub stopping_temperature: Option<f32>,
    pub acceptance_smoothing_factor: Option<f32>,
    pub initial_acceptance_rate: Option<f32>,
    pub delta_fitness_smoothing_factor: Option<f32>,
    
    // Genetic Algorithm
    pub crossover_rate: Option<f32>,
    pub crossover_type: Option<CrossoverType>,
    pub elitism_rate: Option<f32>,
    pub competition_participation_rate: Option<f32>,
    pub competition_type: Option<CompetitionType>,
    pub mutation_rate: Option<f32>,
    
    // Hill Climbing
    pub step: Option<usize>,
    
    // Ant Colony Optimization
    pub evaporation_rate: Option<f32>,
    pub alpha: Option<f32>,
    pub beta: Option<f32>,
    pub pheromone_deposit: Option<f32>,
    
    // VNS
    pub neighborhoods: Option<Vec<NeighborhoodType>>,
    pub local_search: Option<LocalSearchConfig>,
    
    // Common parameters
    pub neighborhood_type: Option<NeighborhoodType>,
    pub local_search_type: Option<LocalSearchType>,
    pub max_steps: Option<usize>,
    pub max_iter: Option<usize>,
    pub population_size: Option<usize>,
}

impl AlgoParams {
    pub fn new() -> Self {
        Self::default()
    }
    
    // Builder methods
    pub fn initial_temperature(mut self, val: f32) -> Self {
        self.initial_temperature = Some(val);
        self
    }
    
    pub fn cooling_rate(mut self, val: f32) -> Self {
        self.cooling_rate = Some(val);
        self
    }
    
    pub fn stopping_temperature(mut self, val: f32) -> Self {
        self.stopping_temperature = Some(val);
        self
    }
    
    pub fn acceptance_smoothing_factor(mut self, val: f32) -> Self {
        self.acceptance_smoothing_factor = Some(val);
        self
    }
    
    pub fn initial_acceptance_rate(mut self, val: f32) -> Self {
        self.initial_acceptance_rate = Some(val);
        self
    }
    
    pub fn delta_fitness_smoothing_factor(mut self, val: f32) -> Self {
        self.delta_fitness_smoothing_factor = Some(val);
        self
    }
    
    pub fn crossover_rate(mut self, val: f32) -> Self {
        self.crossover_rate = Some(val);
        self
    }
    
    pub fn crossover_type(mut self, val: CrossoverType) -> Self {
        self.crossover_type = Some(val);
        self
    }
    
    pub fn elitism_rate(mut self, val: f32) -> Self {
        self.elitism_rate = Some(val);
        self
    }
    
    pub fn competition_participation_rate(mut self, val: f32) -> Self {
        self.competition_participation_rate = Some(val);
        self
    }
    
    pub fn competition_type(mut self, val: CompetitionType) -> Self {
        self.competition_type = Some(val);
        self
    }
    
    pub fn mutation_rate(mut self, val: f32) -> Self {
        self.mutation_rate = Some(val);
        self
    }
    
    pub fn step(mut self, val: usize) -> Self {
        self.step = Some(val);
        self
    }
    
    pub fn evaporation_rate(mut self, val: f32) -> Self {
        self.evaporation_rate = Some(val);
        self
    }
    
    pub fn alpha(mut self, val: f32) -> Self {
        self.alpha = Some(val);
        self
    }
    
    pub fn beta(mut self, val: f32) -> Self {
        self.beta = Some(val);
        self
    }
    
    pub fn pheromone_deposit(mut self, val: f32) -> Self {
        self.pheromone_deposit = Some(val);
        self
    }
    
    pub fn neighborhoods(mut self, val: Vec<NeighborhoodType>) -> Self {
        self.neighborhoods = Some(val);
        self
    }
    
    pub fn local_search(mut self, val: LocalSearchConfig) -> Self {
        self.local_search = Some(val);
        self
    }
    
    pub fn neighborhood_type(mut self, val: NeighborhoodType) -> Self {
        self.neighborhood_type = Some(val);
        self
    }
    
    pub fn local_search_type(mut self, val: LocalSearchType) -> Self {
        self.local_search_type = Some(val);
        self
    }
    
    pub fn max_steps(mut self, val: usize) -> Self {
        self.max_steps = Some(val);
        self
    }
    
    pub fn max_iter(mut self, val: usize) -> Self {
        self.max_iter = Some(val);
        self
    }
    
    pub fn population_size(mut self, val: usize) -> Self {
        self.population_size = Some(val);
        self
    }
    
    // Build specific config from unified params
    pub fn build_sa_config(&self) -> Result<SAConfig, String> {
        Ok(SAConfig {
            initial_temperature: self.initial_temperature
                .ok_or("Missing parameter: initial_temperature for Simulated Annealing")?,
            cooling_rate: self.cooling_rate
                .ok_or("Missing parameter: cooling_rate for Simulated Annealing")?,
            stopping_temperature: self.stopping_temperature
                .ok_or("Missing parameter: stopping_temperature for Simulated Annealing")?,
            acceptance_smoothing_factor: self.acceptance_smoothing_factor
                .ok_or("Missing parameter: acceptance_smoothing_factor for Simulated Annealing")?,
            initial_acceptance_rate: self.initial_acceptance_rate
                .ok_or("Missing parameter: initial_acceptance_rate for Simulated Annealing")?,
            delta_fitness_smoothing_factor: self.delta_fitness_smoothing_factor
                .ok_or("Missing parameter: delta_fitness_smoothing_factor for Simulated Annealing")?,
            neighborhood_type: self.neighborhood_type
                .ok_or("Missing parameter: neighborhood_type for Simulated Annealing")?,
        })
    }
    
    pub fn build_ga_config(&self) -> Result<GAConfig, String> {
        Ok(GAConfig {
            crossover_rate: self.crossover_rate
                .ok_or("Missing parameter: crossover_rate for Genetic Algorithm")?,
            crossover_type: self.crossover_type
                .ok_or("Missing parameter: crossover_type for Genetic Algorithm")?,
            elitism_rate: self.elitism_rate
                .ok_or("Missing parameter: elitism_rate for Genetic Algorithm")?,
            competition_participation_rate: self.competition_participation_rate
                .ok_or("Missing parameter: competition_participation_rate for Genetic Algorithm")?,
            competition_type: self.competition_type
                .ok_or("Missing parameter: competition_type for Genetic Algorithm")?,
            max_iter: self.max_iter
                .ok_or("Missing parameter: max_iter for Genetic Algorithm")?,
            population_size: self.population_size
                .ok_or("Missing parameter: population_size for Genetic Algorithm")?,
            mutation_rate: self.mutation_rate
                .ok_or("Missing parameter: mutation_rate for Genetic Algorithm")?,
            local_search_type: self.local_search_type
                .ok_or("Missing parameter: local_search_type for Genetic Algorithm")?,
        })
    }
    
    pub fn build_hc_config(&self) -> Result<HCConfig, String> {
        Ok(HCConfig {
            step: self.step
                .ok_or("Missing parameter: step for Hill Climbing")?,
            max_steps: self.max_steps
                .ok_or("Missing parameter: max_steps for Hill Climbing")?,
            neighborhood_type: self.neighborhood_type
                .ok_or("Missing parameter: neighborhood_type for Hill Climbing")?,
        })
    }
    
    pub fn build_aco_config(&self) -> Result<ACOConfig, String> {
        Ok(ACOConfig {
            evaporation_rate: self.evaporation_rate
                .ok_or("Missing parameter: evaporation_rate for Ant Colony Optimization")?,
            alpha: self.alpha
                .ok_or("Missing parameter: alpha for Ant Colony Optimization")?,
            beta: self.beta
                .ok_or("Missing parameter: beta for Ant Colony Optimization")?,
            pheromone_deposit: self.pheromone_deposit
                .ok_or("Missing parameter: pheromone_deposit for Ant Colony Optimization")?,
            max_iter: self.max_iter
                .ok_or("Missing parameter: max_iter for Ant Colony Optimization")?,
            local_search_type: self.local_search_type
                .ok_or("Missing parameter: local_search_type for Ant Colony Optimization")?,
        })
    }
    
    pub fn build_vns_config(&self) -> Result<VNSConfig, String> {
        Ok(VNSConfig {
            neighborhoods: self.neighborhoods.clone()
                .ok_or("Missing parameter: neighborhoods for Variable Neighborhood Search")?,
            local_search: self.local_search.clone()
                .ok_or("Missing parameter: local_search for Variable Neighborhood Search")?,
        })
    }
    
    // Main builder that dispatches to the right config builder based on algo type
    pub fn build_config(&self, algo_type: AlgoType) -> Result<AlgoConfig, String> {
        match algo_type {
            AlgoType::SimulatedAnnealing => {
                Ok(AlgoConfig::SimulatedAnnealing(self.build_sa_config()?))
            },
            AlgoType::GeneticAlgorithm => {
                Ok(AlgoConfig::GeneticAlgorithm(self.build_ga_config()?))
            },
            AlgoType::HillClimbing => {
                Ok(AlgoConfig::HillClimbing(self.build_hc_config()?))
            },
            AlgoType::AntColonyOptimization => {
                Ok(AlgoConfig::AntColonyOptimization(self.build_aco_config()?))
            },
            AlgoType::VariableNeighborhoodSearch => {
                Ok(AlgoConfig::VariableNeighborhoodSearch(self.build_vns_config()?))
            },
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
pub enum AlgoType {
    SimulatedAnnealing,
    GeneticAlgorithm,
    HillClimbing,
    AntColonyOptimization,
    VariableNeighborhoodSearch,
}
