pub use crate::problem::instance::{Node, Instance};
pub use crate::problem::solution::{Solution, Population};
pub use crate::problem::evaluation::{Evaluation, Fitness, Fitnesses};
pub use crate::neighbourhood::NeighborFn;
pub use crate::algorithms::Metaheuristic;

// Un conteneur pour les pointeurs vers des idx de vecteurs dans les algorithmes
pub type View = Vec<usize>;
