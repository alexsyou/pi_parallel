//!
#![warn(missing_debug_implementations, missing_docs)]
#![warn(
    clippy::all,
    clippy::restriction,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo
)]

mod crossbeam;
mod flume;
mod mpsc;
mod rayon;
mod seq;
mod plot;

pub const BAR_MAX: usize = 50;
pub const PRECISION: usize = 1 << 32;

fn main() {
    let sw_time: f64 = seq::sequential_execution_while();

    println!();

    let si_time:f64 = seq::sequential_execution_iter();

    println!();

    let ss_time: f64 = seq::sequential_execution_step();

    println!();

    let mut idx = 2;
    let mut mpsc_vec = Vec::new();
    let mut rayon_vec = Vec::new();
    let mut crossbeam_vec = Vec::new();
    let mut flume_vec = Vec::new();

    while idx <= 64 {
        mpsc_vec.push(mpsc::parallel_execution(idx)); 
        println!();

        rayon_vec.push(rayon::parallel_execution(idx));
        println!();

        crossbeam_vec.push(crossbeam::parallel_execution(idx));
        println!();

        flume_vec.push(flume::parallel_execution(idx));
        println!();

        idx *= 2;
    }
    
    plot::plot(sw_time, si_time, &mpsc_vec, &rayon_vec, &crossbeam_vec, &flume_vec);
}
