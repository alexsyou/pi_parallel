use std::time;
use linya::{Bar, Progress};

const BAR_MAX: usize = 50;

pub fn sequential_execution(precision: usize) {
    let now = time::Instant::now();
    let mut i: usize = 1;
    let mut pi: f64 = 0.0;

    // The progress bars allow us to visualize the output.
    // Takes a trivial time, but not zero, so it is optimal to remove if doing performance testing.
    let mut progress = Progress::new();
    let bar: Bar = progress.bar(BAR_MAX, "Finding pi sequentially: ");

    let mut cutoff = precision/BAR_MAX;

    progress.set_and_draw(&bar, 1);

    while i < precision {
        pi += 1.0/i as f64;

        i += 2;

        pi -= 1.0/i as f64;

        i += 2;
        if i > cutoff {
            progress.inc_and_draw(&bar, 1);
            cutoff += precision/BAR_MAX;
        }
    }

    println!("The value of pi is {}", pi * 4.0);
    //let elapsed_time = now.elapsed();
    let new_now = time::Instant::now();
    println!("Took {:?} seconds sequentially", new_now.duration_since(now));
}
