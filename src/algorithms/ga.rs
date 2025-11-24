use std::collections::{HashMap, HashSet};

use crate::shared::{Solution, Instance, Fitness};
use crate::eval;

use super::Metaheuristic;

use rand::Rng;

pub enum CompetitionType {
    Tournament,
    Roulette,
}

pub enum CrossType {
    PMX,
    OX
}

pub struct GeneticAlgorithm{

    // Paramètres de l'algorithme
    crossover_rate: f32,
    competition_participation_count: usize,
    elitism_count: usize,
    crossover_type: CrossType,
    competition_type: CompetitionType,
    
    // Buffers réutilisés, alloués UNE FOIS
    participants_buffer: Vec<usize>,
    best_idx_buffer: Vec<usize>,
    new_population_buffer: Vec<Solution>,
}

impl GeneticAlgorithm {
    pub fn new(
        crossover_rate: f32,
        crossover_type: CrossType,
        elitism_rate: f32,
        competition_participation_rate: f32,
        competition_type: CompetitionType,
        max_population_size: usize,
    ) -> Self {

        let elitism_count = (elitism_rate * max_population_size as f32) as usize;
        let competition_participation_count = (competition_participation_rate * max_population_size as f32) as usize;
        
        GeneticAlgorithm {
            crossover_rate,
            crossover_type,
            elitism_count,
            competition_participation_count,
            competition_type,
            participants_buffer: Vec::with_capacity(competition_participation_count),
            best_idx_buffer: Vec::with_capacity(elitism_count),
            new_population_buffer: Vec::with_capacity(max_population_size),
        }
    }
}

