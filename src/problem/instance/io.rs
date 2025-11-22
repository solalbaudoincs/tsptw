use std::fs::File;
use std::io::{BufRead, BufReader, Write};

use crate::problem::Solution;

use super::{Instance, Node};

fn calculate_distance_matrix(graph: &Vec<Node>) -> Vec<Vec<f64>> {
    let node_number = graph.len();
    let mut distance_matrix = vec![vec![0.0; node_number]; node_number];

    for i in 0..graph.len() {
        for j in 0..graph.len() {
            let node1 = &graph[i];
            let node2 = &graph[j];
            let dist = ((node1.x - node2.x).powi(2) + (node1.y - node2.y).powi(2))
                .sqrt()
                .floor();
            distance_matrix[i][j] = dist;
        }
    }
    // relax distances through intermediate nodes to match the Python reference implementation
    for k in 0..node_number {
        for i in 0..node_number {
            for j in 0..node_number {
                let current = distance_matrix[i][j];
                let via_k = distance_matrix[i][k] + distance_matrix[k][j];
                if via_k < current {
                    distance_matrix[i][j] = via_k;
                }
            }
        }
    }
    distance_matrix
}

pub fn load_instance(path: &str) -> Instance {
    let file = File::open(path).expect("Unable to open instance file");
    let reader = BufReader::new(file);
    let mut graph = Vec::new();
    for line in reader.lines() {
        let l = line.unwrap();
        let vals: Vec<&str> = l.split_whitespace().filter(|s| !s.is_empty()).collect();
        if vals.is_empty()
            || (!vals[0].chars().all(char::is_numeric) || vals[0].parse::<u32>().unwrap() >= 999)
        {
            continue;
        }
        let idx: u32 = vals[0].parse().unwrap();
        let node = Node {
            x: vals[1].parse().unwrap(),
            y: vals[2].parse().unwrap(),
            wstart: vals[4].parse().unwrap(),
            wend: vals[5].parse().unwrap(),
        };
        graph.push(node);
    }
    let distance_matrix = calculate_distance_matrix(&graph);
    Instance {
        graph,
        distance_matrix,
    }
}
