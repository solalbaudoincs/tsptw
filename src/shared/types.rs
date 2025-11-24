// A solution is a hamiltionian cycle represented as a vector of node indices
pub type Solution = Vec<u32>;

// Un conteneur pour les pointeurs vers des idx de vecteurs dans les algorithmes

pub type Fitness = f32;

#[derive(Clone)]
pub struct Window {
    pub wstart: f32,
    pub wend: f32,
}

use ndarray::Array2;

#[derive(Clone)]
pub struct Instance {
    pub windows: Vec<Window>,
    pub distance_matrix: Array2<f32>,
}

impl Instance {
    pub fn size(&self) -> usize {
        self.windows.len()
    }
}

// Node with position information for visualization
pub struct Node {
    pub x: f32,
    pub y: f32,
    pub wstart: f32,
    pub wend: f32,
}

// Instance with graph information for GUI display
pub struct GraphInstance {
    pub graph: Vec<Node>,
}