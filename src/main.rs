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

fn main() {
    let precision: usize = 1 << 30;

    seq::sequential_execution(precision);
    seq::sequential_execution_iter(precision);

    println!();

    mpsc::parallel_execution(25, precision);

    println!();

    rayon::parallel_execution(4, precision);
}
