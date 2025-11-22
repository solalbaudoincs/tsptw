// Un conteneur pour les pointeurs vers des idx de vecteurs dans les algorithmes
pub type View = Vec<usize>;

pub fn argsort_f32(v: &Vec<f32>, buffer: &mut View) -> () {
    for i in 0..buffer.len() {
        buffer[i] = i
    }
    buffer.sort_by(|&i, &j| v[i].total_cmp(&v[j]));
}
