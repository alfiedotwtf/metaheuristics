//! Find an approximate solution to your optimisation problem using Random Search
//!
//! **Note: This isn't really a useful algorithm and is only included for completness.**
//!
//! At each iteration, generate an independantly random solution. When the time runs out, return
//! the best solution found.
//!
//!# Examples
//!
//!```
//!let solution = metaheuristics::random_search::solve(&mut problem, runtime);
//!```

use super::Metaheuristics;
use time::{Duration, PreciseTime};

/// Returns an approximate solution to your optimisation problem using Random Search
///
///# Parameters
///
/// `problem` is the type that implements the `Metaheuristics` trait.
///
/// `runtime` is a `time::Duration` specifying how long to spend searching for a solution.
///
///# Examples
///
///```
///let solution = metaheuristics::random_search::solve(&mut problem, runtime);
///```
pub fn solve<T>(problem: &mut Metaheuristics<T>, runtime: Duration) -> T {
    let mut best_candidate = problem.generate_candidate();
    let start_time         = PreciseTime::now();

    while start_time.to(PreciseTime::now()) < runtime {
        let next_candidate = problem.generate_candidate();

        if problem.rank_candidate(&next_candidate) > problem.rank_candidate(&best_candidate) {
            best_candidate = next_candidate;
        }
    }

    best_candidate
}
