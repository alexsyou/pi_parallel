# Pi_Parallel

A repository demonstrating the parallization of the calculation of pi using Rust. 

## Introduction
The calculation of pi is a popular method to test parallelism. Specifically, the Leibniz formula, given as:
$$ 1 - \frac{1}{3} + \frac{1}{5} - \frac{1}{7} + \frac{1}{9} \dots = \frac{\pi}{4} $$
is embarassingly parallelizable.


This program compares multiple crates for parallel computation in the Rust ecosystem:
* [std::mpsc](https://doc.rust-lang.org/std/sync/mpsc/)
* [rayon](https://docs.rs/rayon/latest/rayon/)
* [flume](https://docs.rs/flume/latest/flume/)
* [crossbeam](https://docs.rs/crossbeam/latest/crossbeam/)


The results of the comparison is generated as a image using the [plotters](https://docs.rs/plotters/0.3.2/plotters/) library, and stored in /images/pi_comparison.png.


## Installation
git clone https://github.com/alexsyou/pi_parallel.git
cd pi_parallel
cargo r --release

## To Run
To ensure the iterator is optimized correctly, run with the release flag (`--release`).
