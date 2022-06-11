use std::sync::mpsc;
use std::thread;
use std::time;

pub fn parallel_execution(thread_count: usize, precision: usize) {
    let (tx, rx) = mpsc::channel();
    
    let now = time::Instant::now();

    let count_even = thread_count/2;
    let count_odd = thread_count/2 + (thread_count%2);

    for c_e in 0..count_even {
        let tx = tx.clone();
        thread::spawn(move || {
            let mut add_sum = 0.0;
            let mut i = 1 + 4 * c_e;
            
            while i < precision {
                add_sum += 1.0/i as f64;
                i += count_even * 4;
            }

            tx.send(add_sum).unwrap();
        });
    }


    for c_o in 0..count_odd {
        let tx = tx.clone();
        thread::spawn(move || {
            let mut sub_sum = 0.0;
            let mut i = 3 + 4 * c_o;
            
            while i < precision {
                sub_sum -= 1.0/i as f64;
                i += count_odd * 4;
            }

            tx.send(sub_sum).unwrap();
        });
    }

    drop(tx);

    let mut pi: f64 = 0.0;
    for received in rx {
        pi += received;
    }

    println!("The value of pi is {}", pi * 4.0);
    //let elapsed_time = now.elapsed();
    let new_now = time::Instant::now();
    println!("Took {:?} seconds parallelized with {} threads", new_now.duration_since(now), thread_count);
}
