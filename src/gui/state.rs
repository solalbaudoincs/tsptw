use rayon::prelude::*;
use std::collections::HashMap;
use std::sync::Arc; // Crucial pour partager l'instance entre les threads

// Vos imports existants...
use crate::algorithms::{Metaheuristic};
use crate::eval::{Evaluation, Lexicographic, Weighted, EvaluationType};
use crate::initializer::{Initializer, RandomInitializer};
use crate::io::io_instance::load_instance;
use crate::neighborhood::{NeighborhoodType, LocalSearchType};
use crate::shared::{GraphInstance, Instance, Solution, Fitness};
use crate::factories::*;
use crate::factories::AlgoType;
use crate::algorithms::{CrossoverType, CompetitionType};

// --- 1. Configuration & Enums (Nettoyage) ---

#[derive(PartialEq, Clone, Copy)]
pub enum AppPhase { Configuration, Running }

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

// Regroupement des paramètres d'algorithme
#[derive(Clone)]
pub struct AlgoConfigParams {
    // Simulated Annealing
    pub sa_temp: f32,
    pub sa_cooling: f32,
    pub sa_stopping: f32,
    pub sa_acceptance_smoothing: f32,
    pub sa_initial_acceptance_rate: f32,
    pub sa_delta_fitness_smoothing: f32,
    pub sa_backtracking_interval: usize,
    
    // Genetic Algorithm
    pub ga_crossover_rate: f32,
    pub ga_crossover_type: CrossoverType,
    pub ga_elitism_rate: f32,
    pub ga_competition_participation_rate: f32,
    pub ga_competition_type: CompetitionType,
    pub ga_mutation_rate: f32,
    
    // Hill Climbing
    pub hc_step: usize,
    
    // Ant Colony Optimization
    pub aco_evaporation: f32,
    pub aco_alpha: f32,
    pub aco_beta: f32,
    pub aco_deposit: f32,
    
    // Common parameters
    pub neighborhood: NeighborhoodType,
    pub local_search_type: LocalSearchType,
    pub max_steps: usize,
    pub population_size: usize,
}

impl Default for AlgoConfigParams {
    fn default() -> Self {
        Self {
            // Simulated Annealing
            sa_temp: 63000.0,
            sa_cooling: 0.99999,
            sa_stopping: 0.001,
            sa_acceptance_smoothing: 0.9,
            sa_initial_acceptance_rate: 0.99999,
            sa_delta_fitness_smoothing: 0.9,
            sa_backtracking_interval: 0,
            
            // Genetic Algorithm
            ga_crossover_rate: 0.8,
            ga_crossover_type: CrossoverType::default(),
            ga_elitism_rate: 0.1,
            ga_competition_participation_rate: 0.5,
            ga_competition_type: CompetitionType::default(),
            ga_mutation_rate: 0.1,
            
            // Hill Climbing
            hc_step: 100,
            
            // Ant Colony Optimization
            aco_evaporation: 0.5,
            aco_alpha: 1.0,
            aco_beta: 2.0,
            aco_deposit: 1.0,
            
            // Common parameters
            neighborhood: NeighborhoodType::default(),
            local_search_type: LocalSearchType::default(),
            max_steps: 100000000,
            population_size: 100,
        }
    }
}

// Regroupement des paramètres d'évaluation
#[derive(Clone)]
pub struct EvalConfigParams {
    // Weighted evaluation
    pub violation_coefficient: f32,
    pub total_distance_weight: f32,
    pub violation_time_weight: f32,
    pub total_time_weight: f32,
    pub delay_weight: f32,
    
    // Lexicographic evaluation
    pub lexicographic_distance_first: bool,
}

impl Default for EvalConfigParams {
    fn default() -> Self {
        Self {
            total_distance_weight: 1.0,
            violation_time_weight: 10.0,
            total_time_weight: 0.0,
            delay_weight: 5.0,
            violation_coefficient: 100.0,
            lexicographic_distance_first: true,
        }
    }
}

// --- 2. Abstraction du Runner ---

// On déplace la logique d'exécution "Weighted" vs "Lexicographic" ici
// pour ne pas polluer RunState.
pub enum Runner {
    Weighted(Box<dyn Metaheuristic<Weighted>>, Weighted),
    Lexicographic(Box<dyn Metaheuristic<Lexicographic>>, Lexicographic),
}

