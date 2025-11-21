use crate::problem::instance::Instance;
use crate::problem::solution::Solution;



pub fn run_solution(instance: &Instance, solution: &Solution) -> (f64, f64) {
    let mut total_distance = 0.0;
    let mut total_violation = 0.0;
    let mut total_time = 0.0;

    let mut visit_edge = |from: u32, to: u32| {
        let travel_time = instance.distance_matrix[&(from, to)];
        total_distance += travel_time;
        total_time   += travel_time;
        
        let window_start = instance.graph[&to].wstart;
        let window_end = instance.graph[&to].wend;
        if total_time < window_start {
            //println!("waiting at node {} from time {:.2} to {:.2}", to, total_time, window_start);
            total_time = window_start;
        }

        if total_time > window_end {
            total_violation += total_time - window_end;
            // println!(
            //     "  Arrival at node {} at time {:.2} violates time window [{:.2}, {:.2}]",
            //     to, total_time, window_start, window_end
            // );
        }
        // println!("Travel from {} to {}: distance {}, total_distance {}", from, to, travel_time, total_distance);
    };

    if !solution.sol_list.is_empty() {
        for window in solution.sol_list.windows(2) {
            visit_edge(window[0], window[1]);
        }
        let last = *solution.sol_list.last().unwrap();
        let first = solution.sol_list[0];
        visit_edge(last, first);
    }

    
    (total_distance, total_violation)
}
