pub mod io;
#[derive(Debug)]
pub struct Node {
    pub x: f32,
    pub y: f32,
    pub wstart: f32,
    pub wend: f32,
}

pub struct Instance {
    pub graph: Vec<Node>,
    pub distance_matrix: Vec<Vec<f32>>,
}
