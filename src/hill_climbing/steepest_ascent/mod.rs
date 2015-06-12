use time::{Duration, PreciseTime};

pub trait SteepestAscent {
    fn generate_candidate()   -> Self;
    fn rank_candidate(&self)  -> f64;
    fn tweak_candidate(&self) -> Self;
}

pub fn run<T: SteepestAscent>(duration: Duration, tries: u64) -> T {
    let mut best_candidate = T::generate_candidate();
    let start_time         = PreciseTime::now();

    while start_time.to(PreciseTime::now()) < duration {
        let mut next_candidate = best_candidate.tweak_candidate();

        for _ in 0..tries {
            let neighbour_candidate = best_candidate.tweak_candidate();

            if neighbour_candidate.rank_candidate() > next_candidate.rank_candidate() {
                next_candidate = neighbour_candidate;
            }
        }

        if next_candidate.rank_candidate() > best_candidate.rank_candidate() {
            best_candidate = next_candidate;
        }
    }

    best_candidate
}
