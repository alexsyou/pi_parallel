use linya::{Bar, Progress};
use std::time;

use crate::{BAR_MAX, PRECISION};

pub fn sequential_execution_while() -> f64 {
    let now = time::Instant::now();
    let mut i: usize = 1;
    let mut pi: f64 = 0.0;

    // The progress bars allow us to visualize the output.
    // Takes a trivial time, but not zero, so it is optimal to remove if doing performance testing.
    let mut progress = Progress::new();
    let bar: Bar = progress.bar(BAR_MAX, "Finding pi sequentially: ");

    let mut cutoff = PRECISION / BAR_MAX;

    progress.set_and_draw(&bar, 1);

    while i < PRECISION {
        pi += 1.0 / i as f64;

        i += 2;

        pi -= 1.0 / i as f64;

        i += 2;
        if i > cutoff {
            progress.inc_and_draw(&bar, 1);
            cutoff += PRECISION / BAR_MAX;
        }
    }

    println!("The value of pi is {}", pi * 4.0);
    //let elapsed_time = now.elapsed();
    let new_now = time::Instant::now();
    let time = new_now.duration_since(now);
    println!("Took {:?} seconds sequentially", time);
    time.as_secs_f64()
}

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

pub fn sequential_execution_step() -> f64 {
    let now = time::Instant::now();

    let pi: f64 = (1..PRECISION).into_iter().step_by(2).fold(0.0, |acc, i| {
        acc + if ((i - 1) % 4) == 0 {
            1.0 / i as f64
        } else {
            -1.0 / i as f64
        }
    });

    println!("Pi is {}", pi * 4.0);
    let new_now = time::Instant::now();
    let time = new_now.duration_since(now);
    println!("Took {:?} seconds with step", time);
    time.as_secs_f64()
}
