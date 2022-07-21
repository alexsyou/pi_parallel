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
mod plot;
mod rayon;
mod seq;

pub const BAR_MAX: usize = 50;
pub const PRECISION: usize = 1 << 32;

fn main() {
    println!("Calculating pi sequentially ...");

    let si_time: f64 = seq::sequential_execution_iter();

    println!();

    let mut idx = 2;
    let mut mpsc_vec = Vec::new();
    let mut rayon_vec = Vec::new();
    let mut crossbeam_vec = Vec::new();
    let mut flume_vec = Vec::new();

    while idx <= 64 {
        println!(
            "Calculating pi with {} threads using mpsc channels ...",
            idx
        );
        mpsc_vec.push(mpsc::parallel_execution(idx));
        println!();

        println!(
            "Calculating pi with {} threads using rayon parallel iterators ...",
            idx
        );
        rayon_vec.push(rayon::parallel_execution(idx));
        println!();

        println!(
            "Calcuating pi with {} threads using crossbeam channels ...",
            idx
        );
        crossbeam_vec.push(crossbeam::parallel_execution(idx));
        println!();

        println!(
            "Calculating pi with {} threads using flume channels ...",
            idx
        );
        flume_vec.push(flume::parallel_execution(idx));
        println!();

        idx *= 2;
    }


    println!("Plotting comparison plot at images/pi_comparison.png");
    plot::plot(si_time, &mpsc_vec, &rayon_vec, &crossbeam_vec, &flume_vec);
}
