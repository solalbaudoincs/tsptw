use crate::neighborhood::NeighborhoodType;
use super::types::{ExperimentConfig, WarmupStats};
use std::collections::HashMap;

pub fn phase0_configs(instance: &str) -> Vec<ExperimentConfig> {
    vec![
        ExperimentConfig {
            config_id: "phase0-Swap".to_string(),
            instance: instance.to_string(),
            neighborhood: NeighborhoodType::Swap,
            warmup: true,
            initial_temperature: 10000.0,
            cooling_rate: 1.0,  // No cooling during calibration
            stopping_temperature: 0.0001,
            acceptance_smoothing_factor: 0.95,
            initial_acceptance_rate: 0.9,
            delta_fitness_smoothing_factor: 0.95,
            distance_weight: 0.0,
            violation_time_weight: 4.0,
            total_time_weight: 4.0,
            delay_weight: 2.0,
            max_steps: 100000,  // Warmup steps for phase 0
            sampling_interval: 100,
            backtrack_interval: 0,
            max_warmup_steps: 100000,
        },
        ExperimentConfig {
            config_id: "phase0-TwoOpt".to_string(),
            instance: instance.to_string(),
            neighborhood: NeighborhoodType::TwoOpt,
            warmup: true,
            initial_temperature: 10000.0,
            cooling_rate: 1.0,
            stopping_temperature: 0.0001,
            acceptance_smoothing_factor: 0.95,
            initial_acceptance_rate: 0.9,
            delta_fitness_smoothing_factor: 0.95,
            distance_weight: 0.0,
            violation_time_weight: 4.0,
            total_time_weight: 4.0,
            delay_weight: 2.0,
            max_steps: 100000,
            sampling_interval: 100,
            backtrack_interval: 0,
            max_warmup_steps: 100000,
        },
    ]
}

pub fn phase1_configs(instance: &str, calibrated_temp: Option<f32>) -> Vec<ExperimentConfig> {
    let temp = calibrated_temp.expect("flop");
    vec![
        ExperimentConfig {
            config_id: "EXP-01A".to_string(),
            instance: instance.to_string(),
            neighborhood: NeighborhoodType::Swap,
            warmup: true,
            initial_temperature: 10000.0,
            cooling_rate: 0.99999,
            stopping_temperature: 0.0001,
            acceptance_smoothing_factor: 0.95,
            initial_acceptance_rate: 0.9,
            delta_fitness_smoothing_factor: 0.95,
            distance_weight: 0.0,
            violation_time_weight: 4.0,
            total_time_weight: 4.0,
            delay_weight: 2.0,
            max_steps: 1000000,
            sampling_interval: 500,
            backtrack_interval: 0,
            max_warmup_steps: 100000,
        },
        ExperimentConfig {
            config_id: "EXP-01B".to_string(),
            instance: instance.to_string(),
            neighborhood: NeighborhoodType::Swap,
            warmup: false,
            initial_temperature: temp,
            cooling_rate: 0.99999,
            stopping_temperature: 0.0001,
            acceptance_smoothing_factor: 0.95,
            initial_acceptance_rate: 0.9,
            delta_fitness_smoothing_factor: 0.95,
            distance_weight: 0.0,
            violation_time_weight: 4.0,
            total_time_weight: 4.0,
            delay_weight: 2.0,
            max_steps: 1000000,
            sampling_interval: 500,
            backtrack_interval: 0,
            max_warmup_steps: 0,
        },
        ExperimentConfig {
            config_id: "EXP-01C".to_string(),
            instance: instance.to_string(),
            neighborhood: NeighborhoodType::TwoOpt,
            warmup: true,
            initial_temperature: 10000.0,
            cooling_rate: 0.99999,
            stopping_temperature: 0.0001,
            acceptance_smoothing_factor: 0.95,
            initial_acceptance_rate: 0.9,
            delta_fitness_smoothing_factor: 0.95,
            distance_weight: 0.0,
            violation_time_weight: 4.0,
            total_time_weight: 4.0,
            delay_weight: 2.0,
            max_steps: 1000000,
            sampling_interval: 500,
            backtrack_interval: 0,
            max_warmup_steps: 100000,
        },
        ExperimentConfig {
            config_id: "EXP-01D".to_string(),
            instance: instance.to_string(),
            neighborhood: NeighborhoodType::TwoOpt,
            warmup: false,
            initial_temperature: temp,
            cooling_rate: 0.99999,
            stopping_temperature: 0.0001,
            acceptance_smoothing_factor: 0.95,
            initial_acceptance_rate: 0.9,
            delta_fitness_smoothing_factor: 0.95,
            distance_weight: 0.0,
            violation_time_weight: 4.0,
            total_time_weight: 4.0,
            delay_weight: 2.0,
            max_steps: 1000000,
            sampling_interval: 500,
            backtrack_interval: 0,
            max_warmup_steps: 0,
        },
    ]
}

