use time::{Duration, PreciseTime};

pub trait SteepestAscent<T> {
    fn clone_candidate(&mut self, candidate: &T) -> T;
    fn generate_candidate(&mut self)             -> T;
    fn rank_candidate(&mut self, candidate: &T)  -> f64;
    fn tweak_candidate(&mut self, candidate: &T) -> T;

    fn run(&mut self, duration: Duration, tries: u64) -> T {
        let mut best_candidate = self.generate_candidate();
        let start_time         = PreciseTime::now();

        while start_time.to(PreciseTime::now()) < duration {
            let mut next_candidate = self.tweak_candidate(&best_candidate);

            for _ in 0..tries {
                let neighbour_candidate = self.tweak_candidate(&best_candidate);

                if self.rank_candidate(&neighbour_candidate) > self.rank_candidate(&next_candidate) {
                    next_candidate = neighbour_candidate;
                }
            }

            if self.rank_candidate(&next_candidate) > self.rank_candidate(&best_candidate) {
                best_candidate = next_candidate;
            }
        }

        best_candidate
    }
}
