use rayon::prelude::*;
use rayon::current_thread_index;
use std::time;

// Imports for progress bars
use linya::{Progress, Bar};
use std::sync::{Mutex, Arc};

const BAR_MAX: usize = 50;

pub fn parallel_execution(thread_count: usize, precision: usize) {
    // Sets the number of threads to be the thread_count
    rayon::ThreadPoolBuilder::new()
        .num_threads(thread_count)
        .build_global()
        .unwrap();


    // Progress bar
    let progress = Mutex::new(Progress::new());

    /* First Attempt: using rayon par_iter to iterate through a vector of all indexes needed
     * However, we cannot allocate this much memory
        let num_idx: usize = precision / 2 + precision % 2;
        let idxs = vec![0; num_idx];

        let output_tuple: (Vec<usize>, Vec<usize>) = idxs
            .iter()
            .enumerate()
            .map(|(idx, _)| (idx, idx * 2 + 1))
            .unzip();
    */

    /* Third Attempt: use par_iter better
     */
    let num_idx: usize = precision / 2 + precision % 2;

    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(thread_count)
        .build()
        .unwrap();


    let mut bar_vec = Vec::new();
    let progress_vec = Mutex::new(vec![0; thread_count]);
    for i in 0..thread_count {
        bar_vec.push(progress.lock().unwrap().bar(BAR_MAX, format!("Thread #{}", i)));
    }

    let now = time::Instant::now();

    let pi: f64 = pool.install(|| {
        (0..num_idx)
            .into_par_iter()
            .map(|i| {
                // let bar_num = current_thread_index().unwrap();
                // let cur_max = progress_vec.lock().unwrap()[bar_num];
                // if i > cur_max && cur_max < num_idx/thread_count {
                //     progress.lock().unwrap().inc_and_draw(&bar_vec[bar_num], 1);
                //     progress_vec.lock().unwrap()[bar_num] += num_idx/(50*thread_count);
                // }
                if i % 2 == 0 {
                    1.0 / (i * 2 + 1) as f64
                } else {
                    -1.0 / (i * 2 + 1) as f64
                }

            })
            .sum()
    });

    println!("The value of pi is {}", pi * 4.0);

    let new_now = time::Instant::now();
    println!(
        "Took {:?} seconds parallelized with {} threads with rayon",
        new_now.duration_since(now),
        thread_count
    );

    /* Second Attempt: use rayon scope to create multiple threads
    let count_even = thread_count / 2;
    let count_odd = thread_count / 2 + (thread_count % 2);
    rayon::scope(|s| {
        for c_e in 0..count_even {
            s.spawn(|| {
                let mut add_sum = 0.0;
                let mut i = 1 + 4 * c_e;

                while i < precision {
                    add_sum += 1.0 / i as f64;
                    i += count_even * 4;
                }
            })
        }
    })
    */
}
