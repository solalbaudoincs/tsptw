use clap::{Parser, Subcommand};
use mh_tsptw::algorithms::{CompetitionType, CrossoverType};
use mh_tsptw::eval::{Evaluation, Weighted, utils::run_solution};
use mh_tsptw::factories::{AlgoParams, AlgoType};
use mh_tsptw::initializer::{Initializer, RandomInitializer};
use mh_tsptw::io::{io_instance::load_instance, io_solution::load_solution};
use mh_tsptw::neighborhood::{NeighborhoodType, LocalSearchType};
use mh_tsptw::shared::{Instance, Solution};
use mh_tsptw::experiments;

type Population = Vec<Solution>;

const CHALLENGE_PATHS: [&str; 4] = ["data/inst1", "data/inst2", "data/inst3", "data/inst_concours"];

const EXAMPLE_SOLUTION_PATHS: [&str; 3] = ["data/inst1.sol", "data/inst2.sol", "data/inst3.sol"];

const CHALLENGE_NB: usize = 1;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Run in GUI mode (default if no command)
    #[arg(long, default_value_t = false)]
    gui: bool,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Run batch experiments
    Experiment {
        /// Phase to run: phase0, phase1, phase2, phase3, phase4, or all
        #[arg(long)]
        phase: String,

        /// Instances to run: all, inst1, inst2, inst3, inst_concours
        #[arg(long, default_value = "all")]
        instances: String,

        /// Number of runs per configuration
        #[arg(long, default_value = "200")]
        runs: usize,

        /// Output directory for results
        #[arg(long, default_value = "results")]
        output_dir: String,

        /// Path to calibrated temperatures JSON file (for phases 1-4)
        #[arg(long)]
        calibrated_temps: Option<String>,
    },
}

