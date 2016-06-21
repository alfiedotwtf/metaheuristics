use rand::{thread_rng, Rng};
use super::super::Metaheuristics;
use time::{Duration, PreciseTime};

pub fn solve<T>(problem: &mut Metaheuristics<T>, duration: Duration, restart_probability: f64) -> T {
    let mut best_candidate = problem.generate_candidate();
    let start_time         = PreciseTime::now();

    while start_time.to(PreciseTime::now()) < duration {
        let next_candidate = match restart_probability > thread_rng().gen_range(0.0, 1.0) {
            true  => problem.generate_candidate(),
            false => problem.tweak_candidate(&best_candidate),
        };

        if problem.rank_candidate(&next_candidate) > problem.rank_candidate(&best_candidate) {
            best_candidate = next_candidate;
        }
    }

    best_candidate
}
