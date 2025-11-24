// use crate::algorithms::{GeneticAlgorithm, SimulatedAnnealing, CrossoverType, CompetitionType, Metaheuristic};
// use crate::shared::{Instance, Solution, Fitness};
// use crate::eval::Evaluation;

// use crate::solver::Solver;

// use rand::Rng;
// use rand::rngs::StdRng;
// use rand::SeedableRng;


// struct GeneticSimulatedAnnealing {
//     ga: GeneticAlgorithm,
//     sa: SimulatedAnnealing,
//     rng: StdRng,
//     nb_ga_steps: usize,
//     nb_sa_steps: usize,
//  }
 
//  impl GeneticSimulatedAnnealing {

//         /// Creates a new GeneticSimulatedAnnealing solver with the given parameters.
//         /// 
//         /// 
//         /// Solver Parameters:
//         /// - `nb_ga_steps`: The number of steps to run the genetic algorithm.
//         /// - `nb_sa_steps`: The number of steps to run the simulated annealing algorithm.
//         /// 
//         /// Genetic Algorithm Parameters:
//         /// - `instance`: The problem instance to solve.
//         /// - `crossover_type`: The type of crossover to use in the genetic algorithm.
//         /// - `elitism_rate`: The rate of elitism in the genetic algorithm.
//         /// - `crossover_rate`: The crossover rate in the genetic algorithm.
//         /// - `mutation_rate`: The mutation rate in the genetic algorithm.
//         /// - `competition_participation_rate`: The competition participation rate in the genetic algorithm.
//         /// 
//         /// 
//         /// Simulated Annealing Parameters:
//         /// - `initial_temperature`: The initial temperature for simulated annealing.
//         /// - `two_opt_rate`: The two-opt rate for simulated annealing.
//         /// - `cooling_rate`: The cooling rate for simulated annealing.
//         /// - `stopping_temperature`: The stopping temperature for simulated annealing.
//         pub fn new(

//             nb_ga_steps: usize,
//             nb_sa_steps: usize,

//             instance: &Instance,
//             crossover_type: CrossoverType,
//             competition_type: CompetitionType,
//             population_size: usize,

//             elitism_rate: f32,
//             crossover_rate: f32,
//             mutation_rate: f32,
//             competition_participation_rate: f32,
//             initial_temperature: f32,
//             two_opt_rate: f32,
//             cooling_rate: f32,
//             stopping_temperature: f32,
//         ) -> Self {
//             GeneticSimulatedAnnealing {
//                 nb_ga_steps,
//                 nb_sa_steps,
//                 rng: StdRng::from_os_rng(),

//                 ga: GeneticAlgorithm::new(
//                     instance,
//                     crossover_rate,
//                     crossover_type,
//                     elitism_rate,
//                     competition_participation_rate,
//                     competition_type,
//                     population_size,
//                 ),

//                 sa: SimulatedAnnealing::new(
//                     initial_temperature,
//                     two_opt_rate,
//                     cooling_rate, // cooling rate
//                     stopping_temperature,  // stopping temperature
//                     instance,
//                 ),
//             }
//         }
//     }

// impl Metaheuristic for GeneticSimulatedAnnealing {
//         fn step<Eval: Evaluation>(
//             &mut self,
//             population: &mut [Solution],
//             fitnesses: &mut [Fitness],
//             instance: &Instance,
//             evaluation: &Eval,
//         ) {
//             // First apply the genetic algorithm step
//             for _ in 0..self.nb_ga_steps {
//                 self.ga.step(population, fitnesses, instance, evaluation);
//             }
//             // Then apply the simulated annealing step
//             for _ in 0..self.nb_sa_steps {
//                 self.sa.step(population, fitnesses, instance, evaluation);
//             }
//         }

//         fn get_metrics(&self) -> std::collections::HashMap<String, f32> {
//             let mut metrics = self.ga.get_metrics();
//             metrics.extend(self.sa.get_metrics());
//             metrics
//         }

//         fn get_metric_names(&self) -> Vec<String> {
//             let mut names = self.ga.get_metric_names();
//             names.extend(self.sa.get_metric_names());
//             names
//         }
//     }

// impl Solver for GeneticSimulatedAnnealing {
//     fn solve<E: Evaluation>(
//         &mut self,
//         max_iterations: usize,
//         instance: &Instance,
//         evaluation: &E,
//     ) -> () {
//         let population_size = self.ga.population_size;
//         let mut population: Vec<Solution> = vec![vec![0; instance.size()]; population_size];
//         let mut fitnesses: Vec<Fitness> = vec![f32::MAX; population_size];

//         // Initialize population randomly
//         for i in 0..population_size {
//             for j in 0..instance.size() {
//                 population[i][j] = j as u32;
//             }
//             rand::thread_rng().shuffle(&mut population[i]);
//             fitnesses[i] = evaluation.score(instance, &population[i]);
//         }

//         for _ in 0..max_iterations {
//             self.step(&mut population, &mut fitnesses, instance, evaluation);
//         }
//     }
// }