impl GeneticAlgorithm {
    fn tournament_selection(fitness: &[Fitness], participants: &[usize]) -> usize {

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
    fn roulette_selection(fitness: &[Fitness], participants: &[usize]) -> usize {

        // Roulette wheel selection logic
        let total_fitness: f32 = participants.iter().map(|&idx| fitness[idx]).sum();
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
    fn select_best(&mut self, fitness: &[Fitness]) -> () {
        
        let elitism_count = self.best_idx_buffer.len();

        for i in 0..elitism_count {
            self.best_idx_buffer[i] = i
        };
        
        utils::argsort_f32(&fitness[0..elitism_count], &mut self.best_idx_buffer[..]);

        for i in elitism_count..fitness.len() {
            let v = fitness;
            let target = fitness[i];
        }
    }
}


impl GeneticAlgorithm {
    fn select_particiants(&mut self, population: &[Solution]) -> () {

        // Logic to select participants for selection round
        let mut rng = rand::rng();
        let mut cpt= 0;
        let mut participants_check = HashSet::new();

        while cpt < self.competition_participation_count {
            let ind = rng.random_range(0..population.len());
            if !participants_check.contains(&ind) {
                participants_check.insert(ind);
                self.participants_buffer[cpt] = ind;
                cpt += 1;
            }
        }
    }
}

impl GeneticAlgorithm {
    fn selection_round(&mut self, population: &[Solution], fitness: &[Fitness]) -> usize {

        // Selection logic based on competition type
        self.select_particiants(population);
        match self.competition_type {
            CompetitionType::Tournament => {
                Self::tournament_selection(fitness, &self.participants_buffer)
            }
            CompetitionType::Roulette => {
                Self::roulette_selection(fitness, &self.participants_buffer)
            }
        }
    }
}

impl GeneticAlgorithm {
    fn ox_crossover(&mut self, parent1: &Solution, parent2: &Solution, idx: usize) -> () {

        // Order Crossover (OX) logic
        let size = parent1.len();
        let mut rng = rand::rng();
        
        let (left_pop_buffer, right_pop_buffer) = self.new_population_buffer.split_at_mut(idx+1);
        let child1_route = &mut left_pop_buffer[idx];
        let child2_route = &mut right_pop_buffer[0];
        
        let mut child1_route_hash: HashSet<u32> = HashSet::new();
        let mut child2_route_hash: HashSet<u32> = HashSet::new();

        let start = rng.random_range(0..size);
        let end = rng.random_range(start..size);

        for i in start..end {
            child1_route[i] = parent1[i];
            child2_route[i] = parent2[i];

            child1_route_hash.insert(parent1[i]);
            child2_route_hash.insert(parent2[i]);
        }
        

        let mut current_pos1 = end % size;
        let mut current_pos2 = end % size;

        for i in 0..size {
            let idx = (end + i) % size;

            if !child1_route_hash.contains(&parent2[idx]) {
                child1_route[current_pos1] = parent2[idx];
                current_pos1 = (current_pos1 + 1) % size;
            }

            if !child2_route_hash.contains(&parent1[idx]) {
                child2_route[current_pos2] = parent1[idx];
                current_pos2 = (current_pos2 + 1) % size;
            }
        }
    }
}

impl GeneticAlgorithm {
    pub fn pmx_resolve_mapping(
        map: &HashMap<u32, u32>,
        start: u32,
    ) -> u32 {
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
    fn pmx_crossover(&mut self, parent1: &Solution, parent2: &Solution, idx: usize) -> () {
        // PMX crossover logic
        let size = parent1.len();
        let mut rng = rand::rng();
        // obtenir deux références mutuelles disjointes
        let (left_pop_buffer, right_pop_buffer) = self.new_population_buffer.split_at_mut(idx+1);
        let child1_route = &mut left_pop_buffer[idx];
        let child2_route = &mut right_pop_buffer[0];

        let mut child1_route_map = HashMap::new();
        let mut child2_route_map = HashMap::new();

        let start = rng.random_range(0..size);
        let end = rng.random_range(start..size);

        for i in start..end {
            child1_route[i] = parent1[i];
            child2_route[i] = parent2[i];

            child1_route_map.insert(parent1[i], parent2[i]);
            child2_route_map.insert(parent2[i], parent1[i]);
        }

        let mut current_pos1 = end % size;
        let mut current_pos2 = end % size;

        for i in 0..size {
            let idx = (end + i) % size;

            if child1_route_map.contains_key(&parent2[idx]) {
                let mapped_value = Self::pmx_resolve_mapping(&child1_route_map, parent2[idx]);
                child1_route[current_pos1] = mapped_value;
                current_pos1 = (current_pos1 + 1) % size;
                }
            

            if child2_route_map.contains_key(&parent1[idx]) {
                let mapped_value = Self::pmx_resolve_mapping(&child2_route_map, parent1[idx]);
                child2_route[current_pos2] = mapped_value;
                current_pos2 = (current_pos2 + 1) % size;
            }
        }
    }
}

impl GeneticAlgorithm {
    fn crossover(&mut self, parent1: &Solution, parent2: &Solution, cpt: usize) -> () {
        match self.crossover_type {
            CrossType::OX => {
                self.ox_crossover(parent1, parent2, cpt)
            }
            CrossType::PMX => {
                self.pmx_crossover(parent1, parent2, cpt)
            }
        }
    }
}

impl GeneticAlgorithm {
    fn select_parents(&mut self, population: &[Solution], fitness: &[Fitness]) -> (usize, usize) {
        let parent1 = self.selection_round(population, fitness);
        let parent2 = self.selection_round(population, fitness);
        (parent1, parent2)
    }
}

impl Metaheuristic for GeneticAlgorithm {
    fn step<Eval: eval::Evaluation>(
        &mut self,
        population: &mut [Solution],
        fitness: &mut [Fitness],
        instance: &Instance,
        evaluation: &Eval
    ) -> () {

        let pop_size = population.len();

        // Selects the best individuals for elitism, puts their indices in best_idx_buffer to avoid
        // reallocating memory each generation
        self.select_best(fitness);

        let mut cpt = 0;

        // Adds the best individuals to the new population
        for idx in self.best_idx_buffer.iter() {
            self.new_population_buffer[cpt] = population[*idx].clone();
            cpt += 1;
        }

        // Generates new individuals through selection, crossover, and mutation
        while cpt < pop_size {
            let (parent1_idx, parent2_idx) = self.select_parents(population, fitness);
            let parent1 = &population[parent1_idx];
            let parent2 = &population[parent2_idx];

            let rand = rand::rng().random_range(0.0..1.0);

            if rand < self.crossover_rate {

                // Crossover, produces two children that are added to the new population buffer in place
                self.crossover(parent1, parent2, cpt)
            } else {

                // No crossover, simply clone the parents into the new population buffer
                self.new_population_buffer[cpt].clone_from_slice(&parent1[..]);
                self.new_population_buffer[cpt + 1].clone_from_slice(&parent2[..]);
            }; cpt += 2;
        }

        // Replace old population with new population
        population.clone_from_slice(&self.new_population_buffer[..]);

        // Re-evaluate fitness of the new population
        for i in self.best_idx_buffer.len()..pop_size {
            fitness[i] = evaluation.score(instance, &population[i]);
        }
    }
}