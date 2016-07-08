//! Find an approximate solution to your optimisation problem using Hill Climbing with random restarts
//!
//! Here we duplicate the functionality of the `metaheuristics::hill_climbing` module but with
//! slight modification - we introduce a probability of restarting the algorithm (whilst
//! remembering the best candidate solution seen so far).
//!
//! The advantage here over vanilla Hill Climbing is that this guarantees we'll never get stuck
//! at a local maximum. In fact, given enough time, Hill Climbing with random restarts will find
//! the globally optimal solution.
//!
//!# Examples
//!
//!```
//!let solution = metaheuristics::hill_climbing::random_restarts::solve(
//!    &mut problem,
//!    runtime,
//!    probability
//!);
//!```

use rand::{thread_rng, Rng};
use super::super::Metaheuristics;
use time::{Duration, PreciseTime};

/// Returns an approximate solution to your optimisation problem using Hill Climbing with random restarts
///
///```
///let solution = metaheuristics::hill_climbing::random_restarts::solve(
///    &mut problem,
///    runtime,
///    probability
///);
///```
///
/// `problem` is the type that implements the `Metaheuristics` trait.
///
/// `runtime` is a `time::Duration` specifying how long to spend searching for a solution.
///
/// `probability` is a value within the range `[0.0, 1.0)` specifying the restart probability.
pub fn solve<T>(problem: &mut Metaheuristics<T>, runtime: Duration, probability: f64) -> T {
    let mut best_candidate = problem.generate_candidate();
    let start_time         = PreciseTime::now();

    while start_time.to(PreciseTime::now()) < runtime {
        let next_candidate = match probability > thread_rng().gen_range(0.0, 1.0) {
            true  => problem.generate_candidate(),
            false => problem.tweak_candidate(&best_candidate),
        };

        if problem.rank_candidate(&next_candidate) > problem.rank_candidate(&best_candidate) {
            best_candidate = next_candidate;
        }
    }

    best_candidate
}
