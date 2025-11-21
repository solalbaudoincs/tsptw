pub mod io;

#[derive(Clone)]
pub struct Solution {
    pub sol_list: Vec<u32>,
    pub sol_val: Option<u32>,
}


pub type Population = Vec<Solution>;