impl Runner {
    /// Exécute un lot d'étapes (batch) et retourne si l'algo est terminé
    pub fn step_batch(
        &mut self, 
        population: &mut [Solution], 
        fitnesses: &mut [Fitness], 
        instance: &Instance, 
        steps: usize, 
        max_total_steps: usize
    ) -> bool {
        match self {
            Runner::Weighted(algo, eval) => {
                for _ in 0..steps {
                    if algo.get_iteration() >= max_total_steps { return true; }
                    algo.step(population, fitnesses, instance, eval);
                }
                algo.get_iteration() >= max_total_steps
            },
            Runner::Lexicographic(algo, eval) => {
                for _ in 0..steps {
                    if algo.get_iteration() >= max_total_steps { return true; }
                    algo.step(population, fitnesses, instance, eval);
                }
                algo.get_iteration() >= max_total_steps
            }
        }
    }

    pub fn get_metrics(&self) -> HashMap<String, f32> {
        match self {
            Runner::Weighted(algo, _) => algo.get_metrics(),
            Runner::Lexicographic(algo, _) => algo.get_metrics(),
        }
    }

    pub fn get_metric_names(&self) -> Vec<String> {
        match self {
            Runner::Weighted(algo, _) => algo.get_metric_names(),
            Runner::Lexicographic(algo, _) => algo.get_metric_names(),
        }
    }

    pub fn get_best_solution(&self, pop: &[Solution], fitnesses: &[Fitness], instance: &Instance) -> Option<(Vec<u32>, f32, f32)> {
        if pop.is_empty() || fitnesses.is_empty() {
            return None;
        }

        let (best_index, _) = fitnesses
            .iter()
            .enumerate()
            .min_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .unwrap();

        let best_solution = pop[best_index].clone();
        
        // Calculate real metrics using evaluation
        use crate::eval::utils::run_solution;
        let result = run_solution(instance, &best_solution);
        Some((best_solution, result.total_distance, result.violation_time))
    }
    
    pub fn current_iteration(&self) -> usize {
        match self {
            Runner::Weighted(algo, _) => algo.get_iteration(),
            Runner::Lexicographic(algo, _) => algo.get_iteration(),
        }
    }
}

// --- 3. Structures de Données (Logs & Visites) ---

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

// --- 4. RunState (État d'une exécution) ---

pub struct RunState {
    pub id: usize,
    pub name: String,
    pub runner: Option<Runner>,
    pub population: Vec<Solution>,
    pub fitnesses: Vec<Fitness>,
    // Utilisation de Arc pour éviter le clone coûteux de l'instance
    pub instance: Option<Arc<Instance>>, 
    pub is_running: bool,
    pub history: Vec<LogEntry>,
    pub current_solution_path: Vec<u32>,
    pub metric_names: Vec<String>,
}

impl RunState {
    pub fn new(id: usize, name: String) -> Self {
        Self {
            id, name,
            runner: None, population: Vec::new(), fitnesses: Vec::new(),
            instance: None, is_running: false,
            history: Vec::new(), current_solution_path: Vec::new(), metric_names: Vec::new(),
        }
    }

    pub fn update(&mut self, steps: usize, max_steps: usize) {
        if !self.is_running || self.runner.is_none() || self.instance.is_none() {
            return;
        }

        let instance = self.instance.as_ref().unwrap();
        let runner = self.runner.as_mut().unwrap();

        // Délégation au Runner (plus propre)
        let finished = runner.step_batch(
            &mut self.population, 
            &mut self.fitnesses, 
            instance, 
            steps, 
            max_steps
        );

        if finished {
            self.is_running = false;
        }

        // Mise à jour des logs et de la meilleure solution (only every 10 steps to avoid cluttering)
        let current_iter = runner.current_iteration();
        if current_iter % 10 == 0 {
            if let Some((path, dist, viol)) = runner.get_best_solution(&self.population, &self.fitnesses, instance) {
                self.current_solution_path = path;
                self.history.push(LogEntry {
                    iteration: current_iter,
                    current_dist: dist,
                    current_viol: viol,
                    metrics: runner.get_metrics(),
                });
            }
        }
    }

