use crate::eval::Evaluation;
use crate::shared::{Fitness, Instance, Solution, Ville};

use super::{Metaheuristic, LocalSearch};

use rand::Rng;
use rand::SeedableRng;
use rand::rngs::StdRng;
use rand::seq::SliceRandom;

#[derive(Clone, Copy, PartialEq)]
pub enum CompetitionType {
    Tournament,
    Roulette,
}

impl Default for CompetitionType {
    fn default() -> Self {
        CompetitionType::Tournament
    }
}

impl CompetitionType {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "Tournament" => Some(CompetitionType::Tournament),
            "Roulette" => Some(CompetitionType::Roulette),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum CrossoverType {
    PMX,
    OX,
}

impl Default for CrossoverType {
    fn default() -> Self {
        CrossoverType::PMX
    }
}

impl CrossoverType {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "PMX" => Some(CrossoverType::PMX),
            "OX" => Some(CrossoverType::OX),
            _ => None,
        }
    }
}

/// Algorithme génétique pour le TSP, avec divers types de crossover et de sélection, implémenté de manière efficace avec
/// des buffers réutilisés pour minimiser les allocations mémoire.
///
/// Paramètres :
/// - `crossover_rate`: taux de crossover
/// - `crossover_type`: type de crossover (PMX, OX)
/// - `elitism_rate`: taux d'élitisme
/// - `competition_participation_rate`: taux de participation à chaque compétition pour l'étape de sélection
/// - `competition_type`: type de compétition (tournoi, roulette)
pub struct GeneticAlgorithm<LS> {
    /// taille de l'instance solution
    population_size: usize,

    crossover_rate: f32,
    mutation_rate: f32,
    competition_participation_count: usize,
    elitism_count: usize,
    crossover_type: CrossoverType,
    competition_type: CompetitionType,

    max_iter: usize,
    iteration: usize,

    rng: StdRng,
    
    // Local search
    local_search: LS,

    // Buffers réutilisés, alloués UNE FOIS
    participants_buffer: Vec<usize>,
    population_idx_buffer: Vec<usize>,
    new_population_buffer: Vec<Solution>,
    child1_visited_buffer: Vec<bool>,
    child2_visited_buffer: Vec<bool>,
    child1_mapping_buffer: Vec<u32>,
    child2_mapping_buffer: Vec<u32>,
}

impl<LS> GeneticAlgorithm<LS> {
    /// Crée un nouvel algorithme génétique avec les paramètres donnés.
    ///
    /// Paramètres :
    /// - `instance`: l'instance du problème à résoudre.
    /// - `crossover_type`: le type de crossover à utiliser dans l'algorithme génétique.
    /// - `elitism_rate`: le taux d'élitisme dans l'algorithme génétique.
    /// - `crossover_rate`: le taux de crossover dans l'algorithme génétique.
    /// - `mutation_rate`: le taux de mutation dans l'algorithme génétique.
    /// - `competition_participation_rate`: le taux de participation à la compétition dans l'algorithme génétique.
    /// - `competition_type`: le type de compétition (tournoi, roulette) dans l'algorithme génétique.
    /// - `population_size`: la taille de la population dans l'algorithme génétique.
    pub fn new(
        instance: &Instance,
        crossover_rate: f32,
        crossover_type: CrossoverType,
        elitism_rate: f32,
        competition_participation_rate: f32,
        competition_type: CompetitionType,
        max_iter: usize,
        population_size: usize,
        mutation_rate: f32,
        local_search: LS,
    ) -> Self {
        let solution_size = instance.size();
        let elitism_count = (elitism_rate * population_size as f32) as usize;
        let competition_participation_count =
            (competition_participation_rate * population_size as f32) as usize;

        GeneticAlgorithm {
            population_size,
            crossover_rate,
            mutation_rate,
            crossover_type,
            elitism_count,
            competition_participation_count,
            max_iter,
            iteration: 0,
            rng: StdRng::from_os_rng(),

            competition_type,
            local_search,
            participants_buffer: vec![0; competition_participation_count],
            population_idx_buffer: vec![0; population_size],
            new_population_buffer: vec![vec![0; solution_size]; population_size],
            child1_visited_buffer: vec![false; solution_size],
            child2_visited_buffer: vec![false; solution_size],
            child1_mapping_buffer: vec![u32::MAX; solution_size],
            child2_mapping_buffer: vec![u32::MAX; solution_size],
        }
    }
}

