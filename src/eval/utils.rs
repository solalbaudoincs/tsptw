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