fn main() {
    let args = Args::parse();

    match args.command {
        Some(Commands::Experiment { phase, instances, runs, output_dir, calibrated_temps }) => {
            run_experiments(&phase, &instances, runs, &output_dir, calibrated_temps.as_deref())
                .expect("Failed to run experiments");
            return;
        }
        None => {}
    }

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
        .sa_warmup_steps(0)
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

fn run_experiments(
    phase: &str,
    instances: &str,
    runs: usize,
    output_dir: &str,
    calibrated_temps_path: Option<&str>,
) -> Result<(), String> {
    use std::time::Instant;

    experiments::output::ensure_output_dir(output_dir)?;

    let instance_list: Vec<&str> = if instances == "all" {
        CHALLENGE_PATHS.to_vec()
    } else {
        vec![instances]
    };

    // Load calibrated temperatures from Phase 0 results or CLI arg
    let calibrated_temps = if let Some(path) = calibrated_temps_path {
        let contents = std::fs::read_to_string(path)
            .map_err(|e| format!("Failed to read calibrated temps: {}", e))?;
        serde_json::from_str(&contents)
            .map_err(|e| format!("Failed to parse calibrated temps: {}", e))?
    } else {
        experiments::configs::get_calibrated_temps(output_dir)
    };

    let start_time = Instant::now();

    match phase {
        "phase0" => {
            for inst_path in &instance_list {
                let inst_name = inst_path.split('/').last().unwrap_or("unknown");
                println!("Running Phase 0 (Calibration) on {}", inst_name);

                let configs = experiments::configs::phase0_configs(inst_name);
                for config in configs {
                    println!("  Running config: {}", config.config_id);
                    let results = experiments::runner::run_experiment(&config, runs, inst_path)?;

                    // Write results
                    let phase_dir = format!("{}/phase0", output_dir);
                    experiments::output::ensure_output_dir(&phase_dir)?;

                    let runs_path = format!("{}/{}_{}_runs.csv", phase_dir, inst_name, config.config_id);
                    experiments::output::write_run_results(&runs_path, &results.runs)?;

                    if let Some(temps) = &results.temp_trajectory {
                        let temp_path = format!("{}/{}_{}_temp_trajectory.csv", phase_dir, inst_name, config.config_id);
                        experiments::output::write_temp_trajectory(&temp_path, temps)?;

                        // Compute warmup stats from final temperatures (step == max_steps)
                        let final_temps: Vec<f32> = temps.iter()
                            .filter(|row| row.step == config.max_steps)
                            .map(|row| row.temperature)
                            .collect();

                        if final_temps.is_empty() {
                            return Err(format!(
                                "No final temperatures found for config {} (expected {} entries with step == {}). Temperature trajectory has {} total entries.",
                                config.config_id, runs, config.max_steps, temps.len()
                            ));
                        }

                        let stats = experiments::runner::compute_warmup_stats(&final_temps);
                        let stats_path = format!("{}/{}_{}_warmup_stats.json", phase_dir, inst_name, config.config_id);
                        experiments::output::write_warmup_stats(&stats_path, &stats)?;
                    }

                    // Write warmup_temp_final.csv with run_id, final_temp, final_fitness
                    let final_rows: Vec<experiments::WarmupFinalRow> = results.runs.iter()
                        .map(|r| experiments::WarmupFinalRow {
                            run_id: r.run_id,
                            final_temp: r.final_temperature,
                            final_fitness: r.final_fitness,
                        })
                        .collect();
                    let final_path = format!("{}/{}_{}_warmup_temp_final.csv", phase_dir, inst_name, config.config_id);
                    experiments::output::write_warmup_final(&final_path, &final_rows)?;

                    let metadata_path = format!("{}/{}_{}_metadata.json", phase_dir, inst_name, config.config_id);
                    experiments::output::write_metadata(&metadata_path, &config, runs, start_time.elapsed().as_millis())?;
                }
            }
        }
        "phase1" => {
            for inst_path in &instance_list {
                let inst_name = inst_path.split('/').last().unwrap_or("unknown");
                println!("Running Phase 1 (Neighborhood Comparison) on {}", inst_name);

                let cal_temp = calibrated_temps.get(inst_name).copied();
                let configs = experiments::configs::phase1_configs(inst_name, cal_temp);
                for config in configs {
                    println!("  Running config: {}", config.config_id);
                    let results = experiments::runner::run_experiment(&config, runs, inst_path)?;

                    let phase_dir = format!("{}/exp01", output_dir);
                    experiments::output::ensure_output_dir(&phase_dir)?;

                    let runs_path = format!("{}/{}_{}_runs.csv", phase_dir, inst_name, config.config_id);
                    experiments::output::write_run_results(&runs_path, &results.runs)?;

                    let convergence = experiments::runner::aggregate_convergence(&results);
                    let convergence_path = format!("{}/{}_{}_convergence.csv", phase_dir, inst_name, config.config_id);
                    experiments::output::write_convergence(&convergence_path, &convergence)?;

                    let metadata_path = format!("{}/{}_{}_metadata.json", phase_dir, inst_name, config.config_id);
                    experiments::output::write_metadata(&metadata_path, &config, runs, start_time.elapsed().as_millis())?;
                }
            }
        }
        "phase2" => {
            for inst_path in &instance_list {
                let inst_name = inst_path.split('/').last().unwrap_or("unknown");
                println!("Running Phase 2 (Advanced Neighborhoods) on {}", inst_name);

                let cal_temp = calibrated_temps.get(inst_name).copied();
                let configs = experiments::configs::phase2_configs(inst_name, cal_temp);
                for config in configs {
                    println!("  Running config: {}", config.config_id);
                    let results = experiments::runner::run_experiment(&config, runs, inst_path)?;

                    let phase_dir = format!("{}/exp02", output_dir);
                    experiments::output::ensure_output_dir(&phase_dir)?;

                    let runs_path = format!("{}/{}_{}_runs.csv", phase_dir, inst_name, config.config_id);
                    experiments::output::write_run_results(&runs_path, &results.runs)?;

                    let convergence = experiments::runner::aggregate_convergence(&results);
                    let convergence_path = format!("{}/{}_{}_convergence.csv", phase_dir, inst_name, config.config_id);
                    experiments::output::write_convergence(&convergence_path, &convergence)?;

                    if let Some(bandit_stats) = &results.bandit_stats {
                        let bandit_path = format!("{}/{}_{}_bandit_stats.csv", phase_dir, inst_name, config.config_id);
                        experiments::output::write_bandit_stats(&bandit_path, bandit_stats)?;
                    }

                    let metadata_path = format!("{}/{}_{}_metadata.json", phase_dir, inst_name, config.config_id);
                    experiments::output::write_metadata(&metadata_path, &config, runs, start_time.elapsed().as_millis())?;
                }
            }
        }
        "phase3" => {
            for inst_path in &instance_list {
                let inst_name = inst_path.split('/').last().unwrap_or("unknown");
                println!("Running Phase 3 (Weight Sensitivity) on {}", inst_name);

                let configs = experiments::configs::phase3_configs(inst_name);
                for config in configs {
                    println!("  Running config: {}", config.config_id);
                    let results = experiments::runner::run_experiment(&config, runs, inst_path)?;

                    let phase_dir = format!("{}/exp03", output_dir);
                    experiments::output::ensure_output_dir(&phase_dir)?;

                    let runs_path = format!("{}/{}_{}_runs.csv", phase_dir, inst_name, config.config_id);
                    experiments::output::write_run_results(&runs_path, &results.runs)?;

                    let convergence = experiments::runner::aggregate_convergence(&results);
                    let convergence_path = format!("{}/{}_{}_convergence.csv", phase_dir, inst_name, config.config_id);
                    experiments::output::write_convergence(&convergence_path, &convergence)?;

                    if let Some(bandit_stats) = &results.bandit_stats {
                        let bandit_path = format!("{}/{}_{}_bandit_stats.csv", phase_dir, inst_name, config.config_id);
                        experiments::output::write_bandit_stats(&bandit_path, bandit_stats)?;
                    }

                    let metadata_path = format!("{}/{}_{}_metadata.json", phase_dir, inst_name, config.config_id);
                    experiments::output::write_metadata(&metadata_path, &config, runs, start_time.elapsed().as_millis())?;
                }
            }
        }
        "phase4" => {
            for inst_path in &instance_list {
                let inst_name = inst_path.split('/').last().unwrap_or("unknown");
                println!("Running Phase 4 (Warmup Ablation) on {}", inst_name);

                let cal_temp = calibrated_temps.get(inst_name).copied();
                let configs = experiments::configs::phase4_configs(inst_name, cal_temp);
                for config in configs {
                    println!("  Running config: {}", config.config_id);
                    let results = experiments::runner::run_experiment(&config, runs, inst_path)?;

                    let phase_dir = format!("{}/exp04", output_dir);
                    experiments::output::ensure_output_dir(&phase_dir)?;

                    let runs_path = format!("{}/{}_{}_runs.csv", phase_dir, inst_name, config.config_id);
                    experiments::output::write_run_results(&runs_path, &results.runs)?;

                    let convergence = experiments::runner::aggregate_convergence(&results);
                    let convergence_path = format!("{}/{}_{}_convergence.csv", phase_dir, inst_name, config.config_id);
                    experiments::output::write_convergence(&convergence_path, &convergence)?;

                    if let Some(bandit_stats) = &results.bandit_stats {
                        let bandit_path = format!("{}/{}_{}_bandit_stats.csv", phase_dir, inst_name, config.config_id);
                        experiments::output::write_bandit_stats(&bandit_path, bandit_stats)?;
                    }

                    let metadata_path = format!("{}/{}_{}_metadata.json", phase_dir, inst_name, config.config_id);
                    experiments::output::write_metadata(&metadata_path, &config, runs, start_time.elapsed().as_millis())?;
                }
            }
        }
        "all" => {
            println!("Running all phases...");
            run_experiments("phase0", instances, runs, output_dir, calibrated_temps_path)?;
            // After Phase 0, calibrated temps will be read from results automatically
            run_experiments("phase1", instances, runs, output_dir, calibrated_temps_path)?;
            run_experiments("phase2", instances, runs, output_dir, calibrated_temps_path)?;
            run_experiments("phase3", instances, runs, output_dir, calibrated_temps_path)?;
            run_experiments("phase4", instances, runs, output_dir, calibrated_temps_path)?;
        }
        _ => return Err(format!("Unknown phase: {}", phase)),
    }

    let elapsed = start_time.elapsed();
    println!("All experiments completed in {:.2}s", elapsed.as_secs_f64());
    Ok(())
}
