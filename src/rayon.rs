use rayon::prelude::*;
use std::time;

use crate::PRECISION;

pub fn parallel_execution(thread_count: usize) -> f64 {
    let now = time::Instant::now();

    /* Third Attempt: use par_iter better
     */
    let num_idx: usize = PRECISION / 2 + PRECISION % 2;

    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(thread_count)
        .build()
        .unwrap();

    let pi: f64 = pool.install(|| {
        (0..num_idx)
            .into_par_iter()
            .fold(
                || 0.0_f64,
                |acc: f64, i: usize| {
                    acc + if i % 2 == 0 {
                        1.0 / (2 * i + 1) as f64
                    } else {
                        -1.0 / (2 * i + 1) as f64
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
        time,
        thread_count
    );
    time.as_secs_f64()
}
