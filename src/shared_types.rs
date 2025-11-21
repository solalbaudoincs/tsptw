
use crate::problem::instance::{Node, Instance};
use crate::problem::solution::{Solution};
use crate::problem::evaluation::Evaluation;
use crate::neighbourhood::NeighborFn;
use crate::algorithms::Metaheuristic;

// Un conteneur pour les pointeurs vers des idx de vecteurs dans les algorithmes
pub type View = Vec<usize>;
pub type Fitness = f32;

