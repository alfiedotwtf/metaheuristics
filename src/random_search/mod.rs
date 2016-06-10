use time::{Duration, PreciseTime};

pub trait RandomSearch<T> {
    fn generate_candidate(&mut self)            -> T;
    fn rank_candidate(&mut self, candidate: &T) -> f64;

    fn run(&mut self, duration: Duration) -> T {
        let mut best_candidate = self.generate_candidate();
        let start_time         = PreciseTime::now();

        while start_time.to(PreciseTime::now()) < duration {
            let next_candidate = self.generate_candidate();

            if self.rank_candidate(&next_candidate) > self.rank_candidate(&best_candidate) {
                best_candidate = next_candidate;
            }
        }

        best_candidate
    }
}
