use super::super::Metaheuristics;
use time::{Duration, PreciseTime};

pub fn solve<T>(problem: &mut Metaheuristics<T>, duration: Duration, tries: u64) -> T {
    let mut best_candidate = problem.generate_candidate();
    let start_time         = PreciseTime::now();

    while start_time.to(PreciseTime::now()) < duration {
        let mut next_candidate = problem.tweak_candidate(&best_candidate);

        for _ in 0..tries {
            let neighbour_candidate = problem.tweak_candidate(&best_candidate);

            if problem.rank_candidate(&neighbour_candidate) > problem.rank_candidate(&next_candidate) {
                next_candidate = neighbour_candidate;
            }
        }

        if problem.rank_candidate(&next_candidate) > problem.rank_candidate(&best_candidate) {
            best_candidate = next_candidate;
        }
    }

    best_candidate
}
