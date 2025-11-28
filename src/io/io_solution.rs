use std::fs::File;
use std::io;
use std::io::prelude::*;

pub struct BestSolution {
    pub path: Vec<u32>,
    pub duree: Option<u32>,
}

pub fn load_solution(path: &String) -> io::Result<BestSolution> {

    let file = File::open(path)?;
    let mut reader = io::BufReader::new(file);

    let mut sol_list_line = String::new();
    
    reader.read_line(&mut sol_list_line)?;

    let sol_list: Vec<u32> = sol_list_line
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

        
    // If the file uses 1-based indices (all >= 1), convert to 0-based
    let max_val = sol_list.iter().copied().max().unwrap_or(0);
    let path = if max_val >= 1 && sol_list.iter().all(|&v| v >= 1) {
        eprintln!("Warning: converting 1-based solution file to 0-based indices");
        sol_list.iter().map(|&v| v - 1).collect()
    } else {
        sol_list
    };

    let mut sol_val_line = String::new();
    reader.read_line(&mut sol_val_line)?;

    let sol_val: u32 = sol_val_line
        .trim()
        .parse()
        .map_err(|e| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Failed to parse solution value: {}", e),
                )
            })?;

    Ok(BestSolution {
        path: path,
        duree: Some(sol_val),
    })
}


/// Save a solution list and score to a file
pub fn save_solution(path: &String, sol_list: &[u32], score: Option<u32>) -> io::Result<()> {
    
    let file = File::create(path)?;
    let mut writer = io::BufWriter::new(file);

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
