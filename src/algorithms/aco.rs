use crate::shared::{Solution, Instance, Fitness, Ville};
use crate::eval;

use super::Metaheuristic;

use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use ndarray::Array2;

pub struct ACO{
    // Paramètres de l'algorithme
    evaporation_rate: f32,
    alpha: f32,
    beta: f32,
    pheromone_matrix: Array2<f32>,
    pheromone_deposit: f32,
    
    // Buffers réutilisés pour éviter les allocations
    visited_buffer: Vec<bool>,
    solution_buffer: Solution,
    desirability_buffer: Vec<f32>,
    unvisited_nodes_buffer: Vec<u32>,
    
    // RNG réutilisé
    rng: StdRng,
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

        let num_nodes = instance.size();
        let eps = 1e-6;
        let pheromone_matrix = 1.0/(instance.distance_matrix.clone() + eps);

        ACO{
            evaporation_rate,
            alpha,
            beta,
            pheromone_matrix,
            pheromone_deposit,
            visited_buffer: vec![false; num_nodes],
            solution_buffer: Vec::with_capacity(num_nodes),
            desirability_buffer: Vec::with_capacity(num_nodes),
            unvisited_nodes_buffer: Vec::with_capacity(num_nodes),
            rng: StdRng::from_os_rng(),
        }
    }
}


impl ACO {
    fn construct_solution(
        &mut self, 
        instance: &Instance
    ) -> () {
        let num_nodes = instance.size();
        
        // Réinitialiser les buffers
        self.visited_buffer.fill(false);
        self.solution_buffer.clear();

        let start_node = self.rng.random_range(0..num_nodes as u32);
        self.solution_buffer.push(start_node);
        self.visited_buffer[start_node as usize] = true;

        while self.solution_buffer.len() < num_nodes {

            let current_node = *self.solution_buffer.last().unwrap();

            let next_node = Self::compute_next_node_bfs(
                current_node, 
                num_nodes,
                &instance.distance_matrix,
                &self.pheromone_matrix,
                self.alpha,
                self.beta,
                &mut self.visited_buffer,
                &mut self.desirability_buffer,
                &mut self.unvisited_nodes_buffer,
                &mut self.rng,
            );

            self.solution_buffer.push(next_node);
            self.visited_buffer[next_node as usize] = true;
        }
    }
    
    fn compute_next_node_bfs(
        current_node: Ville, 
        num_nodes: usize,
        distance_matrix: &Array2<f32>,
        pheromone_matrix: &Array2<f32>,
        alpha: f32,
        beta: f32,
        visited_buffer: &mut Vec<bool>,
        desirability_buffer: &mut Vec<f32>,
        unvisited_nodes_buffer: &mut Vec<Ville>,
        rng: &mut StdRng,
    ) -> Ville {
        
        // Réinitialiser les buffers
        desirability_buffer.clear();
        unvisited_nodes_buffer.clear();
        
        // Calculer la désirabilité pour les nœuds non visités
        for node in 0..num_nodes {

            if !visited_buffer[node] {
                let pheromone = pheromone_matrix[[current_node as usize, node]];
                let distance = distance_matrix[[current_node as usize, node]];
                let value = pheromone.powf(alpha) * (1.0 / (distance + 1e-6)).powf(beta);
                desirability_buffer.push(value);
                unvisited_nodes_buffer.push(node as Ville);
            }
        }

        if unvisited_nodes_buffer.is_empty() {
            return current_node;
        }

        let total: f32 = desirability_buffer.iter().sum();
        if total <= 0.0 {
            let idx = rng.random_range(0..unvisited_nodes_buffer.len());
            return unvisited_nodes_buffer[idx];
        }

        // Sélection par roulette wheel
        let mut pick = rng.random_range(0.0..total);
        for (idx, &value) in desirability_buffer.iter().enumerate() {
            pick -= value;
            if pick <= 0.0 {
                return unvisited_nodes_buffer[idx];
            }
        }
        *unvisited_nodes_buffer
        .last()
        .unwrap()
    }
}


impl Metaheuristic for ACO {
    fn step<Eval: eval::Evaluation>(
        &mut self,
        population: &mut [Solution],
        fitness: &mut [Fitness],
        instance: &Instance,
        metric_fn: &Eval
    ) -> () {
        // Évaporation des phéromones
        self.pheromone_matrix *= 1.0 - self.evaporation_rate;

        // Construire les solutions pour chaque fourmi
        for i in 0..population.len() {

            self.construct_solution(instance);
            let fit = metric_fn.score(instance, &self.solution_buffer);
            
            population[i].clone_from_slice(&self.solution_buffer[..]);
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