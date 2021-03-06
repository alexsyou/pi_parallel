use std::thread;
use std::time;

use crate::PRECISION;

/* This function is deprecated
pub fn parallel_execution(thread_count: usize) -> f64 {
    let now = time::Instant::now();

    let (tx, rx) = flume::unbounded();

    let count_even = thread_count / 2;
    let count_odd = thread_count / 2 + (thread_count % 2);

    let arc = Arc::new(Mutex::new(Progress::new()));

    for c_e in 0..count_even {
        let tx = tx.clone();
        let arc = arc.clone();

        thread::spawn(move || {
            let bar = arc
                .lock()
                .unwrap()
                .bar(BAR_MAX, format!("Even thread #{}", c_e));

            arc.lock().unwrap().set_and_draw(&bar, 1);

            let mut add_sum = 0.0;
            let mut i = 1 + 4 * c_e;

            let mut cutoff = PRECISION / BAR_MAX;

            while i < PRECISION {
                add_sum += 1.0 / i as f64;
                i += count_even * 4;

                if i > cutoff {
                    arc.lock().unwrap().inc_and_draw(&bar, 1);
                    cutoff += PRECISION / BAR_MAX;
                }
            }

            tx.send(add_sum).unwrap();
        });
    }

    for c_o in 0..count_odd {
        let tx = tx.clone();
        let arc = arc.clone();

        thread::spawn(move || {
            let bar = arc
                .lock()
                .unwrap()
                .bar(BAR_MAX, format!("Odd thread #{}", c_o));
            arc.lock().unwrap().set_and_draw(&bar, 1);

            let mut sub_sum = 0.0;
            let mut i = 3 + 4 * c_o;

            let mut cutoff = PRECISION / BAR_MAX;

            while i < PRECISION {
                sub_sum -= 1.0 / i as f64;
                i += count_odd * 4;

                if i > cutoff {
                    arc.lock().unwrap().inc_and_draw(&bar, 1);
                    cutoff += PRECISION / BAR_MAX;
                }
            }

            tx.send(sub_sum).unwrap();
        });
    }

    drop(tx);
    drop(arc);

    let pi: f64 = rx.iter().sum();

    println!("The value of pi is {}", pi * 4.0);

    let new_now = time::Instant::now();
    let time = new_now.duration_since(now);
    println!(
        "Took {:?} seconds parallelized with {} threads with flume channels",
        time, thread_count
    );
    time.as_secs_f64()
}
*/

pub fn parallel_execution(thread_count: usize) -> f64 {
    let now = time::Instant::now();

    let (tx, rx) = flume::unbounded();

    for t in 0..thread_count {
        let tx = tx.clone();

        thread::spawn(move || {
            let init_val: usize = t * 2 + 1;
            let local_pi: f64 = (init_val..PRECISION)
                .into_iter()
                .step_by(2 * thread_count)
                .fold(0.0, |acc, i| {
                    acc + if ((i - 1) % 4) == 0 {
                        1.0 / i as f64
                    } else {
                        -1.0 / i as f64
                    }
                });

            tx.send(local_pi).unwrap();
        });
    }

    drop(tx);

    let pi: f64 = rx.iter().sum();

    println!("The value of pi is {}", pi * 4.0);
    let new_now = time::Instant::now();
    let time = new_now.duration_since(now);
    println!(
        "Took {:?} seconds parallelized with {} threads with flume",
        time, thread_count
    );
    time.as_secs_f64()
}