    // Le calcul du schedule est purement logique, il peut rester ici ou aller dans un module utils
    pub fn get_schedule(&self) -> Vec<VisitInfo> {
        let instance = match &self.instance {
            Some(i) => i,
            None => return Vec::new(),
        };

        if self.current_solution_path.is_empty() { return Vec::new(); }

        let mut schedule = Vec::with_capacity(self.current_solution_path.len());
        let mut current_time = 0.0;

        // Code de calcul de schedule simplifié (similaire à l'original mais plus compact)
        // ... (Logique inchangée, juste l'accès via Arc<Instance>)
        
        // Pour l'exemple, je remets le début de la logique adaptée à Arc
        let start_node = self.current_solution_path[0] as usize;
        schedule.push(VisitInfo {
            node_idx: start_node,
            arrival_time: 0.0,
            wait_time: 0.0,
            window_start: instance.windows[start_node].wstart,
            window_end: instance.windows[start_node].wend,
            violation: 0.0,
        });

        for i in 0..self.current_solution_path.len() {
             let from = self.current_solution_path[i] as usize;
             let to = self.current_solution_path[(i + 1) % self.current_solution_path.len()] as usize;
             
             let travel = instance.distance_matrix[[from, to]];
             let arrival = current_time + travel;
             let win = &instance.windows[to];
             
             let (wait, effective_arrival) = if arrival < win.wstart {
                 (win.wstart - arrival, win.wstart)
             } else {
                 (0.0, arrival)
             };
             
             let violation = if effective_arrival > win.wend { effective_arrival - win.wend } else { 0.0 };
             current_time = effective_arrival;

             schedule.push(VisitInfo {
                 node_idx: to,
                 arrival_time: arrival,
                 wait_time: wait,
                 window_start: win.wstart,
                 window_end: win.wend,
                 violation,
             });
        }

        schedule
    }
}

// --- 5. AppState (État global de l'application) ---

pub struct AppState {
    pub phase: AppPhase,
    pub instance_path: String,
    
    // Utilisation de Arc pour le stockage principal
    pub instance: Option<Arc<Instance>>, 
    pub graph_instance: Option<GraphInstance>,

    // Configuration séparée
    pub algo_type: AlgoType,
    pub evaluation_type: EvaluationType,
    pub algo_config: AlgoConfigParams,
    pub eval_config: EvalConfigParams,
    
    pub steps_per_frame: usize,

    pub runs: Vec<RunState>,
    pub selected_run_index: Option<usize>,
    pub next_run_id: usize,
    pub parallel_runs_count: usize,
    
