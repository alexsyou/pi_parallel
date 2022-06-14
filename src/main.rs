//!
#![warn(missing_debug_implementations, missing_docs)]
#![warn(
 clippy::all,
 clippy::restriction,
 clippy::pedantic,
 clippy::nursery,
 clippy::cargo,
)]

mod mpsc;
mod seq;

fn main() {
    let precision = 1 << 33; 

    seq::sequential_execution(precision);

    println!();

    mpsc::parallel_execution(25, precision);
}
