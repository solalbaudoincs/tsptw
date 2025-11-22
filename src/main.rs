mod algorithms;
mod neighbourhood;
mod problem;
mod runner;
mod shared_types;

use std::f64::consts::E;

use algorithms::{HillClimbing};
use neighbourhood::{TwoOpt, Swap};
use problem::{
    evaluation::{Lexicographic},
    instance::Instance,
    solution::{Population, Solution},
};
use runner::{RunConfig, run};

use problem::evaluation::utils::run_solution;

use crate::problem::evaluation::Weighted;


const CHALLENGE_PATHS: [&str; 3] = [
    "data/inst1",
    "data/inst2",
    "data/inst3",
];

const EXAMPLE_SOLUTION_PATHS: [&str; 3] = [
    "data/inst1.sol",
    "data/inst2.sol",
    "data/inst3.sol",
];

const CHALLENGE_NB : usize = 1; 

fn main() {
    let instance = problem::instance::io::load_instance(CHALLENGE_PATHS[CHALLENGE_NB-1]);
    let evaluation = Weighted{violation_coefficient : 100.0};
    let config = RunConfig {
        max_iterations: 10000,
    };

    // let mut sa_population = build_initial_population(&instance);
    // let neighbourhood = TwoOpt;
    // let sa_init_temp = SimulatedAnnealing::estimate_initial_temperature(
    //     &instance,
    //     &sa_population[0],
    //     &neighbourhood,
    //     10000,
    //     0.01,
    //     &evaluation,
    // );
    // println!("Estimated SA temperature: {}", sa_init_temp);
    // let sa_min_temp = sa_init_temp * 0.0005;
    // let mut sa_algorithm = SimulatedAnnealing::new(sa_init_temp, 0.99, sa_min_temp);
    // let sa_best = run(
    //     &instance,
    //     &mut sa_population,
    //     &mut sa_algorithm,
    //     &neighbourhood,
    //     &evaluation,
    //     &config,
    // );
    // report_result("Simulated Annealing", &instance, &sa_population, sa_best);

    let example_solution = problem::solution::io::load_solution(EXAMPLE_SOLUTION_PATHS[CHALLENGE_NB-1]);
    match example_solution {
        Ok(sol) => {
            let (dist, viol) = run_solution(&instance, &(sol.0));
            println!("Example solution performance: total_distance={}, total_violation={}", dist, viol);
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
            let (total_distance, total_violation) = run_solution(instance, best);
            println!(
                "{} performance: total_distance={}, total_violation={} ",
                name, total_distance, total_violation
            );
        }
        None => eprintln!("{} aborted: population is empty.", name),
    }
}
