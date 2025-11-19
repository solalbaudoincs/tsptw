use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

use super::{Node, Instance};

fn calculate_distance_matrix(graph: &HashMap<u32, Node>) -> HashMap<(u32, u32), f64> {
    let mut distance_matrix = HashMap::new();
    for (&id1, node1) in graph.iter() {
        for (&id2, node2) in graph.iter() {
            let dist = ((node1.x - node2.x).powi(2) + (node1.y - node2.y).powi(2)).sqrt();
            distance_matrix.insert((id1, id2), dist);
        }
    }
    distance_matrix
}

pub fn load_instance(path: &str) -> Instance {
    let file = File::open(path).expect("Unable to open instance file");
    let reader = BufReader::new(file);
    let mut graph = HashMap::new();
    for line in reader.lines() {
        let l = line.unwrap();
        let vals: Vec<&str> = l.split_whitespace().filter(|s| !s.is_empty()).collect();
        if vals.is_empty() || !vals[0].chars().all(char::is_numeric) { continue; }
        let idx: u32 = vals[0].parse().unwrap();
        let node = Node {
            x: vals[1].parse().unwrap(),
            y: vals[2].parse().unwrap(),
            wstart: vals[4].parse().unwrap(),
            wend: vals[5].parse().unwrap(),
        };
        graph.insert(idx, node);
    }
    let distance_matrix = calculate_distance_matrix(&graph);
    Instance {
        graph,
        distance_matrix,
    }
}
