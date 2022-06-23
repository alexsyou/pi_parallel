use std::sync::mpsc;
use std::thread;
use std::time;

// Imports for progress bars
use linya::Progress;
use std::sync::{Arc, Mutex};

use crate::BAR_MAX;

pub fn parallel_execution(thread_count: usize, precision: usize) {
    let (tx, rx) = mpsc::channel();

    let now = time::Instant::now();

    let count_even = thread_count / 2;
    let count_odd = thread_count / 2 + (thread_count % 2);

    // Hold the progress bars
    let arc = Arc::new(Mutex::new(Progress::new()));

    for c_e in 0..count_even {
        let tx = tx.clone();
        let arc = arc.clone();

        thread::spawn(move || {
            // Progress bar in each thread
            let bar = arc
                .lock()
                .unwrap()
                .bar(BAR_MAX, format!("Even thread #{}", c_e));
            arc.lock().unwrap().set_and_draw(&bar, 1);

            let mut add_sum = 0.0;
            let mut i = 1 + 4 * c_e;
            // Progress cutoff
            let mut cutoff = precision / BAR_MAX;

            while i < precision {
                add_sum += 1.0 / i as f64;
                i += count_even * 4;

                // Update progress bar
                if i > cutoff {
                    arc.lock().unwrap().inc_and_draw(&bar, 1);
                    cutoff += precision / BAR_MAX;
                }
            }

            tx.send(add_sum).unwrap();
        });
    }

    for c_o in 0..count_odd {
        let tx = tx.clone();
        let arc = arc.clone();

        thread::spawn(move || {
            // Progress bar in each thread
            let bar = arc
                .lock()
                .unwrap()
                .bar(BAR_MAX, format!("Odd thread #{}", c_o));
            arc.lock().unwrap().set_and_draw(&bar, 1);

            let mut sub_sum = 0.0;
            let mut i = 3 + 4 * c_o;
            // Progress cutoff
            let mut cutoff = precision / BAR_MAX;

            while i < precision {
                sub_sum -= 1.0 / i as f64;
                i += count_odd * 4;

                // Update progress bar
                if i > cutoff {
                    arc.lock().unwrap().inc_and_draw(&bar, 1);
                    cutoff += precision / BAR_MAX;
                }
            }

            tx.send(sub_sum).unwrap();
        });
    }

    // Since we have the originals, we can drop them to proceed
    drop(tx);
    drop(arc);

    let mut pi: f64 = 0.0;
    for received in rx {
        pi += received;
    }

    println!("The value of pi is {}", pi * 4.0);
    //let elapsed_time = now.elapsed();
    let new_now = time::Instant::now();
    println!(
        "Took {:?} seconds parallelized with {} threads with mpsc",
        new_now.duration_since(now),
        thread_count
    );
}