impl<LS> GeneticAlgorithm<LS> {
    /// Sélection par tournoi entre les participants sélectionnés
    fn tournament_selection(&mut self, fitness: &[Fitness]) -> usize {
        // Tournament selection logic
        let best_idx = self
            .rng
            .random_range(0..self.competition_participation_count);
        let mut participant_best_idx = self.participants_buffer[best_idx];

        for _ in 0..self.competition_participation_count {
            let ind = self
                .rng
                .random_range(0..self.competition_participation_count);
            let participant_ind = self.participants_buffer[ind];

            // In TSPTW, LOWER fitness is BETTER (minimization problem)
            if fitness[participant_ind] < fitness[participant_best_idx] {
                participant_best_idx = participant_ind;
            }
        }
        participant_best_idx
    }

    /// Sélection proportionnelle à la fitness (roulette wheel selection)
    fn roulette_selection(&mut self, fitness: &[Fitness]) -> usize {
        // Roulette wheel selection logic
        // For minimization: invert fitness (use 1/fitness) so better solutions have higher probability
        // Find max fitness to avoid division issues
        let max_fitness = self.participants_buffer.iter()
            .map(|&idx| fitness[idx])
            .fold(f32::NEG_INFINITY, |a, b| a.max(b));
        
        // Calculate inverted fitness sum (max - fitness for each)
        let total_inverted: f32 = self
            .participants_buffer
            .iter()
            .map(|&idx| max_fitness - fitness[idx] + 1.0) // +1 to avoid zero weights
            .sum();

        if total_inverted <= f32::EPSILON {
            // Si toutes les fitness sont identiques, retourner un individu aléatoire parmi les participants
            let rand_idx = self
                .rng
                .random_range(0..self.competition_participation_count);
            return self.participants_buffer[rand_idx];
        }

        let mut pick = self.rng.random_range(0.0..total_inverted);

        for idx in 0..self.competition_participation_count {
            let inverted_fitness = max_fitness - fitness[self.participants_buffer[idx]] + 1.0;
            pick -= inverted_fitness;
            if pick <= 0.0 {
                return self.participants_buffer[idx];
            }
        }
        self.participants_buffer.last().cloned().unwrap()
    }

    /// Sélectionne les meilleurs individus de la population et stocke leurs indices dans population_idx_buffer
    fn select_best(&mut self, fitness: &[Fitness]) -> () {
        // Initialize indices
        for i in 0..self.population_size {
            self.population_idx_buffer[i] = i;
        }

        // Use select_nth_unstable to partition and find the best elitism_count indices
        // We want the smallest fitness values (best solutions), so we partition at elitism_count - 1
        self.population_idx_buffer
            .select_nth_unstable_by(self.elitism_count - 1, |&a, &b| {
                fitness[a]
                    .partial_cmp(&fitness[b])
                    .unwrap_or(std::cmp::Ordering::Equal)
            });

        // The first elitism_count elements are now the best indices
        // No need for additional sorting since select_nth_unstable partitions correctly
    }

    /// Sélectionne les participants pour la compétition de manière aléatoire sans remise
    fn select_particiants(&mut self) -> () {
        let pop_size = self.population_size;

        // Initialize the population indices
        for i in 0..pop_size {
            self.population_idx_buffer[i] = i;
        }

        // Shuffle the population indices
        self.population_idx_buffer[..pop_size].shuffle(&mut self.rng);
        
        // Copy the first competition_participation_count indices to participants_buffer
        for i in 0..self.competition_participation_count {
            self.participants_buffer[i] = self.population_idx_buffer[i];
        }
    }

    fn selection_round(&mut self, fitness: &[Fitness]) -> usize {
        // Selection logic based on competition type
        self.select_particiants();
        match self.competition_type {
            CompetitionType::Tournament => self.tournament_selection(fitness),
            CompetitionType::Roulette => self.roulette_selection(fitness),
        }
    }
}

impl<LS> GeneticAlgorithm<LS> {
    fn ox_crossover(
        parent1: &[Ville],
        parent2: &[Ville],
        child_routes: &mut [Solution],
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
            child_routes[0][i] = parent1[i];
            child_routes[1][i] = parent2[i];

            child1_visited_buffer[parent1[i] as usize] = true;
            child2_visited_buffer[parent2[i] as usize] = true;
        }

        let mut current_pos1 = end % size;
        let mut current_pos2 = end % size;

        for i in 0..size {
            let idx = (end + i) % size;

            if !child1_visited_buffer[parent2[idx] as usize] {
                child_routes[0][current_pos1] = parent2[idx];
                current_pos1 = (current_pos1 + 1) % size;
            }

            if !child2_visited_buffer[parent1[idx] as usize] {
                child_routes[1][current_pos2] = parent1[idx];
                current_pos2 = (current_pos2 + 1) % size;
            }
        }
    }
}

