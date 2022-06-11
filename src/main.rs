mod mpsc;
mod seq;

fn main() {
    let precision = 1 << 35; 
    seq::sequential_execution(precision);
    println!("");
    mpsc::parallel_execution(64, precision);
}
