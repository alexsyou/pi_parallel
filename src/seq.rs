use std::time;

pub fn sequential_execution() {
    let now = time::Instant::now();
    let mut i: u32 = 1;
    let mut pi: f64 = 0.0;
    while i < u32::MAX/2 {
        pi += 1.0/i as f64;

        i += 2;

        pi -= 1.0/i as f64;

        i += 2;
    }

    println!("The value of pi is {}", pi * 4.0);
    //let elapsed_time = now.elapsed();
    let new_now = time::Instant::now();
    println!("Took {:?} seconds sequentially", new_now.duration_since(now));
}
