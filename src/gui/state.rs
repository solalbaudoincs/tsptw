use std::collections::HashMap;
use rayon::prelude::*;

use crate::gui::solver::{Solver, ConcreteSolver};
use crate::initializer::{Initializer, RandomInitializer};
use crate::shared::{Instance, GraphInstance, Solution};
use crate::eval::{Evaluation, Weighted, Lexicographic};
use crate::algorithms::{SimulatedAnnealing, GeneticAlgorithm, CrossoverType, CompetitionType, HillClimbing, ACO};
use crate::neighborhood::{Swap, TwoOpt};
use crate::io::io_instance::load_instance;

use crate::neighborhood::NeighborhoodType;
use crate::eval::EvaluationType;

#[derive(PartialEq, Clone, Copy)]
pub enum AppPhase {
    Configuration,
    Running,
}

#[derive(PartialEq, Clone, Copy)]
pub enum AlgoType {
    SimulatedAnnealing,
    GeneticAlgorithm,
    HillClimbing,
    AntColonyOptimization,
}

#[derive(PartialEq, Clone, Copy)]
pub enum ViewTab {
    Route,
    Metrics,
    Gantt,
}

#[derive(PartialEq, Clone, Copy)]
pub enum ViewMode {
    Grid,
    Statistics,
}

pub struct LogEntry {
    pub iteration: usize,
    pub current_dist: f32,
    pub current_viol: f32,
    pub metrics: HashMap<String, f32>,
}

pub struct VisitInfo {
    pub node_idx: usize,
    pub arrival_time: f32,
    pub wait_time: f32,
    pub window_start: f32,
    pub window_end: f32,
    pub violation: f32,
}

pub struct RunState {
    pub id: usize,
    pub name: String,
    pub solver: Option<Box<dyn Solver>>,
    pub is_running: bool,
    pub history: Vec<LogEntry>,
    pub current_solution_path: Vec<u32>,
    pub metric_names: Vec<String>,
}

impl RunState {
    pub fn new(id: usize, name: String) -> Self {
        Self {
            id,
            name,
            solver: None,
            is_running: false,
            history: Vec::new(),
            current_solution_path: Vec::new(),
            metric_names: Vec::new(),
        }
    }

    pub fn update(&mut self, steps: usize, max_steps: usize) {
        if self.is_running {
            if let Some(solver) = &mut self.solver {
                for _ in 0..steps {
                    if solver.get_iteration() >= max_steps {
                        self.is_running = false;
                        break;
                    }
                    solver.step();
                }
                
                if let Some((path, dist, viol)) = solver.get_best_solution() {
                    self.current_solution_path = path;
                    let metrics = solver.get_metrics();
                    self.history.push(LogEntry {
                        iteration: solver.get_iteration(),
                        current_dist: dist,
                        current_viol: viol,
                        metrics,
                    });
                }
            }
        }
    }

    pub fn get_schedule(&self, instance: &Instance) -> Vec<VisitInfo> {
        let mut schedule = Vec::new();
        if self.current_solution_path.is_empty() {
            return schedule;
        }

        let mut current_time = 0.0;
        
        let start_node_idx = self.current_solution_path[0] as usize;
        schedule.push(VisitInfo {
            node_idx: start_node_idx,
            arrival_time: 0.0,
            wait_time: 0.0,
            window_start: instance.windows[start_node_idx].wstart,
            window_end: instance.windows[start_node_idx].wend,
            violation: 0.0,
        });

        for i in 0..self.current_solution_path.len() {
            let from_idx = self.current_solution_path[i] as usize;
            let next_i = (i + 1) % self.current_solution_path.len();
            let to_idx = self.current_solution_path[next_i] as usize;
            
            let travel_time = instance.distance_matrix[[from_idx, to_idx]];
            let arrival_time = current_time + travel_time;
            
            let wstart = instance.windows[to_idx].wstart;
            let wend = instance.windows[to_idx].wend;
            
            let mut wait_time = 0.0;
            let mut violation = 0.0;
            
            let effective_arrival = if arrival_time < wstart {
                wait_time = wstart - arrival_time;
                wstart
            } else {
                arrival_time
            };
            
            if effective_arrival > wend {
                violation = effective_arrival - wend;
            }
            
            current_time = effective_arrival;
            
            schedule.push(VisitInfo {
                node_idx: to_idx,
                arrival_time,
                wait_time,
                window_start: wstart,
                window_end: wend,
                violation,
            });
        }
        schedule
    }
}

