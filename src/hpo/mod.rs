mod grid_search;

mod bayesian_optimizer;

use std::any::Any;

struct HyperparameterOptimizer {
    // Placeholder for hyperparameter optimizer structure
    grid: Vec<Vec<Vec<f32>>>,
    objective_function: fn(&dyn Any) -> f32,
    
}