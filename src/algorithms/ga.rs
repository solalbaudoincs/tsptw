use std::collections::{HashMap, HashSet};

use crate::problem::{Solution, Instance, Population, Evaluation};
use crate::problem::instance::Node;
use crate::problem::evaluation::{Fitness, Fitnesses};
use crate::utils::general::View;
use crate::utils;

use super::Metaheuristic;

use rand::Rng;

enum CompetitionType {
    Tournament,
    Roulette,
}

enum CrossType {
    PMX,
    OX
}

pub struct GeneticAlgorithm{

    // Paramètres de l'algorithme
    mutation_rate: f64,
    crossover_rate: f64,
    crossover_type: CrossType,
    elitism_rate: f64,
    comp_participation: f64,
    comp_type: CompetitionType,
    
    // Buffers réutilisés, alloués UNE FOIS
    participants_buf: View,
    best_idx_buf: View,
    new_population_buf: Vec<Solution>,
    child1_buf: View,
    child2_buf: View,
}

impl GeneticAlgorithm {
    pub fn new(
        mutation_rate: f64,
        crossover_rate: f64,
        crossover_type: CrossType,
        elitism_rate: f64,
        comp_participation: f64,
        comp_type: CompetitionType,
        max_population_size: usize,
        solution_size: usize,
    ) -> Self {
        GeneticAlgorithm {
            mutation_rate,
            crossover_rate,
            crossover_type,
            elitism_rate,
            comp_participation,
            comp_type,
            participants_buf: Vec::with_capacity((comp_participation * max_population_size as f64) as usize),
            best_idx_buf: Vec::with_capacity((max_population_size as f64) as usize),
            new_population_buf: Vec::with_capacity(max_population_size),
            child1_buf: Vec::with_capacity(solution_size),
            child2_buf: Vec::with_capacity(solution_size),
        }
    }
}

impl GeneticAlgorithm {
    fn tournament_selection(fitness: &Vec<Fitness>, participants: &View) -> usize {
        // Tournament selection logic
        let mut rng = rand::rng();
        let mut best_idx = rng.random_range(0..participants.len());
        best_idx = participants[best_idx];

        for _ in 0..participants.len() {
            let mut ind = rng.random_range(0..participants.len());
            ind = participants[ind];
            if fitness[ind] > fitness[best_idx] {
                best_idx = participants[ind];
            }
        }
        best_idx
    }
}

impl GeneticAlgorithm {
    fn roulette_selection(fitness: &Vec<Fitness>, participants: &View) -> usize {
        // Roulette wheel selection logic
        let total_fitness: f64 = participants.iter().map(|&idx| fitness[idx]).sum();
        let mut rng = rand::rng();
        let mut pick = rng.random_range(0.0..total_fitness);
        
        for idx in 0..participants.len() {
            pick -= fitness[participants[idx]];
            if pick <= 0.0 {
                return participants[idx];
            }
        }
        participants.last().cloned().unwrap()
    }
}



impl GeneticAlgorithm {
    fn select_best<'a>(best_idx_buffer: &'a mut View, n_solution: usize, fitness: &Vec<Fitness>) -> &'a [usize] {
        utils::general::argsort_f64(fitness, best_idx_buffer);
        let pop_size = fitness.len();
        &best_idx_buffer[pop_size - n_solution..]
    }
}

impl GeneticAlgorithm {
    fn select_particiants(&mut self, population: &Vec<Solution>) -> () {
        // Logic to select participants for selection round
        let mut rng = rand::rng();
        let num_participants = (self.comp_participation * population.len() as f64) as usize;
        let mut participants: View = Vec::with_capacity(num_participants);
        
        while participants.len() < num_participants {
            let ind = rng.random_range(0..population.len());
            if !participants.contains(&ind) {
                participants.push(ind);
            }
        }
        self.participants_buf.copy_from_slice(&participants);
    }
}

impl GeneticAlgorithm {
    fn selection_round(&mut self, population: &Vec<Solution>, fitness: &Vec<Fitness>) -> usize {
        // Selection logic based on competition type
        self.select_particiants(population);
        match self.comp_type {
            CompetitionType::Tournament => {
                Self::tournament_selection(fitness, &self.participants_buf)
            }
            CompetitionType::Roulette => {
                Self::roulette_selection(fitness, &self.participants_buf)
            }
        }
    }
}

impl GeneticAlgorithm {
    fn ox_crossover(&self, parent1: &Solution, parent2: &Solution) -> (Solution, Solution) {
        // Order Crossover (OX) logic
        let size = parent1.len();
        let mut rng = rand::rng();
        let mut child1_route = vec![None; size];
        let mut child2_route = vec![None; size];

        let start = rng.random_range(0..size);
        let end = rng.random_range(start..size);

        for i in start..end {
            child1_route[i] = Some(parent1[i]);
            child2_route[i] = Some(parent2[i]);
        }
        
        let child1_route_hash: HashSet<Option<Node>> = child1_route.iter().cloned().collect();
        let child2_route_hash: HashSet<Option<Node>> = child2_route.iter().cloned().collect();

        let mut current_pos1 = end % size;
        let mut current_pos2 = end % size;

        for i in 0..size {
            let idx = (end + i) % size;

            if !child1_route_hash.contains(&Some(parent2[idx])) {
                child1_route[current_pos1] = Some(parent2[idx]);
                current_pos1 = (current_pos1 + 1) % size;
            }

            if !child2_route_hash.contains(&Some(parent1[idx])) {
                child2_route[current_pos2] = Some(parent1[idx]);
                current_pos2 = (current_pos2 + 1) % size;
            }
        }

        let child1 = child1_route.into_iter().map(|x| x.unwrap()).collect() ;
        let child2 = child2_route.into_iter().map(|x| x.unwrap()).collect() ;

        (child1, child2)
    }
}

