use rayon::prelude::*;
use std::time;

use crate::PRECISION;

pub fn parallel_execution(thread_count: usize) -> f64 {
    let now = time::Instant::now();

    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(thread_count)
        .build()
        .unwrap();

    let pi: f64 = pool.install(|| {
        (1..PRECISION)
            .into_par_iter()
            .step_by(2)
            .fold(
                || 0.0_f64,
                |acc: f64, i: usize| {
                    acc + if ((i - 1) % 4) == 0 {
                        1.0 / i as f64
                    } else {
                        -1.0 / i as f64
                    }
                },
            )
            .sum::<f64>()
    });

    println!("The value of pi is {}", pi * 4.0);

    let new_now = time::Instant::now();
    let time = new_now.duration_since(now);
    println!(
        "Took {:?} seconds parallelized with {} threads with rayon",
        time, thread_count
    );
    time.as_secs_f64()
}
