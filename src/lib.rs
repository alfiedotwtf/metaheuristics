extern crate rand;
extern crate time;

pub mod hill_climbing;
pub mod random_search;
pub mod simulated_annealing;

pub trait Metaheuristics<T> {
    fn clone_candidate(&mut self, candidate: &T) -> T;
    fn generate_candidate(&mut self)             -> T;
    fn rank_candidate(&mut self, candidate: &T)  -> f64;
    fn tweak_candidate(&mut self, candidate: &T) -> T;
}
