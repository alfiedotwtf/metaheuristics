//! Find approximate solutions to your optimisation problem using metaheuristics algorithms
//!
//!# What are Metaheuristics
//!
//! Metaheuristics are a class of stochastic optimisation algorithms. These type of algorithms rely
//! on randomness to jump around the search space, then sample where they land for possible
//! solutions. In simple terms, **metaheuristics are structured trial and error**.
//!
//!# How can I use this crate
//!
//! By implementing the `Metaheuristics` trait, the algorithms within the following modules will be
//! available to you. To see an example implementation, check out the [Travelling Salesman
//! Problem](https://crates.io/crates/travelling_salesman/) crate.
//!
//!## Example
//!
//!```
//! let solution = metaheuristics::hill_climbing::solve(&mut problem, runtime);
//!```
//!

extern crate rand;
extern crate time;

pub mod hill_climbing;
pub mod random_search;
pub mod simulated_annealing;

/// Implement this simple trait to apply metaheuristics to your optimisation problems
pub trait Metaheuristics<T> {
    /// Clone the supplied candidate solution
    ///
    ///```
    /// let new_candidate = problem.clone_candidate(&old_candidate);
    ///```
    fn clone_candidate(&mut self, candidate: &T) -> T;

    /// Randomly generate a new candidate solution
    ///
    ///```
    /// let candidate = problem.generate_candidate();
    ///```
    fn generate_candidate(&mut self) -> T;

    /// Rank a candidate solution so that it can be compared with another (higher is better)
    ///
    ///```
    /// if problem.rank_candidate(&new_candidate) > problem.rank_candidate(&old_candidate) {
    ///     ...
    /// }
    ///```
    fn rank_candidate(&mut self, candidate: &T)  -> f64;

    /// Clone the supplied candidate solution, then make a small (but random) modification
    ///
    ///```
    /// let new_candidate = problem.tweak_candidate(&old_candidate);
    ///```
    fn tweak_candidate(&mut self, candidate: &T) -> T;
}