impl GeneticAlgorithm {
    pub fn pmx_resolve_mapping(
        map: &HashMap<Option<Node>, Option<Node>>,
        start: Option<Node>,
    ) -> Option<Node> {
        let mut current = start;
        let max_iter = map.len().saturating_add(1);

        for _ in 0..max_iter {
            match map.get(&current) {
                Some(mapped) => {
                    let mapped_value = *mapped;
                    if !map.contains_key(&mapped_value) {
                        return mapped_value;
                    } else {
                        current = mapped_value;
                    }
                }
                None => return current,
            }
        } current
    }
}

impl GeneticAlgorithm {
    fn pmx_crossover(&self, parent1: &Solution, parent2: &Solution) -> (Solution, Solution) {
        // PMX crossover logic
        let size = parent1.len();
        let mut rng = rand::rng();
        let mut child1_route = vec![None; size];
        let mut child2_route = vec![None; size];

        let mut child1_route_map: HashMap<Option<Node>, Option<Node>> = HashMap::new();
        let mut child2_route_map: HashMap<Option<Node>, Option<Node>> = HashMap::new();

        let start = rng.random_range(0..size);
        let end = rng.random_range(start..size);

        for i in start..end {
            child1_route[i] = Some(parent1[i]);
            child2_route[i] = Some(parent2[i]);

            child1_route_map.insert(Some(parent1[i]), Some(parent2[i]));
            child2_route_map.insert(Some(parent2[i]), Some(parent1[i]));
        }

        let mut current_pos1 = end % size;
        let mut current_pos2 = end % size;

        for i in 0..size {
            let idx = (end + i) % size;

            if child1_route_map.contains_key(&Some(parent2[idx])) {
                let mapped_value = Self::pmx_resolve_mapping(&child1_route_map, Some(parent2[idx]));
                child1_route[current_pos1] = mapped_value;
                current_pos1 = (current_pos1 + 1) % size;
                }
            

            if child2_route_map.contains_key(&Some(parent1[idx])) {
                let mapped_value = Self::pmx_resolve_mapping(&child2_route_map, Some(parent1[idx]));
                child2_route[current_pos2] = mapped_value;
                current_pos2 = (current_pos2 + 1) % size;
            }
        }

        let child1 = child1_route.into_iter().map(|x| x.unwrap()).collect() ;
        let child2 = child2_route.into_iter().map(|x| x.unwrap()).collect() ;

        (child1, child2)
    }
}

impl GeneticAlgorithm {
    fn crossover(&self, parent1: &Solution, parent2: &Solution) -> (Solution, Solution) {
        match self.crossover_type {
            CrossType::OX => {
                self.ox_crossover(parent1, parent2)
            }
            CrossType::PMX => {
                self.pmx_crossover(parent1, parent2)
            }
        }
    }
}

impl GeneticAlgorithm {
    fn select_parents(&mut self, population: &Vec<Solution>, fitness: &Vec<Fitness>) -> (usize, usize) {
        let parent1 = self.selection_round(population, fitness);
        let parent2 = self.selection_round(population, fitness);
        (parent1, parent2)
    }
}

impl Metaheuristic for GeneticAlgorithm {
    fn step<Eval: Evaluation>(
        &mut self,
        population: &mut Population,
        fitness: &mut Fitnesses,
        instance: &Instance,
        evaluation: &Eval
    ) -> () {

        let pop_size = population.len();
        let elitism_count = (self.elitism_rate * pop_size as f64) as usize;

        // Sélection des meilleurs individus pour l'élitisme
        let best_idx = Self::select_best(&mut self.best_idx_buf, elitism_count, fitness);

        let mut cpt = 0;

        // Ajout des meilleurs individus à la nouvelle population
        for idx in best_idx {
            self.new_population_buf[cpt] = population[*idx].clone();
            cpt += 1;
        }

        // Génération des nouveaux individus par sélection, croisement et mutation
        while cpt < pop_size {
            let (parent1_idx, parent2_idx) = self.select_parents(population, fitness);
            let parent1 = &population[parent1_idx];
            let parent2 = &population[parent2_idx];

            let rand = rand::rng().random_range(0.0..1.0);

            let (child1, child2) = if rand < self.crossover_rate {
                self.crossover(parent1, parent2)
            } else {
                (parent1.clone(), parent2.clone())
            };

            self.new_population_buf[cpt] = child1;
            self.new_population_buf[cpt + 1] = child2;
            cpt += 2;
        }

        population.clone_from(&self.new_population_buf);
        for i in 0..pop_size {
            fitness[i] = evaluation.score(instance, &population[i]) as f64; // Invalidate fitness
        }
    }
}