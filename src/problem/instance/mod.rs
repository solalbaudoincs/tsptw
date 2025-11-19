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
    pub graph: HashMap<u32, Node>,
    pub distance_matrix: HashMap<(u32, u32), f64>,
}


pub fn distance(n1 : &Node, n2 : &Node) -> f64 {
    ((n1.x - n2.x).powi(2) + (n1.y - n2.y).powi(2)).sqrt()
}