pub fn phase2_configs(instance: &str, calibrated_temp: Option<f32>) -> Vec<ExperimentConfig> {
    let temp = calibrated_temp.expect("Calibrated temperature required for phase 2 configs");
    vec![
        ExperimentConfig {
            config_id: "EXP-02A".to_string(),
            instance: instance.to_string(),
            neighborhood: NeighborhoodType::Alternating,
            warmup: false,
            initial_temperature: temp,
            cooling_rate: 0.99999,
            stopping_temperature: 0.0001,
            acceptance_smoothing_factor: 0.95,
            initial_acceptance_rate: 0.9,
            delta_fitness_smoothing_factor: 0.95,
            distance_weight: 0.0,
            violation_time_weight: 4.0,
            total_time_weight: 4.0,
            delay_weight: 2.0,
            max_steps: 1000000,
            sampling_interval: 500,
            backtrack_interval: 0,
            max_warmup_steps: 0,
        },
        ExperimentConfig {
            config_id: "EXP-02B".to_string(),
            instance: instance.to_string(),
            neighborhood: NeighborhoodType::Bandit,
            warmup: false,
            initial_temperature: temp,
            cooling_rate: 0.99999,
            stopping_temperature: 0.0001,
            acceptance_smoothing_factor: 0.95,
            initial_acceptance_rate: 0.9,
            delta_fitness_smoothing_factor: 0.95,
            distance_weight: 0.0,
            violation_time_weight: 4.0,
            total_time_weight: 4.0,
            delay_weight: 2.0,
            max_steps: 1000000,
            sampling_interval: 500,
            backtrack_interval: 0,
            max_warmup_steps: 0,
        },
    ]
}

pub fn phase3_configs(instance: &str) -> Vec<ExperimentConfig> {
    vec![
        ExperimentConfig {
            config_id: "EXP-03A".to_string(),
            instance: instance.to_string(),
            neighborhood: NeighborhoodType::Bandit,
            warmup: true,
            initial_temperature: 10000.0,
            cooling_rate: 0.99999,
            stopping_temperature: 0.0001,
            acceptance_smoothing_factor: 0.95,
            initial_acceptance_rate: 0.9,
            delta_fitness_smoothing_factor: 0.95,
            distance_weight: 0.0,
            violation_time_weight: 4.0,
            total_time_weight: 4.0,
            delay_weight: 2.0,
            max_steps: 1000000,
            sampling_interval: 500,
            backtrack_interval: 0,
            max_warmup_steps: 100000,
        },
        ExperimentConfig {
            config_id: "EXP-03B".to_string(),
            instance: instance.to_string(),
            neighborhood: NeighborhoodType::Bandit,
            warmup: true,
            initial_temperature: 10000.0,
            cooling_rate: 0.99999,
            stopping_temperature: 0.0001,
            acceptance_smoothing_factor: 0.95,
            initial_acceptance_rate: 0.9,
            delta_fitness_smoothing_factor: 0.95,
            distance_weight: 0.0,
            violation_time_weight: 10.0,
            total_time_weight: 2.0,
            delay_weight: 1.0,
            max_steps: 1000000,
            sampling_interval: 500,
            backtrack_interval: 0,
            max_warmup_steps: 100000,
        },
        ExperimentConfig {
            config_id: "EXP-03C".to_string(),
            instance: instance.to_string(),
            neighborhood: NeighborhoodType::Bandit,
            warmup: true,
            initial_temperature: 10000.0,
            cooling_rate: 0.99999,
            stopping_temperature: 0.0001,
            acceptance_smoothing_factor: 0.95,
            initial_acceptance_rate: 0.9,
            delta_fitness_smoothing_factor: 0.95,
            distance_weight: 0.0,
            violation_time_weight: 2.0,
            total_time_weight: 4.0,
            delay_weight: 4.0,
            max_steps: 1000000,
            sampling_interval: 500,
            backtrack_interval: 0,
            max_warmup_steps: 100000,
        },
        ExperimentConfig {
            config_id: "EXP-03D".to_string(),
            instance: instance.to_string(),
            neighborhood: NeighborhoodType::Bandit,
            warmup: true,
            initial_temperature: 10000.0,
            cooling_rate: 0.99999,
            stopping_temperature: 0.0001,
            acceptance_smoothing_factor: 0.95,
            initial_acceptance_rate: 0.9,
            delta_fitness_smoothing_factor: 0.95,
            distance_weight: 1.0,
            violation_time_weight: 5.0,
            total_time_weight: 3.0,
            delay_weight: 2.0,
            max_steps: 1000000,
            sampling_interval: 500,
            backtrack_interval: 0,
            max_warmup_steps: 100000,
        },
    ]
}

