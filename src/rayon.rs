use rayon::prelude::*;
use std::time;

pub fn parallel_execution(thread_count: usize, precision: usize) {
    // Sets the number of threads to be the thread_count
    rayon::ThreadPoolBuilder::new().num_threads(thread_count).build_global().unwrap();

    let num_idx = precision/2 + precision%2;
    let idxs: [usize; num_idx] = [0; num_idx];
}
