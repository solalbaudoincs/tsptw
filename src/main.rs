
use mh_tsptw::algorithms::SimulatedAnnealing;
use mh_tsptw::neighbourhood::{Swap, TwoOpt, NeighborFnMixer};
use mh_tsptw::problem::{
    Evaluation,
    evaluation::{Weighted},
    instance::{Instance, io::load_instance},
    solution::{Population, io::load_solution},
};
use mh_tsptw::runner::{RunConfig, run};

use mh_tsptw::problem::evaluation::utils::run_solution;

const CHALLENGE_PATHS: [&str; 3] = ["data/inst1", "data/inst2", "data/inst3"];

const EXAMPLE_SOLUTION_PATHS: [&str; 3] = ["data/inst1.sol", "data/inst2.sol", "data/inst3.sol"];

const CHALLENGE_NB: usize = 1;

fn main() {
    let instance = load_instance(CHALLENGE_PATHS[CHALLENGE_NB - 1]);
    let evaluation = Weighted {
        violation_coefficient: 1000000.0,
    };
    let mut neighbourhood = NeighborFnMixer::new(vec![
        Box::new(Swap::new()),
        Box::new(TwoOpt::new()),
    ], vec![0.5, 0.5]);


    let config = RunConfig {
        max_iterations: 100000,
    };

    let mut sa_population = build_initial_population(&instance);
    let mut fitnesss: Vec<f64> = sa_population
        .iter()
        .map(|sol| evaluation.score(&instance, sol))
        .collect();

    let sa_init_temp = 500.0;
    println!("Estimated SA temperature: {}", sa_init_temp);
    let sa_min_temp = sa_init_temp * 0.0005;
    let mut sa_algorithm = SimulatedAnnealing::new(sa_init_temp, 0.95, sa_min_temp);

    let sa_best = run(
        &instance,
        &mut sa_population,
        &mut fitnesss,
        &mut sa_algorithm,
        &mut neighbourhood,
        &evaluation,
        &config,
    );
    report_result("Simulated Annealing", &instance, &sa_population, sa_best);

    let example_solution = load_solution(EXAMPLE_SOLUTION_PATHS[CHALLENGE_NB - 1]);
    match example_solution {
        Ok(sol) => {
            let (dist, viol) = run_solution(&instance, &(sol.0));
            println!(
                "Example solution performance: total_distance={}, total_violation={}",
                dist, viol
            );
        }
        Err(e) => println!("Failed to load example solution: {}", e),
    }
}

fn build_initial_population(instance: &Instance) -> Population {
    let route: Vec<u32> = instance
        .graph
        .iter()
        .enumerate()
        .skip(1) // skip depot
        .map(|(idx, _)| idx as u32)
        .collect();
    let mut greedy_route = route.clone();

    vec![greedy_route]
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
            let (total_distance, total_violation) = run_solution(instance, best);
            println!(
                "{} performance: total_distance={}, total_violation={} ",
                name, total_distance, total_violation
            );
        }
        None => eprintln!("{} aborted: population is empty.", name),
    }
}
