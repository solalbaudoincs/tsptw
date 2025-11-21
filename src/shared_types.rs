
// A solution is a hamiltionian cycle represented as a vector of node indices
pub type Node = usize;
pub type Solution = Vec<Node>;

// Un conteneur pour les pointeurs vers des idx de vecteurs dans les algorithmes
pub type View = Vec<usize>;

pub type Fitness = f32;

pub struct Instance {
    pub dist_matrix: Vec<Vec<f32>>,
    time_windows: Vec<(f32, f32)>,
}

impl Instance {
    pub fn get_window_start(&self, idx: usize) -> f32 {
        self.time_windows[idx].0
    }

    pub fn get_window_end(&self, idx: usize) -> f32 {
        self.time_windows[idx].1
    }
}

pub struct NodeMap {
    pub index_to_id: Vec<String>,
    pub id_to_index: std::collections::HashMap<String, Node>,
}