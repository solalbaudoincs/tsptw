use crate::shared::{Instance, Solution};

pub trait Initializer {
    fn initialize(&mut self, problem: &Instance) -> Solution;
}

mod random_init;
pub use random_init::{RandomInitializer, SeededRandomInitializer};
