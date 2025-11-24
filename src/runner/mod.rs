use std::cmp::Ordering;

use crate::algorithms::Metaheuristic;
use crate::eval::{Evaluation, utils};
use crate::shared::{Instance, Solution, Fitness};

pub struct RunConfig {
    pub max_iterations: u32,
}

pub fn run<Eval: Evaluation, Algo: Metaheuristic>(

    instance: &Instance,
    population: &mut Vec<Solution>,
    fitnesses: &mut Vec<Fitness>,
    algorithm: &mut Algo,
    evaluation: &Eval,
    config: &RunConfig,

) -> Option<usize> {
    for step in 0..config.max_iterations {
        if step % (config.max_iterations / 10) == 0 {
            let best = best_solution_index(population, instance, evaluation)?;
            let eval = utils::run_solution(instance, &population[best]);
            let (dist, viol) = (eval.total_distance, eval.violation_time);
            println!(
                "Iteration {}: Best solution: {}, distance: {}, violation: {}",
                step,
                population[best]
                    .iter()
                    .map(|n| n.to_string())
                    .collect::<Vec<String>>()
                    .join(" -> "),
                dist,
                viol
            );
        }
        algorithm.step(population, fitnesses, instance, evaluation);
    }
    let best = best_solution_index(population, instance, evaluation)?;
    let eval = utils::run_solution(instance, &population[best]);
    let (dist, viol) = (eval.total_distance, eval.violation_time);
    println!(
        "Iteration {}: Best solution: {}, distance: {}, violation: {}",
        config.max_iterations,
        population[best]
            .iter()
            .map(|n| n.to_string())
            .collect::<Vec<String>>()
            .join(" -> "),
        dist,
        viol
    );

    Some(best)
}

fn best_solution_index<T: Evaluation>(
    population: &[Solution],
    instance: &Instance,
    evaluation: &T,
) -> Option<usize> {
    if population.is_empty() {
        return None;
    }

    let mut best = 0;
    for idx in 1..population.len() {
        if evaluation.compare(instance, &population[idx], &population[best]) == Ordering::Greater {
            best = idx;
        }
    }
    Some(best)
}
