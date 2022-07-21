use std::time;

use crate::PRECISION;

pub fn sequential_execution_iter() -> f64 {
    let now = time::Instant::now();

    let num_idx = PRECISION / 2 + PRECISION % 2;

    let pi: f64 = (0..num_idx).into_iter().fold(0.0, |acc, i| {
        acc + if i % 2 == 0 {
            1.0 / (2 * i + 1) as f64
        } else {
            -1.0 / (2 * i + 1) as f64
        }
    });

    println!("The value of pi is {}", pi * 4.0);
    //let elapsed_time = now.elapsed();
    let new_now = time::Instant::now();
    let time = new_now.duration_since(now);
    println!("Took {:?} seconds sequentially with iterator", time);
    time.as_secs_f64()
}
