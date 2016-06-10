use rand::{thread_rng, Rng};
use time::{Duration, PreciseTime};

pub trait SimulatedAnnealing<T> {
    fn clone_candidate(&mut self, candidate: &T) -> T;
    fn generate_candidate(&mut self)             -> T;
    fn rank_candidate(&mut self, candidate: &T)  -> f64;
    fn tweak_candidate(&mut self, candidate: &T) -> T;

    fn run(&mut self, duration: Duration) -> T {
        let mut best_candidate      = self.generate_candidate();
        let mut annealing_candidate = self.tweak_candidate(&best_candidate);
        let start_time              = PreciseTime::now();

        loop {
            let portion_elapsed
                = (start_time.to(PreciseTime::now()).num_milliseconds() as f64)
                / (duration.num_milliseconds() as f64);

            if portion_elapsed >= 1.0 {
                break;
            }

            let next_candidate = self.tweak_candidate(&annealing_candidate);
            let next_is_better = self.rank_candidate(&next_candidate) > self.rank_candidate(&annealing_candidate);

            // echo "set xrange [0:1]; set yrange [0:1]; set grid;f(x) = exp(-10*(x**3)); plot f(x)" | gnuplot -p
            let replacement_threshold = 1.0f64.exp().powf(-10.0 * portion_elapsed.powf(3.0));

            if next_is_better || (thread_rng().gen_range(0.0, 1.0) < replacement_threshold) {
                annealing_candidate = next_candidate;
            }

            if self.rank_candidate(&annealing_candidate) > self.rank_candidate(&best_candidate) {
                best_candidate = self.clone_candidate(&annealing_candidate);
            }
        }

        best_candidate
    }
}
