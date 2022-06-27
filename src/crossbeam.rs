use crossbeam::channel::unbounded;
use crossbeam::thread;
use std::time;

// Progress bars :)
use linya::Progress;
use std::sync::{Arc, Mutex};

use crate::{PRECISION, BAR_MAX};

pub fn parallel_execution(thread_count: usize) {
    let now = time::Instant::now();

    let (s, r) = unbounded();

    let count_even = thread_count / 2;
    let count_odd = thread_count / 2 + (thread_count % 2);

    let arc = Arc::new(Mutex::new(Progress::new()));

    thread::scope(|sco| {
        for c_e in 0..count_even {
            let s = s.clone();
            let arc = arc.clone();

            sco.spawn(move |_| {
                let bar = arc
                    .lock()
                    .unwrap()
                    .bar(BAR_MAX, format!("Even thread #{}", c_e));
                arc.lock().unwrap().set_and_draw(&bar, 1);

                let mut add_sum: f64 = 0.0;
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

                s.send(add_sum).unwrap();
            });
        }

        for c_o in 0..count_odd {
            let s = s.clone();
            let arc = arc.clone();

            sco.spawn(move |_| {
                let bar = arc
                    .lock()
                    .unwrap()
                    .bar(BAR_MAX, format!("Odd Thread #{}", c_o));
                arc.lock().unwrap().set_and_draw(&bar, 1);

                let mut sub_sum: f64 = 0.0;
                let mut i = 3 + 4 * c_o;

                let mut cutoff = PRECISION / BAR_MAX;

                while i < PRECISION {
                    sub_sum -= 1.0 / i as f64;
                    i += 4 * count_odd;

                    if i > cutoff {
                        arc.lock().unwrap().inc_and_draw(&bar, 1);
                        cutoff += PRECISION / BAR_MAX;
                    }
                }

                s.send(sub_sum).unwrap();
            });
        }
    })
    .unwrap();

    drop(s);

    let pi: f64 = r.iter().sum();

    println!("The value of pi is {}", pi * 4.0);

    let new_now = time::Instant::now();
    println!(
        "Took {:?} seconds parallelized with {} threads with crossbeam channels",
        new_now.duration_since(now),
        thread_count
    );
}
