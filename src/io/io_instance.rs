use std::fs;
use std::io;
use std::io::BufRead;

use ndarray::Array2;

use crate::shared::{Instance, GraphInstance, Node, Window};

struct Position {
    x: f32,
    y: f32,
}



fn calculate_distance_matrix(positions: &Vec<Position>) -> Array2<f32> {
    let node_number = positions.len();
    let mut distance_matrix = vec![vec![0.0; node_number]; node_number];

    for i in 0..positions.len() {
        for j in 0..positions.len() {
            let node1 = &positions[i];
            let node2 = &positions[j];
            let dist = ((node1.x - node2.x).powi(2) + (node1.y - node2.y).powi(2)).sqrt().floor();
            distance_matrix[i][j] = dist;
        }
    }
    // relax distances through intermediate nodes to match the Python reference implementation
    for k in 0..node_number {
        for i in 0..node_number {
            for j in 0..node_number {
                let current = distance_matrix[i][j];
                let via_k = distance_matrix[i][k] + distance_matrix[k][j];
                if via_k <  current {
                    distance_matrix[i][j] = via_k;
                }
            }
        }
    }
    Array2::from_shape_vec((node_number, node_number), distance_matrix.into_iter().flatten().collect()).unwrap()
}


pub fn load_instance(path: &str) -> io::Result<(Instance, GraphInstance)> {
    let file = fs::File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut windows = Vec::new();
    let mut positions = Vec::new();
    let mut graph = Vec::new();

    for (line_idx, line_result) in reader.lines().enumerate() {
        let line = line_result?;  // si erreur lecture → retourne Err
        let line = line.trim();

        // Skip empty lines and comment lines (starting with !)
        if line.is_empty() || line.starts_with('!') || line.starts_with("CUST NO.") {
            continue;
        }

        let mut parts = line.split_whitespace();

        let _id = parts.next();

        if _id == Some("999") {
            break; // End of instance data
        }

        let x = parts
            .next()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData,
                format!("Missing x at line {}", line_idx)))?
            .parse::<f32>()
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData,
                format!("Invalid float x at line {}", line_idx)))?;

        let y = parts
            .next()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData,
                format!("Missing y at line {}", line_idx)))?
            .parse::<f32>()
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData,
                format!("Invalid float y at line {}", line_idx)))?;
        
        let _dmd = parts.next();

        let wstart = parts
            .next()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData,
                format!("Missing wstart at line {}", line_idx)))?
            .parse::<f32>()
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData,
                format!("Invalid float wstart at line {}", line_idx)))?;

        let wend = parts
            .next()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData,
                format!("Missing wend at line {}", line_idx)))?
            .parse::<f32>()
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData,
                format!("Invalid float wend at line {}", line_idx)))?;

        windows.push(Window { wstart, wend });
        positions.push(Position { x, y });
        graph.push(Node { x, y, wstart, wend });
    }

    let distance_matrix = calculate_distance_matrix(&positions);
    Ok((
        Instance {
            windows,
            distance_matrix,
        },
        GraphInstance { graph }
    ))
}