use serde::{Serialize, Deserialize};
use crate::neighborhood::NeighborhoodType;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct RunResult {
    pub run_id: usize,
    pub config: String,
    pub best_fitness: f32,
    pub best_step: usize,
    pub final_fitness: f32,
    pub final_temperature: f32,
    pub violations: u32,
    pub feasible: bool,
    pub total_distance: f32,
    pub total_time: f32,
    pub execution_time_ms: u128,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct WarmupFinalRow {
    pub run_id: usize,
    pub final_temp: f32,
    pub final_fitness: f32,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct StepSnapshot {
    pub step: usize,
    pub fitness: f32,
    pub temperature: f32,
    pub feasible: bool,
    pub violations: u32,
    pub distance: f32,
    pub time: f32,
    pub delay: f32,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ConvergenceRow {
    pub step: usize,
    pub mean_fitness: f32,
    pub median_fitness: f32,
    pub q1_fitness: f32,
    pub q3_fitness: f32,
    pub p10_fitness: f32,
    pub p90_fitness: f32,
    pub feasible_rate: f32,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct TempTrajectoryRow {
    pub run_id: usize,
    pub step: usize,
    pub temperature: f32,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct BanditStatsRow {
    pub run_id: usize,
    pub step: usize,
    pub swap_selections: usize,
    pub twoopt_selections: usize,
    pub swap_avg_reward: f64,
    pub twoopt_avg_reward: f64,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct WarmupStats {
    pub mean: f32,
    pub std: f32,
    pub median: f32,
    pub q1: f32,
    pub q3: f32,
    pub min: f32,
    pub max: f32,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ExperimentConfig {
    pub config_id: String,
    pub instance: String,
    pub neighborhood: NeighborhoodType,
    pub warmup: bool,
    pub initial_temperature: f32,
    pub cooling_rate: f32,
    pub stopping_temperature: f32,
    pub acceptance_smoothing_factor: f32,
    pub initial_acceptance_rate: f32,
    pub delta_fitness_smoothing_factor: f32,
    pub distance_weight: f32,
    pub violation_time_weight: f32,
    pub total_time_weight: f32,
    pub delay_weight: f32,
    pub max_steps: usize,
    pub sampling_interval: usize,
    pub backtrack_interval: usize,
    pub max_warmup_steps: usize,
}

#[derive(Clone, Debug)]
pub struct ExperimentResults {
    pub config: ExperimentConfig,
    pub runs: Vec<RunResult>,
    pub convergence_data: Vec<Vec<StepSnapshot>>,
    pub temp_trajectory: Option<Vec<TempTrajectoryRow>>,
    pub bandit_stats: Option<Vec<BanditStatsRow>>,
}

#[derive(Clone, Debug)]
pub struct SingleRunData {
    pub run_id: usize,
    pub best_fitness: f32,
    pub best_step: usize,
    pub final_fitness: f32,
    pub final_temperature: f32,
    pub violations: u32,
    pub feasible: bool,
    pub total_distance: f32,
    pub total_time: f32,
    pub execution_time_ms: u128,
    pub snapshots: Vec<StepSnapshot>,
    pub temp_trajectory: Vec<TempTrajectoryRow>,
    pub bandit_stats: Vec<BanditStatsRow>,
}
