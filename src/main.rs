use clap::Parser;
use mh_tsptw::algorithms::{CompetitionType, CrossoverType};
use mh_tsptw::eval::{Evaluation, Weighted, utils::run_solution};
use mh_tsptw::factories::{AlgoParams, AlgoType};
use mh_tsptw::initializer::{Initializer, RandomInitializer};
use mh_tsptw::io::{io_instance::load_instance, io_solution::load_solution};
use mh_tsptw::neighborhood::{NeighborhoodType, LocalSearchType};
use mh_tsptw::shared::{Instance, Solution};

type Population = Vec<Solution>;

const CHALLENGE_PATHS: [&str; 3] = ["data/inst1", "data/inst2", "data/inst3"];

const EXAMPLE_SOLUTION_PATHS: [&str; 3] = ["data/inst1.sol", "data/inst2.sol", "data/inst3.sol"];

const CHALLENGE_NB: usize = 1;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Run in GUI mode
    #[arg(long, default_value_t = false)]
    gui: bool,
}

fn main() {
    let args = Args::parse();

    if args.gui {
        let native_options = eframe::NativeOptions::default();
        eframe::run_native(
            "Metaheuristics TSPTW",
            native_options,
            Box::new(|cc| Ok(Box::new(mh_tsptw::gui::app::TspApp::new(cc)))),
        )
        .unwrap();
        return;
    }

    let (instance, _graph_instance) = load_instance(CHALLENGE_PATHS[CHALLENGE_NB - 1]).unwrap();
    let evaluation = Weighted {
        total_distance_weight: 1.0,
        violation_time_weight: 10000.0,
        total_time_weight: 0.0,
        delay_weight: 0.0,
    };

    // Simulated Annealing with factory pattern
    let sa_params = AlgoParams::new()
        .initial_temperature(1000.0)
        .cooling_rate(0.995)
        .stopping_temperature(0.001)
        .acceptance_smoothing_factor(0.9)
        .initial_acceptance_rate(0.8)
        .delta_fitness_smoothing_factor(0.9)
        .neighborhood_type(NeighborhoodType::Swap);

    let sa_config = sa_params.build_config(AlgoType::SimulatedAnnealing).unwrap();
    let sa_factory = sa_config.into_factory();
    let mut sa_algorithm = sa_factory.build(&instance);

    let max_iterations = 10000;
    let mut sa_population = build_initial_population(&instance);

    for _ in 0..max_iterations {
        let mut fitnesss: Vec<f32> = sa_population
            .iter()
            .map(|sol| evaluation.score(&instance, sol))
            .collect();
        sa_algorithm.step(&mut sa_population, &mut fitnesss, &instance, &evaluation);
    }

    let sa_best = sa_population
        .iter()
        .enumerate()
        .min_by(|(_, a), (_, b)| {
            evaluation
                .score(&instance, a)
                .partial_cmp(&evaluation.score(&instance, b))
                .unwrap()
        })
        .map(|(idx, _)| idx);
    report_result("Simulated Annealing", &instance, &sa_population, sa_best);

    // Genetic Algorithm with factory pattern
    let ga_population_size = 100;
    let ga_params = AlgoParams::new()
        .crossover_rate(0.8)
        .crossover_type(CrossoverType::PMX)
        .elitism_rate(0.1)
        .competition_participation_rate(0.5)
        .competition_type(CompetitionType::Tournament)
        .population_size(ga_population_size)
        .max_iter(1000)
        .mutation_rate(0.1)
        .local_search_type(LocalSearchType::Swap);

    let ga_config = ga_params.build_config(AlgoType::GeneticAlgorithm).unwrap();
    let ga_factory = ga_config.into_factory();
    let mut ga_algorithm = ga_factory.build(&instance);

    let mut random_init = RandomInitializer;
    let mut ga_population: Vec<Solution> = (0..ga_population_size)
        .map(|_| random_init.initialize(&instance))
        .collect();

    let ga_max_iterations = 1000;
    for _ in 0..ga_max_iterations {
        let mut ga_fitnesss: Vec<f32> = ga_population
            .iter()
            .map(|sol| evaluation.score(&instance, sol))
            .collect();
        ga_algorithm.step(&mut ga_population, &mut ga_fitnesss, &instance, &evaluation);
    }

    let ga_best = ga_population
        .iter()
        .enumerate()
        .min_by(|(_, a), (_, b)| {
            evaluation
                .score(&instance, a)
                .partial_cmp(&evaluation.score(&instance, b))
                .unwrap()
        })
        .map(|(idx, _)| idx);
    report_result("Genetic Algorithm", &instance, &ga_population, ga_best);

    let example_solution = load_solution(&EXAMPLE_SOLUTION_PATHS[CHALLENGE_NB - 1].to_string());
    match example_solution {
        Ok(sol) => {
            let eval_result = run_solution(&instance, &sol.path);
            println!(
                "Example solution performance: total_distance={}, total_violation={}",
                eval_result.total_distance, eval_result.violation_time
            );
        }
        Err(e) => println!("Failed to load example solution: {}", e),
    }
}

fn build_initial_population(instance: &Instance) -> Population {
    let route: Vec<u32> = (0..instance.size() as u32).collect();
    vec![route]
}

fn report_result(
    name: &str,
    instance: &Instance,
    population: &Population,
    best_idx: Option<usize>,
) {
    match best_idx {
        Some(idx) => {
            let best = &population[idx];
            println!(
                "{} finished. Best solution visits {} nodes. Full solution {}",
                name,
                best.len(),
                best.iter()
                    .map(|n| n.to_string())
                    .collect::<Vec<String>>()
                    .join(" -> ")
            );
            let eval_result = run_solution(instance, best);
            println!(
                "{} performance: total_distance={}, total_violation={} ",
                name, eval_result.total_distance, eval_result.violation_time
            );
        }
        None => eprintln!("{} aborted: population is empty.", name),
    }
}
