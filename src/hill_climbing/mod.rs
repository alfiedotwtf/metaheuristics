pub mod random_restarts;
pub mod steepest_ascent;

use time::{Duration, PreciseTime};

pub trait HillClimbing {
    fn generate_candidate()   -> Self;
    fn rank_candidate(&self)  -> f64;
    fn tweak_candidate(&self) -> Self;
}

pub fn run<T: HillClimbing>(duration: Duration) -> T {
    let mut best_candidate = T::generate_candidate();
    let start_time         = PreciseTime::now();

    while start_time.to(PreciseTime::now()) < duration {
        let next_candidate = best_candidate.tweak_candidate();

        if next_candidate.rank_candidate() > best_candidate.rank_candidate() {
            best_candidate = next_candidate;
        }
    }

    best_candidate
}
