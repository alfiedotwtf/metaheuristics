use time::{Duration, PreciseTime};

pub trait RandomSearch {
    fn generate_candidate()  -> Self;
    fn rank_candidate(&self) -> f64;
}

pub fn run<T: RandomSearch>(duration: Duration) -> T {
    let mut best_candidate = T::generate_candidate();
    let start_time         = PreciseTime::now();

    while start_time.to(PreciseTime::now()) < duration {
        let next_candidate = T::generate_candidate();

        if next_candidate.rank_candidate() > best_candidate.rank_candidate() {
            best_candidate = next_candidate;
        }
    }

    best_candidate
}
