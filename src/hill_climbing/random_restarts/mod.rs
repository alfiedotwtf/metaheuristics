use rand::{thread_rng, Rng};
use time::{Duration, PreciseTime};

pub trait RandomRestarts<T> {
    fn clone_candidate(&mut self, candidate: &T) -> T;
    fn generate_candidate(&mut self)             -> T;
    fn rank_candidate(&mut self, candidate: &T)  -> f64;
    fn tweak_candidate(&mut self, candidate: &T) -> T;

    fn run(&mut self, duration: Duration, restart_probability: f64) -> T {
        let mut best_candidate = self.generate_candidate();
        let start_time         = PreciseTime::now();

        while start_time.to(PreciseTime::now()) < duration {
            let next_candidate = match restart_probability > thread_rng().gen_range(0.0, 1.0) {
                true  => self.generate_candidate(),
                false => self.tweak_candidate(&best_candidate),
            };

            if self.rank_candidate(&next_candidate) > self.rank_candidate(&best_candidate) {
                best_candidate = next_candidate;
            }
        }

        best_candidate
    }
}
