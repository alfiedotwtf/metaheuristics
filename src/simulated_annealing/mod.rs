use rand::{thread_rng, Rng};
use time::{Duration, PreciseTime};

pub trait SimulatedAnnealing {
    fn generate_candidate()   -> Self;
    fn clone_candidate(&self) -> Self;
    fn rank_candidate(&self)  -> f64;
    fn tweak_candidate(&self) -> Self;
}

pub fn run<T: SimulatedAnnealing>(duration: Duration) -> T {
    let mut best_candidate      = T::generate_candidate();
    let mut annealing_candidate = best_candidate.tweak_candidate();
    let start_time              = PreciseTime::now();

    loop {
        let portion_elapsed
            = (start_time.to(PreciseTime::now()).num_milliseconds() as f64)
            / (duration.num_milliseconds() as f64);

        if portion_elapsed >= 1.0 {
            break;
        }

        let next_candidate        = annealing_candidate.tweak_candidate();
        let next_is_better        = next_candidate.rank_candidate() > annealing_candidate.rank_candidate();
        let replacement_threshold = 1.0f64.exp().powf(-10.0 * portion_elapsed.powf(3.0));

        if next_is_better || (thread_rng().gen_range(0.0, 1.0) < replacement_threshold) {
            annealing_candidate = next_candidate;
        }

        if annealing_candidate.rank_candidate() > best_candidate.rank_candidate() {
            best_candidate = annealing_candidate.clone_candidate();
        }
    }

    best_candidate
}
