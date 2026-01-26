use rayon::prelude::*;
use std::sync::Arc;
use std::time::Instant;

use crate::algorithms::{SimulatedAnnealing, Metaheuristic};
use crate::eval::{Evaluation, Weighted};
use crate::initializer::{Initializer, SeededRandomInitializer};
use crate::io::io_instance::load_instance;
use crate::neighborhood::Neighborhood;
use crate::shared::Instance;

use super::types::*;

pub fn run_experiment(
    config: &ExperimentConfig,
    num_runs: usize,
    instance_path: &str,
) -> Result<ExperimentResults, String> {
    let (instance, _) = load_instance(instance_path)
        .map_err(|e| format!("Failed to load instance: {}", e))?;

    let instance_arc = Arc::new(instance.clone());

    let evaluation = Weighted {
        total_distance_weight: config.distance_weight,
        violation_time_weight: config.violation_time_weight,
        total_time_weight: config.total_time_weight,
        delay_weight: config.delay_weight,
    };

    let evaluation_arc = Arc::new(evaluation);

    let mut run_results = Vec::new();
    let mut convergence_data = Vec::new();
    let mut temp_trajectory = Vec::new();
    let mut bandit_stats = Vec::new();

    // Parallel run of experiments
    let results: Vec<_> = (0..num_runs)
        .into_par_iter()
        .map(|run_id| {
            run_single(
                run_id,
                config,
                instance_arc.as_ref(),
                evaluation_arc.as_ref(),
            )
        })
        .collect();

    for data in results {
        run_results.push(RunResult {
            run_id: data.run_id,
            config: config.config_id.clone(),
            best_fitness: data.best_fitness,
            best_step: data.best_step,
            final_fitness: data.final_fitness,
            final_temperature: data.final_temperature,
            violations: data.violations,
            feasible: data.feasible,
            total_distance: data.total_distance,
            total_time: data.total_time,
            execution_time_ms: data.execution_time_ms,
        });

        convergence_data.push(data.snapshots);

        if !data.temp_trajectory.is_empty() {
            temp_trajectory.extend(data.temp_trajectory);
        }

        if !data.bandit_stats.is_empty() {
            bandit_stats.extend(data.bandit_stats);
        }
    }

    Ok(ExperimentResults {
        config: config.clone(),
        runs: run_results,
        convergence_data,
        temp_trajectory: if temp_trajectory.is_empty() {
            None
        } else {
            Some(temp_trajectory)
        },
        bandit_stats: if bandit_stats.is_empty() {
            None
        } else {
            Some(bandit_stats)
        },
    })
}

pub fn run_single(
    run_id: usize,
    config: &ExperimentConfig,
    instance: &Instance,
    evaluation: &Weighted,
) -> SingleRunData {
    let start_time = Instant::now();

    // Derive seeds from run_id
    let init_seed = (run_id as u64) * 3 + 1;
    let neighborhood_seed = (run_id as u64) * 3 + 2;
    let sa_seed = (run_id as u64) * 3;

    // Initialize solution
    let mut initializer = SeededRandomInitializer::new(init_seed);
    let mut solution = initializer.initialize(instance);
    let mut fitness = evaluation.score(instance, &solution);

    // Create neighborhood with seed
    let neighborhood = Neighborhood::from_type_with_seed(config.neighborhood, instance, neighborhood_seed);

    // Create SA instance
    let mut sa = if config.warmup {
        SimulatedAnnealing::new_with_seed(
            config.initial_temperature,
            config.cooling_rate,
            config.stopping_temperature,
            config.acceptance_smoothing_factor,
            config.initial_acceptance_rate,
            config.delta_fitness_smoothing_factor,
            neighborhood,
            config.backtrack_interval,
            sa_seed,
            config.max_warmup_steps,
        )
    } else {
        SimulatedAnnealing::new_cold_start_with_seed(
            config.initial_temperature,
            config.cooling_rate,
            config.stopping_temperature,
            config.acceptance_smoothing_factor,
            config.initial_acceptance_rate,
            config.delta_fitness_smoothing_factor,
            neighborhood,
            config.backtrack_interval,
            sa_seed,
            config.max_warmup_steps,
        )
    };

    let mut best_fitness = fitness;
    let mut best_step = 0;
    let mut snapshots = Vec::new();
    let mut temp_trajectory = Vec::new();
    let mut bandit_stats_vec = Vec::new();

    // Main loop
    let mut population = vec![solution.clone()];
    let mut fitnesses = vec![fitness];

    for step in 0..config.max_steps {
        sa.step(&mut population, &mut fitnesses, instance, evaluation);

        solution = population[0].clone();
        fitness = fitnesses[0];

        if fitness < best_fitness {
            best_fitness = fitness;
            best_step = step;
        }

        // Sample convergence data
        if step % config.sampling_interval == 0 || step == config.max_steps - 1 {
            let result = crate::eval::utils::run_solution(instance, &solution);
            snapshots.push(StepSnapshot {
                step,
                fitness,
                temperature: sa.temperature(),
                feasible: result.violation_time < 0.01,
                violations: if result.violation_time > 0.0 { 1 } else { 0 },
                distance: result.total_distance,
                time: result.total_time,
                delay: result.delay,
            });
        }

        // Record temperature trajectory at sampling interval
        if step % config.sampling_interval == 0 {
            temp_trajectory.push(TempTrajectoryRow {
                run_id,
                step,
                temperature: sa.temperature(),
            });
        }

        // Record bandit stats at sampling interval
        if let Some(stats) = sa.get_bandit_stats() {
            if step % config.sampling_interval == 0 || step == config.max_steps - 1 {
                bandit_stats_vec.push(BanditStatsRow {
                    run_id,
                    step,
                    swap_selections: stats.swap_selections,
                    twoopt_selections: stats.twoopt_selections,
                    swap_avg_reward: stats.swap_avg_reward,
                    twoopt_avg_reward: stats.twoopt_avg_reward,
                });
            }
        }
    }

    // Record final state (step = max_steps, after all iterations complete)
    let final_temperature = sa.temperature();
    temp_trajectory.push(TempTrajectoryRow {
        run_id,
        step: config.max_steps,
        temperature: final_temperature,
    });

    let final_result = crate::eval::utils::run_solution(instance, &solution);
    let execution_time = start_time.elapsed().as_millis();

    SingleRunData {
        run_id,
        best_fitness,
        best_step,
        final_fitness: fitness,
        final_temperature,
        violations: if final_result.violation_time > 0.0 { 1 } else { 0 },
        feasible: final_result.violation_time < 0.01,
        total_distance: final_result.total_distance,
        total_time: final_result.total_time,
        execution_time_ms: execution_time,
        snapshots,
        temp_trajectory,
        bandit_stats: bandit_stats_vec,
    }
}

