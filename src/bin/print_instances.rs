use mh_tsptw::io::io_instance::load_instance;

fn main() {
    let paths = ["data/inst1"];

    for path in paths {
        println!("Loading {}", path);
        match load_instance(path) {
            Ok((instance, _graph)) => {
                println!("  OK: {} nodes", instance.windows.len());
                // println!("  Distance matrix: {}x{}", instance.distance_matrix.nrows(), instance.distance_matrix.ncols());
                // println!("  First windows:");
                for (i, w) in instance.windows.iter().enumerate() {
                    println!("    {}: [{}, {}]", i, w.wstart, w.wend);
                }
                println!();
            }
            Err(e) => {
                println!("  FAILED to load {}: {}", path, e);
            }
        }
    }
}