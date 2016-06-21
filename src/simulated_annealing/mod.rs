use rand::{thread_rng, Rng};
use super::Metaheuristics;
use time::{Duration, PreciseTime};

pub fn solve<T>(problem: &mut Metaheuristics<T>, duration: Duration) -> T {
    let mut best_candidate       = problem.generate_candidate();
    let mut annealing_candidate  = problem.tweak_candidate(&best_candidate);
    let start_time               = PreciseTime::now();
    let duration_in_milliseconds = duration.num_milliseconds() as f64;

    loop {
        let portion_elapsed = (start_time.to(PreciseTime::now()).num_milliseconds() as f64) / duration_in_milliseconds;

        if portion_elapsed >= 1.0 {
            break;
        }

        let next_candidate = problem.tweak_candidate(&annealing_candidate);
        let next_is_better = problem.rank_candidate(&next_candidate) > problem.rank_candidate(&annealing_candidate);

        // echo "set xrange [0:1]; set yrange [0:1]; set grid;f(x) = exp(-10*(x**3)); plot f(x)" | gnuplot -p
        let replacement_threshold = 1.0f64.exp().powf(-10.0 * portion_elapsed.powf(3.0));

        if next_is_better || (thread_rng().gen_range(0.0, 1.0) < replacement_threshold) {
            annealing_candidate = next_candidate;
        }

        if problem.rank_candidate(&annealing_candidate) > problem.rank_candidate(&best_candidate) {
            best_candidate = problem.clone_candidate(&annealing_candidate);
        }
    }

    best_candidate
}