pub fn aggregate_convergence(results: &ExperimentResults) -> Vec<ConvergenceRow> {
    let mut aggregated = std::collections::HashMap::new();

    // Collect all data by step
    for run_data in &results.convergence_data {
        for snapshot in run_data {
            let entry = aggregated
                .entry(snapshot.step)
                .or_insert_with(Vec::new);
            entry.push(snapshot.fitness);
        }
    }

    // Sort by step and compute statistics
    let mut rows = Vec::new();
    for step in 0..results.config.max_steps {
        if let Some(fitnesses) = aggregated.get(&step) {
            let mut sorted = fitnesses.clone();
            sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());

            let len = sorted.len() as f32;
            let mean = sorted.iter().sum::<f32>() / len;
            let median = sorted[(sorted.len() / 2) as usize];
            let q1_idx = (sorted.len() / 4) as usize;
            let q3_idx = (3 * sorted.len() / 4) as usize;
            let p10_idx = ((sorted.len() as f32 * 0.1) as usize).max(0);
            let p90_idx = ((sorted.len() as f32 * 0.9) as usize).min(sorted.len() - 1);

            let q1 = sorted[q1_idx];
            let q3 = sorted[q3_idx];
            let p10 = sorted[p10_idx];
            let p90 = sorted[p90_idx];

            let feasible_rate = results.convergence_data
                .iter()
                .filter_map(|run| {
                    run.iter()
                        .find(|s| s.step == step)
                        .map(|s| if s.feasible { 1.0 } else { 0.0 })
                })
                .sum::<f32>() / results.convergence_data.len() as f32;

            rows.push(ConvergenceRow {
                step,
                mean_fitness: mean,
                median_fitness: median,
                q1_fitness: q1,
                q3_fitness: q3,
                p10_fitness: p10,
                p90_fitness: p90,
                feasible_rate,
            });
        }
    }

    rows
}

pub fn compute_warmup_stats(temps: &[f32]) -> WarmupStats {
    let mut sorted = temps.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let len = sorted.len() as f32;
    let mean = sorted.iter().sum::<f32>() / len;
    let variance = sorted.iter()
        .map(|t| (t - mean).powi(2))
        .sum::<f32>() / len;
    let std = variance.sqrt();

    let median = sorted[(sorted.len() / 2) as usize];
    let q1_idx = (sorted.len() / 4) as usize;
    let q3_idx = (3 * sorted.len() / 4) as usize;
    let q1 = sorted[q1_idx];
    let q3 = sorted[q3_idx];
    let min = sorted[0];
    let max = sorted[sorted.len() - 1];

    WarmupStats {
        mean,
        std,
        median,
        q1,
        q3,
        min,
        max,
    }
}
