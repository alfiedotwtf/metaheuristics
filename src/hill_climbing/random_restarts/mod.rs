use rand::{thread_rng, Rng};
use time::{Duration, PreciseTime};

pub trait RandomRestarts {
    fn generate_candidate()   -> Self;
    fn rank_candidate(&self)  -> f64;
    fn tweak_candidate(&self) -> Self;
}

pub fn run<T: RandomRestarts>(duration: Duration, restart_probability: f64) -> T {
    let mut best_candidate = T::generate_candidate();
    let start_time         = PreciseTime::now();

    while start_time.to(PreciseTime::now()) < duration {
        let next_candidate = match restart_probability > thread_rng().gen_range(0.0, 1.0) {
            true  => T::generate_candidate(),
            false => best_candidate.tweak_candidate(),
        };

        if next_candidate.rank_candidate() > best_candidate.rank_candidate() {
            best_candidate = next_candidate;
        }
    }

    best_candidate
}
