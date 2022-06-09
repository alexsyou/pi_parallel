use std::sync::mpsc;
use std::thread;
use std::time;

pub fn parallel_execution() {
    let (tx, rx) = mpsc::channel();

    let precision: u32 = u32::MAX/2;
    
    let now = time::Instant::now();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let mut i: u32 = 1;
        let mut add_sum = 0.0;
        while i < precision {
            add_sum += 1.0/i as f64;

            i += 4;
            //if i % 1000001 == 0 {
            //    println!("i is {}", i);
            //}
        } 

        tx1.send(add_sum).unwrap();
    });


    thread::spawn(move || {
        let mut i: u32 = 3;
        let mut sub_sum = 0.0;
        while i < precision {
            sub_sum -= 1.0/i as f64;
            i += 4;
            //if i % 1000003 == 0 {
            //    println!("i is {}", i);
            //}
        }
        tx.send(sub_sum).unwrap();
    });


    let mut pi: f64 = 0.0;
    for received in rx {
        pi += received;
    }

    println!("The value of pi is {}", pi * 4.0);
    //let elapsed_time = now.elapsed();
    let new_now = time::Instant::now();
    println!("Took {:?} seconds parallelized with two threads", new_now.duration_since(now));
}
