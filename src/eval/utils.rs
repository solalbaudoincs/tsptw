use crate::shared::{Instance, Solution};


pub struct Eval {
    pub total_distance: f32,
    pub violation_time: f32,
    pub total_time: f32,
    pub nb_violations: u32,
    pub delay: f32,
}

pub fn run_solution(instance: &Instance, solution: &Solution) -> Eval {

    let mut total_distance: f32 = 0.0;
    let mut total_time: f32 = 0.0;
    let mut violation_time: f32 = 0.0;
    let mut nb_violations: u32 = 0;
    let mut delay: f32 = 0.0;

    for idx in 0..(solution.len()) {

        let from = solution[idx] as usize;
        let to = solution[(idx + 1) % solution.len()] as usize;

        total_time += instance.distance_matrix[[from, to]];
        total_distance += instance.distance_matrix[[from, to]];
        let next_start = instance.windows[to].wstart;
        let next_end = instance.windows[to].wend;

        if total_time<next_start {
            delay += next_start - total_time;
            total_time = next_start;
        }
        if total_time>next_end {
            violation_time += total_time-next_end;
            nb_violations += 1;
        }
    }
    Eval {
        total_distance, 
        violation_time,
        total_time,
        nb_violations,
        delay,
    }
}


// pub fn run_solution(instance: &Instance, solution: &Solution) -> (f32, f32) {
//     let mut total_distance: f32 = 0.0;
//     let mut total_violation: f32 = 0.0;
//     let mut total_time: f32 = 0.0;

//     let mut visit_edge = |from: usize, to: usize| {
//         let travel_time = instance.distance_matrix[from][to as usize];
//         total_distance += travel_time;
//         total_time += travel_time;

//         let window_start = instance.graph[to].wstart;
//         let window_end = instance.graph[to].wend;
//         if total_time < window_start {
//             //println!("waiting at node {} from time {:.2} to {:.2}", to, total_time, window_start);
//             total_time = window_start;
//         }

//         if total_time > window_end {
//             total_violation += total_time - window_end;
//             // println!(
//             //     "  Arrival at node {} at time {:.2} violates time window [{:.2}, {:.2}]",
//             //     to, total_time, window_start, window_end
//             // );
//         }
//         // println!("Travel from {} to {}: distance {}, total_distance {}", from, to, travel_time, total_distance);
//     };

//     if !solution.is_empty() {
//         for window in solution.windows(2) {
//             visit_edge(window[0] as usize, window[1] as usize);
//         }
//         let last = *solution.last().unwrap();
//         let first = solution[0];
//         visit_edge(last as usize, first as usize);
//     }

//     (total_distance, total_violation)
// }
