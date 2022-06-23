use rayon::prelude::*;
use rayon::current_thread_index;
use std::time;
use std::sync::{Mutex};

// Linya Progress bars
use linya::{Bar, Progress};

use crate::BAR_MAX;

pub fn parallel_execution(thread_count: usize, precision: usize) {
    let now = time::Instant::now();

    /* Third Attempt: use par_iter better
     */
    let num_idx: usize = precision / 2 + precision % 2;

    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(thread_count)
        .build()
        .unwrap();

    let pi: f64 = pool.install(|| {
        (0..num_idx)
            .into_par_iter()
            .fold(|| 0.0_f64, |acc: f64, i: usize| 
                acc + if i % 2 == 0 { 1.0/(2*i+1) as f64 } else { -1.0/(2*i+1) as f64 })
            .sum::<f64>()
    });

    println!("The value of pi is {}", pi * 4.0);

    let new_now = time::Instant::now();
    println!(
        "Took {:?} seconds parallelized with {} threads with rayon",
        new_now.duration_since(now),
        thread_count
    );
}

/* Second method of parallel_execution
 * This is slower than the first one, but creates separate threads which can use linya progress
 * bars
 */
pub fn parallel_execution_with_separate_threads(thread_count: usize, precision: usize) {
    let now = time::Instant::now();

    let progress = Mutex::new(Progress::new());

    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(thread_count)
        .build()
        .unwrap();
    
    let pi: Mutex<f64> = Mutex::new(0.0);

    pool.install(|| {
        (0..thread_count).into_par_iter().for_each(|n| {
            let bar: Bar = progress.lock().unwrap().bar(BAR_MAX, format!("Rayon Thread #{}", n));
            progress.lock().unwrap().set_and_draw(&bar, 1);

            let mut idx = n * 2 + 1;
            let mut thread_pi: f64 = 0.0;

            let mut cutoff = precision / BAR_MAX;

            if n % 2 == 0 {
                while idx < precision {
                    thread_pi += 1.0 / idx as f64;

                    idx += 2 * thread_count;

                    if idx > cutoff {
                        cutoff += precision / BAR_MAX;
                        progress.lock().unwrap().inc_and_draw(&bar, 1);
                    }
                }
            } else {
                while idx < precision {
                    thread_pi -= 1.0 / idx as f64;
                    
                    idx += 2 * thread_count;

                    if idx > cutoff {
                        cutoff += precision / BAR_MAX;
                        progress.lock().unwrap().inc_and_draw(&bar, 1);
                    }
                }
            }

            let mut out_pi = pi.lock().unwrap();

            *out_pi += thread_pi;
            println!("{}", *out_pi);
        });
    });

    let final_pi = pi.lock().unwrap();
    println!("The value of pi is {}", *final_pi * 4.0);

    let new_now = time::Instant::now();
    println!(
        "Took {:?} seconds parallelized with {} individual threads with rayon",
        new_now.duration_since(now),
        thread_count
    );
}
