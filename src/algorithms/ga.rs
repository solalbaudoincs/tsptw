use crate::shared::{Solution, Instance, Fitness, Ville};
use crate::eval;

use super::Metaheuristic;

use rand::Rng;
use rand::rngs::StdRng;
use rand::SeedableRng;

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
    solution_size: usize,
    crossover_rate: f32,
    competition_participation_count: usize,
    elitism_count: usize,
    crossover_type: CrossType,
    competition_type: CompetitionType,
    
    rng: StdRng,

    // Buffers réutilisés, alloués UNE FOIS
    participants_buffer: Vec<usize>,
    best_idx_buffer: Vec<usize>,
    new_population_buffer: Vec<Solution>,
    child1_visited_buffer: Vec<bool>,
    child2_visited_buffer: Vec<bool>,
    child1_mapping_buffer: Vec<u32>,
    child2_mapping_buffer: Vec<u32>,
}

impl GeneticAlgorithm {
    pub fn new(
        instance: &Instance,
        crossover_rate: f32,
        crossover_type: CrossType,
        elitism_rate: f32,
        competition_participation_rate: f32,
        competition_type: CompetitionType,
        max_population_size: usize,
    ) -> Self {

        let solution_size = instance.size();
        let elitism_count = (elitism_rate * max_population_size as f32) as usize;
        let competition_participation_count = (competition_participation_rate * max_population_size as f32) as usize;
        
        GeneticAlgorithm {
            solution_size,
            crossover_rate,
            crossover_type,
            elitism_count,
            competition_participation_count,

            rng: StdRng::from_os_rng(),

            competition_type,
            participants_buffer: Vec::with_capacity(competition_participation_count),
            best_idx_buffer: Vec::with_capacity(max_population_size),
            new_population_buffer: Vec::with_capacity(max_population_size),
            child1_visited_buffer: vec![false; solution_size],
            child2_visited_buffer: vec![false; solution_size],
            child1_mapping_buffer: vec![u32::MAX; solution_size],
            child2_mapping_buffer: vec![u32::MAX; solution_size],
        }
    }
}

impl GeneticAlgorithm {
    fn tournament_selection(&mut self, fitness: &[Fitness]) -> usize {

        // Tournament selection logic
        let best_idx = self.rng.random_range(0..self.competition_participation_count);
        let mut participant_best_idx = self.participants_buffer[best_idx];

        for _ in 0..self.competition_participation_count {
            let ind = self.rng.random_range(0..self.competition_participation_count);
            let participant_ind = self.participants_buffer[ind];
            if fitness[participant_ind] > fitness[participant_best_idx] {
                participant_best_idx = participant_ind;
            }
        }
        participant_best_idx
    }

    fn roulette_selection(&mut self, fitness: &[Fitness]) -> usize {

        // Roulette wheel selection logic
        let total_fitness: f32 = self.participants_buffer
        .iter()
        .map(|&idx| fitness[idx])
        .sum();
        let mut pick = self.rng.random_range(0.0..total_fitness);
        
        for idx in 0..self.competition_participation_count {
            pick -= fitness[self.participants_buffer[idx]];
            if pick <= 0.0 {
                return self.participants_buffer[idx];
            }
        }
        self.participants_buffer
        .last()
        .cloned()
        .unwrap()
    }


    fn select_best(&mut self, fitness: &[Fitness]) -> () {
        
        // Initialize indices
        for i in 0..self.solution_size {
            self.best_idx_buffer[i] = i;
        }
        
        // Use select_nth_unstable to partition and find the best elitism_count indices
        // We want the smallest fitness values (best solutions), so we partition at elitism_count - 1
        self.best_idx_buffer.select_nth_unstable_by(
            self.elitism_count - 1, 
            |&a, &b| {
            fitness[a]
            .partial_cmp(&fitness[b])
            .unwrap_or(std::cmp::Ordering::Equal)
        }
    );
        
        // The first elitism_count elements are now the best indices
        // No need for additional sorting since select_nth_unstable partitions correctly
    }

    fn select_particiants(&mut self, population: &[Solution]) -> () {

        // Selection aléatoire uniforme avec roulette simple
        for i in 0..self.competition_participation_count {
            let mut pick = self.rng.random_range(0..population.len() - i);
            
            // Parcourir la population et décrémenter pick jusqu'à 0
            for j in 0..population.len() {
                // Si l'individu n'est pas déjà sélectionné
                let mut already_selected = false;
                for k in 0..i {
                    if self.participants_buffer[k] == j {
                        already_selected = true;
                        break;
                    }
                }
                
                if !already_selected {
                    if pick == 0 {
                        self.participants_buffer[i] = j;
                        break;
                    }
                    pick -= 1;
                }
            }
        }
    }

    fn selection_round(&mut self, population: &[Solution], fitness: &[Fitness]) -> usize {

        // Selection logic based on competition type
        self.select_particiants(population);
        match self.competition_type {
            CompetitionType::Tournament => {
                self.tournament_selection(fitness)
            }
            CompetitionType::Roulette => {
                self.roulette_selection(fitness)
            }
        }
    }
}

