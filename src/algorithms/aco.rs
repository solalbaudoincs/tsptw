use crate::shared::{Solution, Instance, Fitness};
use crate::eval;

use super::MetaHeuristic;

use rand::Rng;
use ndarray::Array2;

pub struct ACO{
    // Paramètres de l'algorithme
    num_ants: usize,
    evaporation_rate: f32,
    alpha: f32,
    beta: f32,
    pheromone_matrix: Array2<f32>,
    max_iterations: usize,
    pheromone_deposit: f32,
}

impl ACO {
    pub fn new(
        instance: &Instance,
        evaporation_rate: f32,
        alpha: f32,
        beta: f32,
        max_iterations: usize,
        pheromone_deposit: f32
    ) -> Self {

        let num_ants = instance.size();
        let eps = 1e-6;
        let pheromone_matrix = 1.0/(instance.distance_matrix.clone() + eps);

        ACO{
            num_ants,
            evaporation_rate,
            alpha,
            beta,
            pheromone_matrix,
            max_iterations,
            pheromone_deposit
        }
    }
}


impl ACO {
    fn compute_next_node(&self, current_node: u32, unvisited: &[u32], instance: &Instance) -> u32 {
        if unvisited.is_empty() {
            return current_node;
        }

        let dist_matrix = &instance.distance_matrix;
        
        // Calculer la désirabilité uniquement pour les nœuds non visités
        let mut desirability: Vec<f32> = Vec::with_capacity(unvisited.len());
        for &next_node in unvisited {
            let pheromone = self.pheromone_matrix[[current_node as usize, next_node as usize]];
            let distance = dist_matrix[[current_node as usize, next_node as usize]];
            let value = pheromone.powf(self.alpha) * (1.0 / (distance + 1e-6)).powf(self.beta);
            desirability.push(value);
        }

        let total: f32 = desirability.iter().sum();
        if total <= 0.0 {
            // Si toutes les probabilités sont nulles, choisir aléatoirement
            let idx = rand::rng().random_range(0..unvisited.len());
            return unvisited[idx];
        }

        // Sélection par roulette wheel sur les nœuds non visités
        let mut pick = rand::rng().random_range(0.0..total);
        for (idx, &value) in desirability.iter().enumerate() {
            pick -= value;
            if pick <= 0.0 {
                return unvisited[idx];
            }
        }
        *unvisited.last().unwrap()
    }

    fn construct_solution(&self, instance: &Instance) -> Solution {
        let mut rng = rand::rng();
        let num_nodes = instance.size() as u32;
        let mut unvisited: Vec<u32> = (0..num_nodes).collect();
        let mut solution: Solution = Vec::with_capacity(num_nodes as usize);

        let start_node = rng.random_range(0..num_nodes);
        solution.push(start_node);
        unvisited.retain(|&x| x != start_node);

        while !unvisited.is_empty() {
            let current_node = *solution.last().unwrap();
            let next_node = self.compute_next_node(current_node, &unvisited, instance);
            solution.push(next_node);
            unvisited.retain(|&x| x != next_node);
        }

        solution
    }
}


impl MetaHeuristic for ACO {
    fn step<Eval: eval::Metric>(
        &mut self,
        population: &mut [Solution],
        fitness: &mut [Fitness],
        instance: &Instance,
        metric_fn: &Eval
    ) -> () {
        // Évaporation des phéromones
        self.pheromone_matrix *= 1.0 - self.evaporation_rate;

        // Construire les solutions pour chaque fourmi
        for i in 0..self.num_ants.min(population.len()) {
            let solution = self.construct_solution(instance);
            let fit = metric_fn.score(&solution, instance);
            
            population[i] = solution;
            fitness[i] = fit;

            // Dépôt de phéromones sur le chemin parcouru
            let pheromone_amount = self.pheromone_deposit / (fit + 1e-6);
            for j in 0..population[i].len() {
                let from = population[i][j] as usize;
                let to = population[i][(j + 1) % population[i].len()] as usize;
                self.pheromone_matrix[[from, to]] += pheromone_amount;
                self.pheromone_matrix[[to, from]] += pheromone_amount;
            }
        }
    }
}