pub fn phase4_configs(instance: &str, calibrated_temp: Option<f32>) -> Vec<ExperimentConfig> {
    let temp = calibrated_temp.expect("flop");
    vec![
        ExperimentConfig {
            config_id: "EXP-04A".to_string(),
            instance: instance.to_string(),
            neighborhood: NeighborhoodType::Bandit,
            warmup: true,
            initial_temperature: 10000.0,
            cooling_rate: 0.99999,
            stopping_temperature: 0.0001,
            acceptance_smoothing_factor: 0.95,
            initial_acceptance_rate: 0.9,
            delta_fitness_smoothing_factor: 0.95,
            distance_weight: 0.0,
            violation_time_weight: 4.0,
            total_time_weight: 4.0,
            delay_weight: 2.0,
            max_steps: 1000000,
            sampling_interval: 500,
            backtrack_interval: 0,
            max_warmup_steps: 100000,
        },
        ExperimentConfig {
            config_id: "EXP-04B".to_string(),
            instance: instance.to_string(),
            neighborhood: NeighborhoodType::Bandit,
            warmup: false,  
            initial_temperature: temp,
            cooling_rate: 0.99999,
            stopping_temperature: 0.0001,
            acceptance_smoothing_factor: 0.95,
            initial_acceptance_rate: 0.9,
            delta_fitness_smoothing_factor: 0.95,
            distance_weight: 0.0,
            violation_time_weight: 4.0,
            total_time_weight: 4.0,
            delay_weight: 2.0,
            max_steps: 1000000,
            sampling_interval: 500,
            backtrack_interval: 0,
            max_warmup_steps: 0,
        },
    ]
}

pub fn get_calibrated_temps(results_dir: &str) -> HashMap<String, f32> {
    let mut temps = HashMap::new();
    let instances = ["inst1", "inst2", "inst3", "inst_concours"];
    let default_temps: HashMap<&str, f32> = [
        ("inst1", 2500.0),
        ("inst2", 2800.0),
        ("inst3", 3200.0),
        ("inst_concours", 3100.0),
    ].into_iter().collect();

    for inst in &instances {
        // Try to read warmup stats from Phase 0 results (use Swap neighborhood)
        let stats_path = format!("{}/phase0/{}_phase0-Swap_warmup_stats.json", results_dir, inst);
        if let Ok(contents) = std::fs::read_to_string(&stats_path) {
            if let Ok(stats) = serde_json::from_str::<WarmupStats>(&contents) {
                temps.insert(inst.to_string(), stats.mean);
                continue;
            }
        }
        // Fall back to defaults
        temps.insert(inst.to_string(), *default_temps.get(inst).unwrap_or(&2500.0));
    }

    temps
}
