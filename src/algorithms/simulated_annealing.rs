
use super::Metaheuristic;
use crate::neighbourhood::NeighbourhoodGenerator;
use crate::problem::{
    evaluation::{Evaluation},
    instance::Instance,
    solution::{Population, Solution},
};


struct LcgRng(u64);

impl LcgRng {
    fn next(&mut self) -> u64 {
        const MULTIPLIER: u64 = 636_413_622_384_679_3005;
        const INCREMENT: u64 = 1;
        self.0 = self.0.wrapping_mul(MULTIPLIER).wrapping_add(INCREMENT);
        self.0
    }

    fn next_f64(&mut self) -> f64 {
        let max = u64::MAX as f64;
        (self.next() as f64) / (max + 1.0)
    }

    fn range(&mut self, upper: usize) -> usize {
        if upper == 0 {
            0
        } else {
            let sample = self.next_f64();
            (sample * upper as f64) as usize
        }
    }
}

pub struct SimulatedAnnealing {
    temperature: f64,
    cooling_rate: f64,
    min_temperature: f64,
    rng: LcgRng,
}

impl SimulatedAnnealing {
    pub fn new(
        temperature: f64,
        cooling_rate: f64,
        min_temperature: f64,
    ) -> Self {
        assert!(temperature > 0.0, "initial temperature must be positive");
        assert!(
            cooling_rate > 0.0 && cooling_rate < 1.0,
            "cooling rate should be between 0 and 1"
        );
        assert!(
            min_temperature >= 0.0 && min_temperature < temperature,
            "min temperature must be non-negative and smaller than initial temperature"
        );

        let seed = temperature.to_bits() ^  min_temperature.to_bits();
        Self {
            temperature,
            cooling_rate,
            min_temperature,
            rng: LcgRng(seed),
        }
    }


    fn cool(&mut self) {
        self.temperature = (self.temperature * self.cooling_rate).max(self.min_temperature);
    }

    pub fn estimate_initial_temperature<N: NeighbourhoodGenerator, Eval: Evaluation>(
            instance: &Instance,
            base: &Solution,
            neighbourhood: &N,
            samples: usize,
            p_init: f64,
            evaluation: &Eval,
        ) -> f64 {
        assert!(
            p_init > 0.0 && p_init < 1.0,
            "p_init must be between 0 and 1"
        );
        let neighbours: Vec<Solution> = neighbourhood.generate(base).collect();
        if neighbours.is_empty() {
            return 1.0;
        }

        let mut rng = LcgRng(base.sol_list.len() as u64 + neighbours.len() as u64 + 1);
        let mut scores = Vec::with_capacity(samples);

        for _ in 0..samples {
            let idx = rng.range(neighbours.len());
            let neighbor_score = evaluation.score(instance, &neighbours[idx]);
            scores.push(neighbor_score);
        }
        
        //compute deltas
        let mut deltas = Vec::with_capacity(samples*(samples-1)/2);
        for i in 0..scores.len() {
            for j in (i + 1)..scores.len() {
                let delta = (scores[j] - scores[i]).abs();
                deltas.push(delta);
            }
        }

        let mean = deltas.iter().copied().sum::<f64>() / deltas.len() as f64;
        let variance =
            deltas.iter().map(|d| (*d - mean).powi(2)).sum::<f64>() / deltas.len() as f64;
        let sigma = variance.sqrt();

        let temp = (-3.0 * sigma) / p_init.ln();
        if temp.is_finite() && temp > 0.0 {
            temp
        } else {
            sigma.max(1.0)
        }
    }
}

impl Metaheuristic for SimulatedAnnealing {
    fn step<Eval: Evaluation, N: NeighbourhoodGenerator>(
        &mut self,
        population: &mut Population,
        best: usize,
        instance: &Instance,
        neighbourhood: &N,
        evaluation: &Eval,
    ) {
        if population.is_empty() || best >= population.len() {
            return;
        }

        let neighbours: Vec<Solution> = neighbourhood.generate(&population[best]).collect();
        if neighbours.is_empty() {
            return;
        }

        let current_score = evaluation.score(instance, &population[best]);
        let neighbours_len = neighbours.len();

        let idx = self.rng.range(neighbours_len);
        let candidate = neighbours[idx].clone();
        let candidate_score = evaluation.score(instance, &candidate);
        let delta = candidate_score - current_score;

        let probability = (-delta / self.temperature).exp();
        let accept = delta < 0.0 || self.rng.next_f64() < probability;

        if accept {
            population[best] = candidate;
        }

        self.cool();
    }
}
