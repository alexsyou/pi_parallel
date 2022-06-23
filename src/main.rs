//!
#![warn(missing_debug_implementations, missing_docs)]
#![warn(
    clippy::all,
    clippy::restriction,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo
)]

mod mpsc;
mod rayon;
mod seq;
mod crossbeam;

pub const BAR_MAX: usize = 50;

fn main() {
    let precision: usize = 1 << 30;

    seq::sequential_execution(precision);

    println!();

    seq::sequential_execution_iter(precision);

    println!();

    mpsc::parallel_execution(8, precision);

    println!();

    rayon::parallel_execution(8, precision);

    println!();

    crossbeam::parallel_execution(8, precision);
}
