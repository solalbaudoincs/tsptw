pub mod io;

use std::collections::HashMap;

#[derive(Debug)]
pub struct Node {
    pub x: f64,
    pub y: f64,
    pub wstart: f64,
    pub wend: f64,
}

pub struct Instance {
    pub graph: Vec<Node>,
    pub distance_matrix: Vec<Vec<f64>>,
}