pub struct AppState {
    pub phase: AppPhase,
    pub instance_path: String,
    pub instance: Option<Instance>,
    pub graph_instance: Option<GraphInstance>,
    
    pub algo_type: AlgoType,
    pub neighborhood_type: NeighborhoodType,
    pub evaluation_type: EvaluationType,
    
    pub sa_temp: f32,
    pub sa_cooling: f32,
    pub violation_coefficient: f32,
    pub lexicographic_distance_first: bool,

    // ACO parameters
    pub aco_evaporation_rate: f32,
    pub aco_alpha: f32,
    pub aco_beta: f32,
    pub aco_pheromone_deposit: f32,
    
    pub population_size: usize,

    pub steps_per_frame: usize,
    pub max_steps: usize,
    
    pub runs: Vec<RunState>,
    pub selected_run_index: Option<usize>,
    pub next_run_id: usize,
    pub parallel_runs_count: usize,
    pub view_mode: ViewMode,
    
    // Visualization state
    pub left_col_ratio: f32,
    pub right_top_ratio: f32,
    pub violation_log_scale: bool,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            phase: AppPhase::Configuration,
            instance_path: "data/inst1".to_string(),
            instance: None,
            graph_instance: None,
            algo_type: AlgoType::SimulatedAnnealing,
            neighborhood_type: NeighborhoodType::Swap,
            evaluation_type: EvaluationType::Weighted,
            sa_temp: 1000.0,
            sa_cooling: 0.9995,
            violation_coefficient: 1000.0,
            lexicographic_distance_first: false,
            aco_evaporation_rate: 0.1,
            aco_alpha: 1.0,
            aco_beta: 2.0,
            aco_pheromone_deposit: 1.0,
            population_size: 50,
            steps_per_frame: 100,
            max_steps: 10000,
            runs: Vec::new(),
            selected_run_index: None,
            next_run_id: 0,
            parallel_runs_count: 1,
            view_mode: ViewMode::Grid,
            left_col_ratio: 0.6,
            right_top_ratio: 0.5,
            violation_log_scale: false,
        }
    }

    pub fn load_instance(&mut self) {
        if std::path::Path::new(&self.instance_path).exists() {
            match load_instance(&self.instance_path) {
                Ok((instance, graph_instance)) => {
                    self.instance = Some(instance);
                    self.graph_instance = Some(graph_instance);
                    self.runs.clear();
                    self.selected_run_index = None;
                }
                Err(e) => {
                    eprintln!("Error loading instance: {}", e);
                }
            }
        } else {
            eprintln!("Instance not found: {}", self.instance_path);
        }
    }

    pub fn start_new_run(&mut self) {
        if let Some(instance) = &self.instance {
            let mut run = RunState::new(self.next_run_id, format!("Run {}", self.next_run_id));
            self.next_run_id += 1;

            let mut initializer = RandomInitializer;
            let population = vec![initializer.initialize(instance)];
            
            match self.algo_type {
                AlgoType::SimulatedAnnealing => {
                    let algo = SimulatedAnnealing::new(self.sa_temp, self.sa_cooling, 0.001, instance);
                    
                    match self.evaluation_type {
                        EvaluationType::Weighted => {
                            let eval = Weighted { violation_coefficient: self.violation_coefficient };
                            let initial_fitness = eval.score(instance, &population[0]);
                            let fitnesses = vec![initial_fitness];
                            
                            match self.neighborhood_type {
                                NeighborhoodType::Swap => {
                                    let neighbor = Swap::new();
                                    let solver = ConcreteSolver::new(algo, eval, neighbor, instance.clone(), population, fitnesses);
                                    run.metric_names = solver.get_metric_names();
                                    run.solver = Some(Box::new(solver));
                                },
                                NeighborhoodType::TwoOpt => {
                                    let neighbor = TwoOpt::new();
                                    let solver = ConcreteSolver::new(algo, eval, neighbor, instance.clone(), population, fitnesses);
                                    run.metric_names = solver.get_metric_names();
                                    run.solver = Some(Box::new(solver));
                                }
                            }
                        },
                        EvaluationType::Lexicographic => {
                            let eval = Lexicographic::new(self.lexicographic_distance_first);
                            let initial_fitness = eval.score(instance, &population[0]);
                            let fitnesses = vec![initial_fitness];
                            
                            match self.neighborhood_type {
                                NeighborhoodType::Swap => {
                                    let neighbor = Swap::new();
                                    let solver = ConcreteSolver::new(algo, eval, neighbor, instance.clone(), population, fitnesses);
                                    run.metric_names = solver.get_metric_names();
                                    run.solver = Some(Box::new(solver));
                                },
                                NeighborhoodType::TwoOpt => {
                                    let neighbor = TwoOpt::new();
                                    let solver = ConcreteSolver::new(algo, eval, neighbor, instance.clone(), population, fitnesses);
                                    run.metric_names = solver.get_metric_names();
                                    run.solver = Some(Box::new(solver));
                                }
                            }
                        }
                    }
                }
                AlgoType::GeneticAlgorithm => {
                    let pop_size: usize = self.population_size;
                    let mut population: Vec<Solution> = Vec::with_capacity(pop_size);
                    for _ in 0..pop_size {
                        population.push(initializer.initialize(instance));
                    }

                    let algo = GeneticAlgorithm::new(
                        instance,
                        0.8,  // crossover_rate
                        CrossoverType::PMX,
                        0.1,  // elitism_rate
                        0.5,  // competition_participation_rate
                        CompetitionType::Tournament,
                        pop_size,
                    );

                    let eval = Weighted { violation_coefficient: self.violation_coefficient };
                    let fitnesses: Vec<f32> = population.iter().map(|s| eval.score(instance, s)).collect();

                    let neighbor = Swap::new();
                    let solver = ConcreteSolver::new(algo, eval, neighbor, instance.clone(), population, fitnesses);
                    run.metric_names = Solver::get_metric_names(&solver);
                    run.solver = Some(Box::new(solver));
                }
                AlgoType::HillClimbing => {
                    let population = vec![initializer.initialize(instance)];
                    let eval = Weighted { violation_coefficient: self.violation_coefficient };
                    let initial_fitness = eval.score(instance, &population[0]);
                    let fitnesses = vec![initial_fitness];

                    let algo = HillClimbing::new(20, instance);

                    let neighbor = Swap::new();
                    let solver = ConcreteSolver::new(algo, eval, neighbor, instance.clone(), population, fitnesses);
                    run.metric_names = Solver::get_metric_names(&solver);
                    run.solver = Some(Box::new(solver));
                }
                AlgoType::AntColonyOptimization => {
                    let pop_size: usize = self.population_size;
                    let mut population: Vec<Solution> = Vec::with_capacity(pop_size);
                    for _ in 0..pop_size {
                        population.push(initializer.initialize(instance));
                    }

                    let algo = ACO::new(
                        instance,
                        self.aco_evaporation_rate,
                        self.aco_alpha,
                        self.aco_beta,
                        self.max_steps,
                        self.aco_pheromone_deposit,
                    );

                    let eval = Weighted { violation_coefficient: self.violation_coefficient };
                    let fitnesses: Vec<f32> = population.iter().map(|s| eval.score(instance, s)).collect();

                    let neighbor = Swap::new();
                    let solver = ConcreteSolver::new(algo, eval, neighbor, instance.clone(), population, fitnesses);
                    run.metric_names = Solver::get_metric_names(&solver);
                    run.solver = Some(Box::new(solver));
                }
            }
            run.is_running = true;
            self.runs.push(run);
        }
    }

    pub fn update_solvers(&mut self) {
        let steps = self.steps_per_frame;
        let max_steps = self.max_steps;
        self.runs.par_iter_mut().for_each(|run| {
            run.update(steps, max_steps);
        });
    }
}
