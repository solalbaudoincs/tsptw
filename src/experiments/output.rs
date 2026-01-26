use csv::Writer;
use std::fs::{self, File};

use super::types::*;

pub fn ensure_output_dir(dir: &str) -> Result<(), String> {
    fs::create_dir_all(dir)
        .map_err(|e| format!("Failed to create output directory: {}", e))
}

pub fn write_run_results(path: &str, results: &[RunResult]) -> Result<(), String> {
    let file = File::create(path)
        .map_err(|e| format!("Failed to create file {}: {}", path, e))?;

    let mut writer = Writer::from_writer(file);
    for result in results {
        writer
            .serialize(result)
            .map_err(|e| format!("Failed to serialize run result: {}", e))?;
    }

    writer.flush()
        .map_err(|e| format!("Failed to flush writer: {}", e))
}

pub fn write_convergence(path: &str, rows: &[ConvergenceRow]) -> Result<(), String> {
    let file = File::create(path)
        .map_err(|e| format!("Failed to create file {}: {}", path, e))?;

    let mut writer = Writer::from_writer(file);
    for row in rows {
        writer
            .serialize(row)
            .map_err(|e| format!("Failed to serialize convergence row: {}", e))?;
    }

    writer.flush()
        .map_err(|e| format!("Failed to flush writer: {}", e))
}

pub fn write_temp_trajectory(path: &str, rows: &[TempTrajectoryRow]) -> Result<(), String> {
    let file = File::create(path)
        .map_err(|e| format!("Failed to create file {}: {}", path, e))?;

    let mut writer = Writer::from_writer(file);
    for row in rows {
        writer
            .serialize(row)
            .map_err(|e| format!("Failed to serialize temp trajectory row: {}", e))?;
    }

    writer.flush()
        .map_err(|e| format!("Failed to flush writer: {}", e))
}

pub fn write_warmup_final(path: &str, rows: &[WarmupFinalRow]) -> Result<(), String> {
    let file = File::create(path)
        .map_err(|e| format!("Failed to create file {}: {}", path, e))?;

    let mut writer = Writer::from_writer(file);
    for row in rows {
        writer
            .serialize(row)
            .map_err(|e| format!("Failed to serialize warmup final row: {}", e))?;
    }

    writer.flush()
        .map_err(|e| format!("Failed to flush writer: {}", e))
}

pub fn write_bandit_stats(path: &str, rows: &[BanditStatsRow]) -> Result<(), String> {
    let file = File::create(path)
        .map_err(|e| format!("Failed to create file {}: {}", path, e))?;

    let mut writer = Writer::from_writer(file);
    for row in rows {
        writer
            .serialize(row)
            .map_err(|e| format!("Failed to serialize bandit stats row: {}", e))?;
    }

    writer.flush()
        .map_err(|e| format!("Failed to flush writer: {}", e))
}

pub fn write_warmup_stats(path: &str, stats: &WarmupStats) -> Result<(), String> {
    let json = serde_json::to_string_pretty(stats)
        .map_err(|e| format!("Failed to serialize warmup stats: {}", e))?;

    std::fs::write(path, json)
        .map_err(|e| format!("Failed to write warmup stats: {}", e))
}

pub fn write_metadata(path: &str, config: &ExperimentConfig, num_runs: usize, runtime_ms: u128) -> Result<(), String> {
    let metadata = serde_json::json!({
        "experiment_id": config.config_id,
        "instance": config.instance,
        "neighborhood": format!("{:?}", config.neighborhood),
        "warmup": config.warmup,
        "initial_temperature": config.initial_temperature,
        "cooling_rate": config.cooling_rate,
        "stopping_temperature": config.stopping_temperature,
        "max_steps": config.max_steps,
        "backtrack_interval": config.backtrack_interval,
        "distance_weight": config.distance_weight,
        "violation_time_weight": config.violation_time_weight,
        "total_time_weight": config.total_time_weight,
        "delay_weight": config.delay_weight,
        "runs": num_runs,
        "seed_range": [0, num_runs - 1],
        "execution_date": chrono::Local::now().to_rfc3339(),
        "total_runtime_ms": runtime_ms,
    });

    let json = serde_json::to_string_pretty(&metadata)
        .map_err(|e| format!("Failed to serialize metadata: {}", e))?;

    std::fs::write(path, json)
        .map_err(|e| format!("Failed to write metadata: {}", e))
}

pub fn format_phase_path(results_dir: &str, phase: &str, inst: &str, config_id: &str, file_type: &str) -> String {
    format!("{}/{}/{}_{}.{}", results_dir, phase, inst, config_id, file_type)
}

pub fn format_warmup_path(results_dir: &str, inst: &str, neighborhood: &str, file_type: &str) -> String {
    format!("{}/phase0/{}_{}_warmup.{}", results_dir, inst, neighborhood, file_type)
}
