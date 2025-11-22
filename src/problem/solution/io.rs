use super::Solution;
use crate::problem::Instance;

use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};

use rayon::str;

pub fn load_solution(path: &str) -> Result<(Solution, u32), std::io::Error> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);

    let mut sol_list_line = String::new();
    reader.read_line(&mut sol_list_line)?;
    let sol_list: Vec<u32> = sol_list_line
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .map(| n: u32| n-1 ) 
        .collect();


    let mut sol_val_line = String::new();
    reader.read_line(&mut sol_val_line)?;
    let sol_val: u32 = sol_val_line.trim().parse().map_err(|e| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("Failed to parse solution value: {}", e),
        )
    })?;
    Ok((sol_list, sol_val))
}

pub fn print_solution(sol : &Solution, instance: &Instance) {
    let sol_line = sol
        .iter()
        .map(|n| (n + 1).to_string()) 
        .collect::<Vec<_>>()
        .join(" ");
    println!("{}", sol_line);
    let (dist, viol) = crate::problem::evaluation::utils::run_solution(
        instance,
        sol,
    );
    println!(
        "Solution performance: total_distance={}, total_violation={}", dist, viol
    );
}

/// Save a solution list and score to a file
pub fn save_solution(path: &str, sol_list: &[u32], score: Option<u32>) -> std::io::Result<()> {
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);

    // Write solution list as space-separated
    let sol_line = sol_list
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<_>>()
        .join(" ");
    writeln!(writer, "{}", sol_line)?;

    // Write score (if present, else blank line)
    if let Some(s) = score {
        writeln!(writer, "{}", s)?;
    } else {
        writeln!(writer)?;
    }
    Ok(())
}
