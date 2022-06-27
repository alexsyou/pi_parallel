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

pub const BAR_MAX: usize = 50;
pub const PRECISION: usize = 1 << 34;

fn main() {
    seq::sequential_execution();

    println!();

    seq::sequential_execution_iter();

    println!();

    mpsc::parallel_execution(8);

    println!();

    rayon::parallel_execution(8);

    println!();

    crossbeam::parallel_execution(8);

    println!();

    flume::parallel_execution(8);
}