    // View state
    pub view_mode: ViewMode,
    pub left_col_ratio: f32,
    pub right_top_ratio: f32,
    pub violation_log_scale: bool,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            phase: AppPhase::Configuration,
            instance_path: "data/inst_concours".to_string(),
            instance: None,
            graph_instance: None,
            algo_type: AlgoType::SimulatedAnnealing,
            evaluation_type: EvaluationType::Weighted,
            algo_config: AlgoConfigParams::default(),
            eval_config: EvalConfigParams::default(),
            steps_per_frame: 10000,
            runs: Vec::new(),
            selected_run_index: None,
            next_run_id: 0,
            parallel_runs_count: 1000,
            view_mode: ViewMode::Grid,
            left_col_ratio: 0.6,
            right_top_ratio: 0.5,
            violation_log_scale: false,
        }
    }

    pub fn load_instance(&mut self) {
        if std::path::Path::new(&self.instance_path).exists() {
            match load_instance(&self.instance_path) {
                Ok((inst, graph)) => {
                    // On wrap dans un Arc immédiatement
                    self.instance = Some(Arc::new(inst));
                    self.graph_instance = Some(graph);
                    self.runs.clear();
                    self.selected_run_index = None;
                }
                Err(e) => eprintln!("Error: {}", e),
            }
        }
    }

    // Helper pour créer la config spécifique aux factories en utilisant AlgoParams
    fn build_algo_config(&self) -> Result<AlgoConfig, String> {
        let params = AlgoParams::new()
            // Simulated Annealing parameters
            .initial_temperature(self.algo_config.sa_temp)
            .cooling_rate(self.algo_config.sa_cooling)
            .stopping_temperature(self.algo_config.sa_stopping)
            .acceptance_smoothing_factor(self.algo_config.sa_acceptance_smoothing)
            .initial_acceptance_rate(self.algo_config.sa_initial_acceptance_rate)
            .delta_fitness_smoothing_factor(self.algo_config.sa_delta_fitness_smoothing)
            .sa_backtracking_interval(self.algo_config.sa_backtracking_interval)
            // Genetic Algorithm parameters
            .crossover_rate(self.algo_config.ga_crossover_rate)
            .crossover_type(self.algo_config.ga_crossover_type)
            .elitism_rate(self.algo_config.ga_elitism_rate)
            .competition_participation_rate(self.algo_config.ga_competition_participation_rate)
            .competition_type(self.algo_config.ga_competition_type)
            .mutation_rate(self.algo_config.ga_mutation_rate)
            // Hill Climbing parameters
            .step(self.algo_config.hc_step)
            // Ant Colony Optimization parameters
            .evaporation_rate(self.algo_config.aco_evaporation)
            .alpha(self.algo_config.aco_alpha)
            .beta(self.algo_config.aco_beta)
            .pheromone_deposit(self.algo_config.aco_deposit)
            // Common parameters
            .neighborhood_type(self.algo_config.neighborhood)
            .local_search_type(self.algo_config.local_search_type)
            .max_steps(self.algo_config.max_steps)
            .max_iter(self.algo_config.max_steps)
            .population_size(self.algo_config.population_size);
        
        params.build_config(self.algo_type)
    }

    pub fn start_new_run(&mut self) {
        let instance_arc = match &self.instance {
            Some(i) => i.clone(), // Clone l'Arc (pas cher), pas la structure
            None => return,
        };

        let mut run = RunState::new(self.next_run_id, format!("Run {}", self.next_run_id));
        self.next_run_id += 1;

        let factory_enum = match self.build_algo_config() {
            Ok(config) => config.into_factory(),
            Err(e) => {
                eprintln!("Failed to build algorithm config: {}", e);
                return;
            }
        };
        let mut initializer = RandomInitializer; // Ou configurable via self.config

        // Construction du Runner propre
        let (runner, pop, fits) = match self.evaluation_type {
            EvaluationType::Weighted => {
                let eval = Weighted {
                    total_distance_weight: self.eval_config.total_distance_weight,
                    violation_time_weight: self.eval_config.violation_time_weight,
                    total_time_weight: self.eval_config.total_time_weight,
                    delay_weight: self.eval_config.delay_weight,
                };
                let algo = factory_enum.build(&instance_arc);
                let (p, f) = self.init_population(&mut initializer, &instance_arc, &eval);
                (Runner::Weighted(algo, eval), p, f)
            },
            EvaluationType::Lexicographic => {
                let eval = Lexicographic::new(self.eval_config.lexicographic_distance_first);
                let algo = factory_enum.build(&instance_arc);
                let (p, f) = self.init_population(&mut initializer, &instance_arc, &eval);
                (Runner::Lexicographic(algo, eval), p, f)
            },
        };

        run.metric_names = runner.get_metric_names();
        run.runner = Some(runner);
        run.population = pop;
        run.fitnesses = fits;
        run.instance = Some(instance_arc);
        run.is_running = true;
        
        self.runs.push(run);
    }

    fn init_population<E: Evaluation>(
        &self, 
        initializer: &mut RandomInitializer, 
        instance: &Instance, 
        eval: &E
    ) -> (Vec<Solution>, Vec<Fitness>) {
        let size = match self.algo_type {
            AlgoType::SimulatedAnnealing | AlgoType::HillClimbing => 1,
            _ => self.algo_config.population_size
        };

        let mut pop = Vec::with_capacity(size);
        let mut fits = Vec::with_capacity(size);

        for _ in 0..size {
            let sol = initializer.initialize(instance);
            fits.push(eval.score(instance, &sol));
            pop.push(sol);
        }
        (pop, fits)
    }

    pub fn update_solvers(&mut self) {
        let steps = self.steps_per_frame;
        let max_steps = self.algo_config.max_steps;
        
        // Utilisation de Rayon pour paralléliser l'update des runs
        self.runs.par_iter_mut().for_each(|run| {
            run.update(steps, max_steps);
        });
    }
}