impl GeneticAlgorithm {
    fn ox_crossover(
        parent1: &[Ville],
        parent2: &[Ville],
        child1_route: &mut Solution,
        child2_route: &mut Solution,
        child1_visited_buffer: &mut Vec<bool>,
        child2_visited_buffer: &mut Vec<bool>,
        rng: &mut StdRng,
    ) -> () {

        // Order Crossover (OX) logic
        let size = parent1.len();
        
        child1_visited_buffer.fill(false);
        child2_visited_buffer.fill(false);

        let start = rng.random_range(0..size);
        let end = rng.random_range(start..size);

        for i in start..end {
            child1_route[i] = parent1[i];
            child2_route[i] = parent2[i];

            child1_visited_buffer[parent1[i] as usize] = true;
            child2_visited_buffer[parent2[i] as usize] = true;
        }

        let mut current_pos1 = end % size;
        let mut current_pos2 = end % size;

        for i in 0..size {
            let idx = (end + i) % size;

            if !child1_visited_buffer[parent2[idx] as usize] {
                child1_route[current_pos1] = parent2[idx];
                current_pos1 = (current_pos1 + 1) % size;
            }

            if !child2_visited_buffer[parent1[idx] as usize] {
                child2_route[current_pos2] = parent1[idx];
                current_pos2 = (current_pos2 + 1) % size;
            }
        }
    }
}

impl GeneticAlgorithm {

    fn pmx_resolve_mapping(
        mapping: &[u32], 
        start: u32
    ) -> u32 {
        let mut current = start;
        let max_iter = mapping.len() + 1;
        
        for _ in 0..max_iter {
            let mapped = mapping[current as usize];
            if mapped == u32::MAX {
                return current;
            }
            current = mapped;
        }
        current
    }

    fn pmx_crossover(
        parent1: &[Ville], 
        parent2: &[Ville], 
        child1_route: &mut Solution,
        child2_route: &mut Solution,
        child1_mapping_buffer: &mut Vec<u32>,
        child2_mapping_buffer: &mut Vec<u32>,
        child1_visited_buffer: &mut Vec<bool>,
        child2_visited_buffer: &mut Vec<bool>,
        rng: &mut StdRng,
    ) -> () {

        // PMX crossover logic
        let size = parent1.len();
        
        let start = rng.random_range(0..size);
        let end = rng.random_range(start..size);

        // Copier le segment et créer le mapping
        for i in start..end {
            let val1 = parent1[i];
            let val2 = parent2[i];
            
            child1_route[i] = val1;
            child2_route[i] = val2;
            
            child1_visited_buffer[val1 as usize] = true;
            child2_visited_buffer[val2 as usize] = true;
            
            // Créer le mapping: dans child1, val1 mappe vers val2 (et vice versa)
            child1_mapping_buffer[val1 as usize] = val2;
            child2_mapping_buffer[val2 as usize] = val1;
        }
        
        // Remplir le reste en utilisant le mapping PMX
        for i in 0..size {
            
            if i >= start && i < end {
                continue; // Skip le segment déjà copié
            }

            // Pour child1, prendre de parent2
            let mut val2 = parent2[i];
            if child1_visited_buffer[val2 as usize] {
                // Suivre le mapping jusqu'à trouver une valeur non utilisée
                val2 = Self::pmx_resolve_mapping(&child1_mapping_buffer, val2);
            }
            child1_route[i] = val2;
            child1_visited_buffer[val2 as usize] = true;
            
            // Pour child2, prendre de parent1
            let mut val1 = parent1[i];
            if child2_visited_buffer[val1 as usize] {
                // Suivre le mapping jusqu'à trouver une valeur non utilisée
                val1 = Self::pmx_resolve_mapping(&child2_mapping_buffer, val1);
            }
            child2_route[i] = val1;
            child2_visited_buffer[val1 as usize] = true;
        }
    }
}   


impl GeneticAlgorithm {
    fn crossover(&mut self, parent1: &[Ville], parent2: &[Ville], cpt: usize) -> () {
        
        let (left_pop_buffer, right_pop_buffer) = self.new_population_buffer.split_at_mut(cpt+1);
        let child1_route = &mut left_pop_buffer[cpt];
        let child2_route = &mut right_pop_buffer[0];

        match self.crossover_type {
            CrossType::OX => {
                Self::ox_crossover(
                    parent1, 
                    parent2, 
                    child1_route,
                    child2_route,
                    &mut self.child1_visited_buffer,
                    &mut self.child2_visited_buffer,
                    &mut self.rng,
                )
            }
            CrossType::PMX => {
                Self::pmx_crossover(
                    parent1, 
                    parent2, 
                    child1_route,
                    child2_route,
                    &mut self.child1_mapping_buffer,
                    &mut self.child2_mapping_buffer,
                    &mut self.child1_visited_buffer,
                    &mut self.child2_visited_buffer,
                    &mut self.rng,
                )
            }
        }
    }

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

        let best_idx = &self.best_idx_buffer[..self.elitism_count];

        let mut cpt = 0;

        // Adds the best individuals to the new population
        for idx in best_idx.iter() {
            self.new_population_buffer[cpt].clone_from_slice(&population[*idx][..]);
            cpt += 1;
        }

        // Generates new individuals through selection, crossover, and mutation
        while cpt < pop_size {
            let (parent1_idx, parent2_idx) = self.select_parents(population, fitness);
            let parent1 = &population[parent1_idx];
            let parent2 = &population[parent2_idx];

            let rand = self.rng.random_range(0.0..1.0);

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
        for i in self.elitism_count..pop_size {
            fitness[i] = evaluation.score(instance, &population[i]);
        }
    }
}