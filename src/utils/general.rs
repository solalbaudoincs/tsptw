use crate::shared_types::View;


pub fn argsort_f64(v: &Vec<f64>, buffer: &mut View) -> () {
    for i in 0..buffer.len() {
        buffer[i] = i
    }
    buffer.sort_by(|&i, &j| v[i].total_cmp(&v[j]));
}
