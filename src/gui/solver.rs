use std::collections::HashMap;
use crate::algorithms::Metaheuristic;
use crate::neighborhood::NeighborFn;

use crate::shared::{Instance, Solution, Fitness};
use crate::eval::{Evaluation, utils};


pub trait Solver: Send + Sync {
    fn step(&mut self);
    fn get_metrics(&self) -> HashMap<String, f32>;
    fn get_metric_names(&self) -> Vec<String>;
    fn get_best_solution(&self) -> Option<(Vec<u32>, f32, f32)>; // path, distance, violation
    fn get_iteration(&self) -> usize;
}

pub struct ConcreteSolver<Algo, Eval, N> {
    pub algo: Algo,
    pub eval: Eval,
    pub neighbor: N,
    pub instance: Instance,
    pub population: Vec<Solution>,
    pub fitnesses: Vec<Fitness>,
    pub iteration: usize,
}

impl<Algo, Eval, N> ConcreteSolver<Algo, Eval, N> 
where 
    Algo: Metaheuristic,
    Eval: Evaluation,
    N: NeighborFn
{
    pub fn new(
        algo: Algo,
        eval: Eval,
        neighbor: N,
        instance: Instance,
        population: Vec<Solution>,
        fitnesses: Vec<Fitness>,
    ) -> Self {
        Self {
            algo,
            eval,
            neighbor,
            instance,
            population,
            fitnesses,
            iteration: 0,
        }
    }
}

impl<Algo, Eval, N> Solver for ConcreteSolver<Algo, Eval, N>
where
    Algo: Metaheuristic + Send + Sync,
    Eval: Evaluation + Send + Sync,
    N: NeighborFn + Send + Sync,
{
    fn step(&mut self) {
        self.algo.step(
            &mut self.population,
            &mut self.fitnesses,
            &mut self.neighbor,
            &self.instance,
            &self.eval,
        );
        self.iteration += 1;
    }

    fn get_metrics(&self) -> HashMap<String, f32> {
        self.algo.get_metrics()
    }

    fn get_metric_names(&self) -> Vec<String> {
        self.algo.get_metric_names()
    }

    fn get_best_solution(&self) -> Option<(Vec<u32>, f32, f32)> {
        if self.population.is_empty() {
            return None;
        }

        let mut best_idx = 0;
        for idx in 1..self.population.len() {
             if self.eval.compare(&self.instance, &self.population[idx], &self.population[best_idx]) == std::cmp::Ordering::Less {
                best_idx = idx;
            }
        }
        
        let best_sol = &self.population[best_idx];
        let eval = utils::run_solution(&self.instance, best_sol);
        let (dist, viol) = (eval.total_distance, eval.violation_time);
        
        Some((best_sol.clone(), dist as f32, viol as f32))
    }

    fn get_iteration(&self) -> usize {
        self.iteration
    }
}
