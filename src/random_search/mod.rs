use super::Metaheuristics;
use time::{Duration, PreciseTime};

pub fn solve<T>(problem: &mut Metaheuristics<T>, duration: Duration) -> T {
    let mut best_candidate = problem.generate_candidate();
    let start_time         = PreciseTime::now();

    while start_time.to(PreciseTime::now()) < duration {
        let next_candidate = problem.generate_candidate();

        if problem.rank_candidate(&next_candidate) > problem.rank_candidate(&best_candidate) {
            best_candidate = next_candidate;
        }
    }

    best_candidate
}
