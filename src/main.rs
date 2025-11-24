
use clap::Parser;
use mh_tsptw::algorithms::SimulatedAnnealing;
use mh_tsptw::neighborhood::Swap;
use mh_tsptw::eval::{Evaluation, Weighted, utils::run_solution};
use mh_tsptw::shared::{Instance, Solution};
use mh_tsptw::io::{io_instance::load_instance, io_solution::load_solution};
use mh_tsptw::runner::{RunConfig, run};

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
        ).unwrap();
        return;
    }

    let (instance, _graph_instance) = load_instance(CHALLENGE_PATHS[CHALLENGE_NB - 1]).unwrap();
    let evaluation = Weighted {
        violation_coefficient: 10000000.0f32,
    };
    let mut neighborhood = Swap::new();


    let config = RunConfig {
        max_iterations: 1000000,
    };

    let mut sa_population = build_initial_population(&instance);
    let mut fitnesss: Vec<f32> = sa_population
        .iter()
        .map(|sol| evaluation.score(&instance, sol))
        .collect();

    let mut temp_sa = SimulatedAnnealing::new(1000.0, 0.995, 0.001, &instance);
    let sa_init_temp = temp_sa.estimate_initial_temperature(
        &instance,
        &evaluation,
        &mut neighborhood,
        10000,
        0.9f32,
    );


    println!("Estimated SA temperature: {}", sa_init_temp);
    let sa_min_temp = sa_init_temp * 0.0005f32;
    let mut sa_algorithm = SimulatedAnnealing::new(sa_init_temp, 0.995f32, sa_min_temp, &instance);

    let sa_best = run(
        &instance,
        &mut sa_population,
        &mut fitnesss,
        &mut sa_algorithm,
        &mut neighborhood,
        &evaluation,
        &config,
    );
    report_result("Simulated Annealing", &instance, &sa_population, sa_best);

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
