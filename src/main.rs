mod problem;
mod neighbourhood;
mod algorithms;
mod runner;

use algorithms::HillClimbing;
use neighbourhood::swap_neighbourhood;
use problem::{
    evaluation::Lexicographic,
    instance::Instance,
    solution::{Population, Solution},
};
use runner::{run, RunConfig};

fn main() {
    let instance = problem::instance::io::load_instance("data/inst1");
    let mut population = build_initial_population(&instance);

    let evaluation = Lexicographic::new(true);
    let mut algorithm = HillClimbing::new(100);
    let config = RunConfig { max_iterations: 25 };

    match run(
        &instance,
        &mut population,
        &mut algorithm,
        swap_neighbourhood,
        &evaluation,
        &config,
    ) {
        Some(best_idx) => {
            let best = &population[best_idx];
            let preview: Vec<u32> = best.sol_list.iter().cloned().take(5).collect();
            println!(
                "Runner finished. Best solution visits {} nodes. Prefix: {:?}",
                best.sol_list.len(), preview
            );
        }
        None => eprintln!("Runner aborted: population is empty."),
    }
}

fn build_initial_population(instance: &Instance) -> Population {
    let mut route: Vec<u32> = instance.graph.keys().copied().collect();
    route.sort_unstable();

    vec![Solution {
        sol_list: route,
        sol_val: None,
    }]
}
