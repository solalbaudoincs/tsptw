use crate::problem::solution::Solution;
use crate::problem::instance::{Instance, distance};



pub fn run_solution(instance: &Instance, solution: &Solution) -> (f64, f64) {
    let mut total_distance = 0.0;
    let mut total_violation = 0.0;
    for i in 0..(instance.graph.len()-1) {
        let from = solution.sol_list[i];
        let to = solution.sol_list[i+1];

        let travel_time = instance.distance_matrix[&(from, to)];
        let arrival_time = total_distance;

        total_distance += travel_time; 
        
        if arrival_time < instance.graph[&to].wstart {
            total_violation += instance.graph[&to].wstart - arrival_time;
        } else if arrival_time > instance.graph[&to].wend {
            total_violation += arrival_time - instance.graph[&to].wend;
        }
    }
    (total_distance, total_violation)
}