impl<LS> GeneticAlgorithm<LS> {
    fn pmx_resolve_mapping(mapping: &[Ville], start: Ville) -> Ville {
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
        child_routes: &mut [Solution],
        child1_mapping_buffer: &mut Vec<Ville>,
        child2_mapping_buffer: &mut Vec<Ville>,
        rng: &mut StdRng,
    ) -> () {
        child1_mapping_buffer.fill(u32::MAX);
        child2_mapping_buffer.fill(u32::MAX);

        // PMX crossover logic
        let size = parent1.len();

        let start = rng.random_range(0..size);
        let end = rng.random_range(start..size);

        // Copier le segment et créer le mapping
        for i in start..end {
            let val1 = parent1[i];
            let val2 = parent2[i];

            child_routes[0][i] = val1;
            child_routes[1][i] = val2;

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
            if child1_mapping_buffer[val2 as usize] != u32::MAX {
                // Suivre le mapping jusqu'à trouver une valeur non utilisée
                val2 = Self::pmx_resolve_mapping(&child1_mapping_buffer, val2);
            }
            child_routes[0][i] = val2;
            child1_mapping_buffer[val2 as usize] = val2;

            // Pour child2, prendre de parent1
            let mut val1 = parent1[i];
            if child2_mapping_buffer[val1 as usize] != u32::MAX {
                // Suivre le mapping jusqu'à trouver une valeur non utilisée
                val1 = Self::pmx_resolve_mapping(&child2_mapping_buffer, val1);
            }
            child_routes[1][i] = val1;
            child2_mapping_buffer[val1 as usize] = val1;
        }
    }
}

impl<LS> GeneticAlgorithm<LS> {
    fn crossover(&mut self, parent1: &[Ville], parent2: &[Ville], cpt: usize) -> () {
        match self.crossover_type {
            CrossoverType::OX => Self::ox_crossover(
                parent1,
                parent2,
                &mut self.new_population_buffer[cpt..cpt + 2],
                &mut self.child1_visited_buffer,
                &mut self.child2_visited_buffer,
                &mut self.rng,
            ),
            CrossoverType::PMX => Self::pmx_crossover(
                parent1,
                parent2,
                &mut self.new_population_buffer[cpt..cpt + 2],
                &mut self.child1_mapping_buffer,
                &mut self.child2_mapping_buffer,
                &mut self.rng,
            ),
        }
    }

    fn select_parents(&mut self, fitness: &[Fitness]) -> (usize, usize) {
        let parent1 = self.selection_round(fitness);
        let parent2 = self.selection_round(fitness);
        (parent1, parent2)
    }
}

impl<LS, Eval> Metaheuristic<Eval> for GeneticAlgorithm<LS>
where
    Eval: Evaluation,
    LS: LocalSearch<Eval>,
{
    fn step(
        &mut self,
        population: &mut [Solution],
        fitness: &mut [Fitness],
        instance: &Instance,
        evaluation: &Eval,
    ) -> () {
        let pop_size = population.len();

        // Selects the best individuals for elitism, puts their indices in population_idx_buffer to avoid
        // reallocating memory each generation
        self.select_best(fitness);

        let best_idx = &self.population_idx_buffer[..self.elitism_count];
        let mut cpt = 0;

        // Adds the best individuals to the new population
        for idx in best_idx.iter() {
            self.new_population_buffer[cpt].clone_from_slice(&population[*idx][..]);
            cpt += 1;
        }

        // Generates new individuals through selection, crossover, and mutation
        while cpt + 1 < pop_size {
            let (parent1_idx, parent2_idx) = self.select_parents(fitness);
            let parent1 = &population[parent1_idx];
            let parent2 = &population[parent2_idx];

            let rand = self.rng.random_range(0.0..1.0);

            if rand < self.crossover_rate {
                // Crossover, produces two children that are added to the new population buffer in place
                self.crossover(parent1, parent2, cpt);
            } else {
                // No crossover, simply clone the parents into the new population buffer
                self.new_population_buffer[cpt].clone_from_slice(&parent1[..]);
                self.new_population_buffer[cpt + 1].clone_from_slice(&parent2[..]);
            };

            cpt += 2;
        }

        // Replace old population with new population
        population.clone_from_slice(&self.new_population_buffer[..]);

        // Apply local search to all non-elite individuals (local search acts as mutation)
        for i in self.elitism_count..pop_size {
            let rand = self.rng.random_range(0.0..1.0);
            if rand < self.mutation_rate {
                self.local_search.search(&mut population[i], &mut fitness[i], instance, evaluation);
            }
        }

        // Re-evaluate fitness of the new population
        for i in self.elitism_count..pop_size {
            fitness[i] = evaluation.score(instance, &population[i]);
        }
        
        // Increment iteration counter
        self.iteration += 1;
    }

    fn stop_condition_met(&self) -> bool {
        self.iteration >= self.max_iter
    }
    fn get_iteration(&self) -> usize {
        self.iteration
    }
}
